use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};

use rust_mcp_sdk::macros::JsonSchema;

use std::process::Command;

#[mcp_tool(name = "list_drivers", description = "Lists the drivers")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ListDrivers {}

impl ListDrivers {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let result = std::process::Command::new("lspci")
            .arg("-k") // long format
            .output();

        let message = match result {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(e) => format!("Failed to run command: {}", e),
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(
            message,
        )]))
    }
}

#[mcp_tool(
    name = "list_device_drivers",
    description = "Lists the drivers corresponding to the device"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ListDeviceDrivers {
    device_name: String,
}

impl ListDeviceDrivers {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("lspci -k | grep -iA5 '{}'", self.device_name))
            .output();

        let message = match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(e) => format!("Failed to run lspci: {e}"),
        };
        
        Ok(CallToolResult::text_content(vec![TextContent::from(
            message,
        )]))
    }
}

// tool_box!(DriverTools, [ListDrivers, ListDeviceDrivers]);
