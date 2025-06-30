use rmcp::{
    model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool_handler, ServerHandler,
};

use crate::command_handler::mcp_server::GolemCliMcpServer;

#[tool_handler]
impl ServerHandler for GolemCliMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "Help user to do what normally they can do using golem cli".to_string(),
            ),
            protocol_version: ProtocolVersion::V_2025_03_26,
            server_info: Implementation {
                name: "Golem Cli Mcp Server".to_string(),
                version: "0.1.0".to_string(),
            },
        }
    }
}
