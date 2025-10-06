use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};

use rust_mcp_sdk::macros::JsonSchema;

use std::process::Command;

#[mcp_tool(name = "update_intel_drivers", description = "Updates the i915 drivers to the latest version")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct UpdateIntelDrivers {}

impl UpdateIntelDrivers {
  pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
    let output = Command::new("sudo")
      .arg("apt")
      .arg("install")
      .arg("--reinstall")
      .arg("xserver-xorg-video-intel")
      .arg("mesa-utils")
      .output();

      let message = match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.trim().is_empty() {
                    "Your drivers are up to date.".to_string()
                } else {
                    format!("i915 drivers updated successfully:\n{}", stdout)
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

#[mcp_tool(name = "fetch_intel_driver_version", description = "Fetches the installed i915 driver version")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct FetchIntelDriverVersion {}

impl FetchIntelDriverVersion {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let output = Command::new("modinfo")
          .arg("i915")
          .output();

          let message = match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find(|line| line.contains("version:"))
                    .unwrap_or("Driver version not found")
                    .to_string()
            }
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(e) => format!("Failed to fetch i915 driver version.\nError: {}", e),
        };

      Ok(CallToolResult::text_content(vec![TextContent::from(message)]))

   }
}

// tool_box!(IntelDriverTools, [UpdateIntelDrivers, FetchIntelDriverVersion]);
