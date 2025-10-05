use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::{macros::mcp_tool, tool_box};

use rust_mcp_sdk::macros::JsonSchema;

use std::process::Command;

#[mcp_tool(name = "get_driver_health", description = "Retuns the kenel-releated driver errors")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct GetDriverHealth {}

impl GetDriverHealth {
  let result = Command::new("sudo").arg("dmesg").arg("-l").arg("err,crit,alert,emerg").arg("|").arg("grep").arg("-i").arg("driver").output();
  
