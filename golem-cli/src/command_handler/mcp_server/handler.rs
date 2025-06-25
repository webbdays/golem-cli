use async_trait::async_trait;
use rust_mcp_sdk::{
    mcp_server::ServerHandler,
    schema::{
        schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
        ListToolsResult, RpcError,
    },
    McpServer,
};

use crate::command_handler::mcp_server::tools::GolemCliTools;
pub struct GolemCliMcpServerHandler;

#[async_trait]
impl ServerHandler for GolemCliMcpServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: GolemCliTools::tools(),
            meta: None,
            next_cursor: None,
        })
    }
    
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {

        
        let tool = GolemCliTools::try_from(request.params)?;


        match tool {
            GolemCliTools::CreateNewComponentTool(create_new_component_tool) => {
                create_new_component_tool.call_tool().await
            }
            GolemCliTools::UpdateComponentTool(update_component_tool) => {
                update_component_tool.call_tool().await
            }
            // GolemCliTools::BuildComponentTool(build_component_tool) => {
            //     build_component_tool.call_tool().await
            // }
            GolemCliTools::FilterTemplatesComponentTool(filter_templates_component_tool) => {
                filter_templates_component_tool.call_tool().await
            }
            // GolemCliTools::DeployComponentTool(deploy_components_tool) => {
            //     deploy_components_tool.call_tool().await
            // }
            GolemCliTools::ListComponentTool(list_components_tool) => {
                list_components_tool.call_tool().await
            }
            _ => {
                Err(CallToolError("Tool not implemented".to_string().into()))
            },
        }
    
    }
}
