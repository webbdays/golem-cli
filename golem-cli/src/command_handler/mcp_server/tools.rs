use crate::command::shared_args::{
    BuildArgs, ComponentOptionalComponentNames, ForceBuildArg, UpdateOrRedeployArgs,
};
use crate::log::{pop_mcp_log_lines, Output};
use crate::model::ComponentName;
use crate::{command::GolemCliGlobalFlags, command_handler::component::ComponentCommandHandler};
use golem_templates::model::PackageName;
use rust_mcp_sdk::macros::JsonSchema;
use rust_mcp_sdk::schema::CallToolResultContentItem;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[mcp_tool(
    name = "create_new_component",
    description = "Create a new component with template name and component package name (e.g. [golem-cli:component_name] or [golem-cli:component_name:version]"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateNewComponentTool {
    template: String,
    component_package_name: String,
}

impl CreateNewComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {

        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                
                let command_new = ComponentCommandHandler::new(ctx.into());
                match command_new
                    .cmd_new(
                        Some(self.template),
                        PackageName::from_string(self.component_package_name),
                    )
                    .await
                {
                    Ok(_) => Ok(CallToolResult {
                        content: pop_mcp_log_lines()
                            .into_iter()
                            .map(|text| CallToolResultContentItem::text_content(text, None))
                            .collect(),
                        meta: None,
                        is_error: None,
                    }),
                    Err(e) => Err(CallToolError(e.into())),
                }
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

#[mcp_tool(
    name = "update_component",
    description = "Update component with full template name and full component package name (e.g. [golem-cli:component_name] or [golem-cli:component_name:version]"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateComponentTool {
    template: String,
    component_package_name: String,
}

impl UpdateComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {
        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                let command_new = ComponentCommandHandler::new(ctx.into());
                match command_new
                    .cmd_new(
                        Some(self.template),
                        PackageName::from_string(self.component_package_name),
                    )
                    .await
                {
                    Ok(_) => Ok(CallToolResult {
                        content: pop_mcp_log_lines()
                            .into_iter()
                            .map(|text| CallToolResultContentItem::text_content(text, None))
                            .collect(),
                        meta: None,
                        is_error: None,
                    }),
                    Err(e) => Err(CallToolError(e.into())),
                }
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

#[mcp_tool(
    name = "build_component",
    description = "Build component with full component names (e.g. [golem-cli:component_name] or [golem-cli:component_name:version])"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BuildComponentTool {
    component_names: Vec<String>,
}

impl BuildComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {
        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                let command_new = ComponentCommandHandler::new(ctx.into());
                match command_new
                    .cmd_build(
                        ComponentOptionalComponentNames {
                            component_name: self
                                .component_names
                                .iter()
                                .map(|name| ComponentName(name.to_string()))
                                .collect(),
                        },
                        BuildArgs {
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(_) => Ok(CallToolResult {
                        content: pop_mcp_log_lines()
                            .into_iter()
                            .map(|text| CallToolResultContentItem::text_content(text, None))
                            .collect(),
                        meta: None,
                        is_error: None,
                    }),
                    Err(e) => Err(CallToolError(e.into())),
                }
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

#[mcp_tool(
    name = "filter_templates",
    description = "Filter templates by filter string (e.g. 'golem' to get all templates containing 'rust' in their name or description'"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FilterTemplatesComponentTool {
    filter: Option<String>,
}

impl FilterTemplatesComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {
        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                let command_new = ComponentCommandHandler::new(ctx.into());

                command_new.cmd_templates(self.filter);
                Ok(CallToolResult {
                    content: pop_mcp_log_lines()
                            .into_iter()
                            .map(|text| CallToolResultContentItem::text_content(text, None))
                            .collect(),
                    meta: None,
                    is_error: None,
                })
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

#[mcp_tool(
    name = "deploy_components",
    description = "Deploy components with their full component names (e.g. 'golem:example-component:0.1.0' or 'golem:example-component' if you want to deploy the latest version of the component'"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeployComponentTool {
    component_names: Vec<String>,
}

impl DeployComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {
        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                let command_new = ComponentCommandHandler::new(ctx.into());

                match command_new
                    .cmd_deploy(
                        ComponentOptionalComponentNames {
                            component_name: self
                                .component_names
                                .iter()
                                .map(|name| ComponentName(name.to_string()))
                                .collect(),
                        },
                        ForceBuildArg {
                            ..Default::default()
                        },
                        UpdateOrRedeployArgs {
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(_) => Ok(CallToolResult {
                        content: vec![],
                        meta: None,
                        is_error: None,
                    }),
                    Err(e) => Err(CallToolError(e.into())),
                }
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

#[mcp_tool(
    name = "list_components",
    description = "List components with their full component names (e.g. 'golem:example-component:0.1.0' or 'golem:example-component' if you want to deploy the latest version of the component'"
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListComponentTool {
    component_name: Option<String>,
}

impl ListComponentTool {
    pub async fn call_tool(self) -> Result<CallToolResult, CallToolError> {
        let start_local_server_yes = Arc::new(tokio::sync::RwLock::new(false));

        match crate::context::Context::new(
            GolemCliGlobalFlags::default(),
            Some(Output::Mcp),
            start_local_server_yes,
            Box::new(|| Box::pin(async { Ok(()) })), // dummy, not starting anything for now
        )
        .await
        {
            Ok(ctx) => {
                let command_new = ComponentCommandHandler::new(ctx.into());

                let component_name = self
                    .component_name
                    .as_ref()
                    .map(|name| ComponentName(name.to_string()));
                match command_new.cmd_list(component_name).await {
                    Ok(_) => Ok(CallToolResult {
                        content: pop_mcp_log_lines()
                            .into_iter()
                            .map(|text| CallToolResultContentItem::text_content(text, None))
                            .collect(),
                        meta: None,
                        is_error: None,
                    }),
                    Err(e) => Err(CallToolError(e.into())),
                }
            }
            Err(e) => Err(CallToolError(e.into())),
        }
    }
}

tool_box!(
    GolemCliTools,
    [
        CreateNewComponentTool,
        UpdateComponentTool,
        BuildComponentTool,
        FilterTemplatesComponentTool,
        DeployComponentTool,
        ListComponentTool
    ]
);
