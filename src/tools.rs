use rust_mcp_sdk::tool_box;

use crate::nvidia_tools::{UpdateNvidiaDrivers, FetchNvidiaDriverVersion};
use crate::module_tools::{ListModules, ListModuleDependencies};
use crate::intel_tools::{UpdateIntelDrivers, FetchIntelDriverVersion};
use crate::driver_tools::{ListDrivers, ListDeviceDrivers};

tool_box!(DriverManagerTools, [UpdateNvidiaDrivers, FetchNvidiaDriverVersion, ListModules, ListModuleDependencies, UpdateIntelDrivers, FetchIntelDriverVersion, ListDrivers, ListDeviceDrivers]);

