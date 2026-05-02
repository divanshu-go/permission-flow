use tauri::{
    Manager, Runtime,
    plugin::{Builder, TauriPlugin},
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::PermissionFlowPlugin;
#[cfg(mobile)]
use mobile::PermissionFlowPlugin;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the plugin APIs.
pub trait PermissionFlowExt<R: Runtime> {
    fn permission_flow(&self) -> &PermissionFlowPlugin<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PermissionFlowExt<R> for T {
    fn permission_flow(&self) -> &PermissionFlowPlugin<R> {
        self.state::<PermissionFlowPlugin<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("permission-flow")
        .invoke_handler(tauri::generate_handler![
            commands::start_flow,
            commands::stop_current_flow
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let permission_flow = mobile::init(app, api)?;
            #[cfg(desktop)]
            let permission_flow = desktop::init(app, api)?;
            app.manage(permission_flow);
            Ok(())
        })
        .build()
}
