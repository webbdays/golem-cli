use std::sync::Arc;
use rust_mcp_sdk::mcp_server::{hyper_server, HyperServerOptions};
use rust_mcp_sdk::schema::{Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools, LATEST_PROTOCOL_VERSION};
use rust_mcp_sdk::TransportOptions;

use crate::command::mcp_server::McpServerSubcommand;
use crate::command_handler::mcp_server::handler::GolemCliMcpServerHandler;
use crate::context::Context;
use crate::log::log_action;

mod handler;
mod tools;

pub struct McpServerCommandHandler {
    _ctx: Arc<Context>,
}

impl McpServerCommandHandler {
    pub fn new(_ctx: Arc<Context>) -> Self {
        Self { _ctx }
    }

    pub async fn handle_command(
        &self,
        subcommand: McpServerSubcommand,
    ) -> anyhow::Result<()> {
        match subcommand {
            McpServerSubcommand::Run { port, timeout} => self.mcp_server_start(port, timeout).await,
        }
    }

    async fn mcp_server_start(&self, port: Option<u16>, timeout: Option<u64>) -> anyhow::Result<()> {
        let port = port.unwrap_or(8080);
        let timeout = timeout.unwrap_or(6);


        let server_details = InitializeResult {
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {list_changed: None}),
                
                ..Default::default()
            },
            instructions: Some("Help user to do what normally they can do using golem cli directly".to_string()),
            meta: None,
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            server_info: Implementation { name: "Golem Cli Mcp Server".to_string(), version: "0.1.0".to_string() },
        };

        let hyper_server_options = HyperServerOptions {
            host: "127.0.0.1".to_string(),
            port,
            transport_options: TransportOptions {
                timeout: std::time::Duration::from_secs(timeout),
                ..Default::default()
            }.into(),
            ..Default::default()
        };
        log_action("Mcp Server Start", &format!("Golem Cli running MCP server at port: {}", port) );

        let server = hyper_server::create_server(server_details, GolemCliMcpServerHandler{}, hyper_server_options);
        match server.start().await {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(anyhow::anyhow!("Failed to start MCP server, {}",e))
                },
        }
    }
}
