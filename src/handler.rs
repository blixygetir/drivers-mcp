use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};
use std::sync::Arc;

use crate::tools::DriverTools;

pub struct MyServerHandler;

#[async_trait]
#[allow(unused)]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(&self, request: ListToolsRequest, runtime: Arc<dyn McpServer>) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: DriverTools::tools(),
        })
    }

    async fn handle_call_tool_request(&self, request: CallToolRequest, runtime: Arc<dyn McpServer>) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: DriverTools = DriverTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            DriverTools::ListDrivers(list_drivers_tool) => list_drivers_tool.call_tool(),
            DriverTools::ListDeviceDrivers(list_device_drivers_tool) => list_device_drivers_tool.call_tool(),
        }
    }
}