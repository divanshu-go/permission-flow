use serde::de::DeserializeOwned;
use tauri::{
    AppHandle, Runtime,
    plugin::{PluginApi, PluginHandle},
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_permission_flow);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PermissionFlowPlugin<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "PermissionFlowPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_permission_flow)?;
    Ok(PermissionFlowPlugin(handle))
}

/// Access to the permission-flow APIs.
pub struct PermissionFlowPlugin<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> PermissionFlowPlugin<R> {
    pub fn start_flow(&self, payload: StartFlowRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("start_flow", payload)
            .map_err(Into::into)
    }

    pub fn stop_current_flow(&self) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("stop_current_flow", ())
            .map_err(Into::into)
    }
}
