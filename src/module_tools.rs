use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};

use rust_mcp_sdk::macros::JsonSchema;

#[mcp_tool(name = "list_modules", description = "Lists all the modules")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ListModules {}

impl ListModules {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let result = std::process::Command::new("lsmod").output();

        let message = match result {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(e) => format!("Failed to list modules.\nError: {} ", e),
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(
            message,
        )]))
    }
}

#[mcp_tool(name = "list_modules", description = "Lists all the modules")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ListModuleDependcies {
  module_name: String
}

impl ListModuleDependencies {
  pub fn call_tool(&self) -> Resut<CallToolResult, CallToolError> {
    let command = format!(
      "lsmod | awk '$1==\"{}\" {{print \"Module: \"$1 \"\\nUsage Count: \"$3 \"\\nDependent Modules: \"$4}}'",
        module_name
      );

      let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    let message = match result {
      Ok(output) if output.status.success() -> {
          String::from_utf8_lossy(&output.stdout).to_string()
      }
      Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
      Err(e) => format!("Failed to list module dependencies.\nError: {} ", e),
    };

    Ok(CallToolResult::text_content(vec![TextContent::from(
            message,
        )]))
      }
}
// tool_box!(ModuleTools, [ListModules, ListModuleDependcies]);
