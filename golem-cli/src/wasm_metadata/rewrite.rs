use crate::wasm_metadata::add_metadata::AddMetadataField;
use crate::wasm_metadata::{AddMetadata, ComponentNames, ModuleNames, Producers};
use anyhow::Result;
use std::mem;
use wasm_encoder::ComponentSection as _;
use wasm_encoder::{ComponentSectionId, Encode, Section};
use wasmparser::{KnownCustom, Parser, Payload::*};

pub(crate) fn rewrite_wasm(
    metadata: &AddMetadata,
    add_producers: &Producers,
    input: &[u8],
) -> Result<Vec<u8>> {
    let mut producers_found = false;
    let mut names_found = false;
    let mut stack = Vec::new();
    let mut output = Vec::new();
    for payload in Parser::new(0).parse_all(input) {
        let payload = payload?;

        // Track nesting depth, so that we don't mess with inner producer sections:
        match payload {
            Version { encoding, .. } => {
                output.extend_from_slice(match encoding {
                    wasmparser::Encoding::Component => &wasm_encoder::Component::HEADER,
                    wasmparser::Encoding::Module => &wasm_encoder::Module::HEADER,
                });
            }
            ModuleSection { .. } | ComponentSection { .. } => {
                stack.push(mem::take(&mut output));
                continue;
            }
            End { .. } => {
                let mut parent = match stack.pop() {
                    Some(c) => c,
                    None => break,
                };
                if output.starts_with(&wasm_encoder::Component::HEADER) {
                    parent.push(ComponentSectionId::Component as u8);
                    output.encode(&mut parent);
                } else {
                    parent.push(ComponentSectionId::CoreModule as u8);
                    output.encode(&mut parent);
                }
                output = parent;
            }
            _ => {}
        }

        // Only rewrite the outermost custom sections
        if let CustomSection(c) = &payload {
            if stack.is_empty() {
                match c.as_known() {
                    KnownCustom::Producers(_) => {
                        producers_found = true;
                        let mut producers = Producers::from_bytes(c.data(), c.data_offset())?;
                        // Add to the section according to the command line flags:
                        producers.merge(add_producers);
                        // Encode into output:
                        producers.section().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::Name(_) => {
                        names_found = true;
                        let mut names = ModuleNames::from_bytes(c.data(), c.data_offset())?;
                        let name = match &metadata.name {
                            AddMetadataField::Set(name) => Some(name.clone()),
                            AddMetadataField::Keep => None,
                            AddMetadataField::Clear => None,
                        };
                        names.merge(&ModuleNames::from_name(&name));

                        names.section()?.as_custom().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::ComponentName(_) => {
                        names_found = true;
                        let mut names = ComponentNames::from_bytes(c.data(), c.data_offset())?;
                        let name = match &metadata.name {
                            AddMetadataField::Set(name) => Some(name.clone()),
                            AddMetadataField::Keep => None,
                            AddMetadataField::Clear => None,
                        };
                        names.merge(&ComponentNames::from_name(&name));
                        names.section()?.as_custom().append_to(&mut output);
                        continue;
                    }
                    KnownCustom::Unknown if c.name() == "author" => {
                        if metadata.authors.is_keep() {
                            let author = crate::wasm_metadata::Authors::parse_custom_section(c)?;
                            author.append_to(&mut output);
                            continue;
                        } else if metadata.authors.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "description" => {
                        if metadata.description.is_keep() {
                            let description =
                                crate::wasm_metadata::Description::parse_custom_section(c)?;
                            description.append_to(&mut output);
                            continue;
                        } else if metadata.description.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "licenses" => {
                        if metadata.licenses.is_keep() {
                            let licenses = crate::wasm_metadata::Licenses::parse_custom_section(c)?;
                            licenses.append_to(&mut output);
                            continue;
                        } else if metadata.licenses.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "source" => {
                        if metadata.source.is_keep() {
                            let source = crate::wasm_metadata::Source::parse_custom_section(c)?;
                            source.append_to(&mut output);
                            continue;
                        } else if metadata.source.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "homepage" => {
                        if metadata.source.is_keep() {
                            let homepage = crate::wasm_metadata::Homepage::parse_custom_section(c)?;
                            homepage.append_to(&mut output);
                            continue;
                        } else if metadata.source.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "revision" => {
                        if metadata.source.is_keep() {
                            let revision = crate::wasm_metadata::Revision::parse_custom_section(c)?;
                            revision.append_to(&mut output);
                            continue;
                        } else if metadata.source.is_clear() {
                            continue;
                        }
                    }
                    KnownCustom::Unknown if c.name() == "version" => {
                        if metadata.version.is_keep() {
                            let version = crate::wasm_metadata::Version::parse_custom_section(c)?;
                            version.append_to(&mut output);
                            continue;
                        } else if metadata.version.is_clear() {
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        // All other sections get passed through unmodified:
        if let Some((id, range)) = payload.as_section() {
            wasm_encoder::RawSection {
                id,
                data: &input[range],
            }
            .append_to(&mut output);
        }
    }
    if !names_found {
        if let AddMetadataField::Set(name) = &metadata.name {
            if output.starts_with(&wasm_encoder::Component::HEADER) {
                let names = ComponentNames::from_name(&Some(name.clone()));
                names.section()?.append_to_component(&mut output);
            } else {
                let names = ModuleNames::from_name(&Some(name.clone()));
                names.section()?.append_to(&mut output)
            }
        }
    }
    if !producers_found && !add_producers.is_empty() {
        let mut producers = Producers::empty();
        // Add to the section according to the command line flags:
        producers.merge(add_producers);
        // Encode into output:
        producers.section().append_to(&mut output);
    }
    if let AddMetadataField::Set(author) = &metadata.authors {
        author.append_to(&mut output);
    }
    if let AddMetadataField::Set(description) = &metadata.description {
        description.append_to(&mut output);
    }
    if let AddMetadataField::Set(licenses) = &metadata.licenses {
        licenses.append_to(&mut output);
    }
    if let AddMetadataField::Set(source) = &metadata.source {
        source.append_to(&mut output);
    }
    if let AddMetadataField::Set(homepage) = &metadata.homepage {
        homepage.append_to(&mut output);
    }
    if let AddMetadataField::Set(revision) = &metadata.revision {
        revision.append_to(&mut output);
    }
    if let AddMetadataField::Set(version) = &metadata.version {
        version.append_to(&mut output);
    }
    Ok(output)
}
