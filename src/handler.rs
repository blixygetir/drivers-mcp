use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};
use std::sync::Arc;

use crate::tools::DriverManagerTools;

pub struct MyServerHandler;

#[async_trait]
#[allow(unused)]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(&self, request: ListToolsRequest, runtime: Arc<dyn McpServer>) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: DriverManagerTools::tools(),
        })
    }

    async fn handle_call_tool_request(&self, request: CallToolRequest, runtime: Arc<dyn McpServer>) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: DriverTools = DriverTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            DriverManagerTools::ListDrivers(list_drivers_tool) => list_drivers_tool.call_tool(),
            DriverManagerTools::ListDeviceDrivers(list_device_drivers_tool) => list_device_drivers_tool.call_tool(),
            DriverManagerTools::ListModules(list_modules_tool) => list_modules_tool.call_tool(),
            DriverManagerTools::ListModuleDependencies(list_modules_dependencies_tool) => list_modules_dependencies_tool.call_tool(),
            DriverManagerTools::UpdateIntelDrivers(update_intel_drivers_tool) => update_intel_drivers_tool.call_tool(),
            DriverManagerTools::FetchIntelDriverVersion(fetch_intel_drivers_version_tool) => fetch_intel_drivers_version_tool.call_tool(),
            DriverManagerTools::UpdateNvidiaDrivers(update_nvidia_drivers_tool) => update_nvidia_drivers_tool.call_tool(),
            DriverManagerTools::FetchNvidiaDriverVersion(fetch_nvidia_drivers_version_tool) => fetch_nvidia_drivers_version_tool.call_tool(),
        }
    }
}
