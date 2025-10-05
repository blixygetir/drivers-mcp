use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};

use rust_mcp_sdk::macros::JsonSchema;

use std::process::Command;

#[mcp_tool(name = "update_nvidia_drivers", description = "Updates the nvida gpu drivers to the latest version")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct UpdateNvidiaDrivers {}

impl UpdateNvidiaDrivers {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let output = Command::new("sudo")
            .arg("ubuntu-drivers")
            .arg("autoinstall")
            .output();

        let message = match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.trim().is_empty() {
                    "Your drivers are up to date.".to_string()
                } else {
                    format!("nvidia drivers updated successfully:\n{}", stdout)
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Failed to update:\n{}", stderr)
            }
            Err(e) => format!("Failed to run updates.\nError: {}", e),
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(message)]))
    }
}

#[mcp_tool(name = "fetch_nvidia_driver_version", description = "Fetches the installed nvida driver version")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct FetchNvidiaDriverVersion {}

impl FetchNvidiaDriverVersion {
  pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
      let output = Command::new("nvidia-smi").output();

        let message = match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find(|line| line.contains("Driver Version"))
                    .unwrap_or("Driver version not found")
                    .to_string()
            }
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(e) => format!("Failed to fetch nvidia driver version.\nError: {}", e),
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(message)]))
  }
}

// tool_box!(NvidiaDriverTools, [UpdateNvidiaDrivers, FetchNvidiaDriverVersion]);
