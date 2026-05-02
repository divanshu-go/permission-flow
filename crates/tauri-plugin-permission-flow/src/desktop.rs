use std::{
    cell::RefCell,
    ffi::CString,
    sync::mpsc::sync_channel,
    thread::{self, ThreadId},
};

use serde::de::DeserializeOwned;
use tauri::{AppHandle, Runtime, plugin::PluginApi};

use crate::Error;
use crate::models::*;

#[cfg(target_os = "macos")]
use permission_flow::{Permission as NativePermission, PermissionFlowController, StartFlowOptions};

#[cfg(target_os = "macos")]
thread_local! {
    static CONTROLLER: RefCell<Option<PermissionFlowController>> = const { RefCell::new(None) };
}

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PermissionFlowPlugin<R>> {
    Ok(PermissionFlowPlugin {
        app: app.clone(),
        main_thread_id: thread::current().id(),
    })
}

/// Access to the permission-flow APIs.
pub struct PermissionFlowPlugin<R: Runtime> {
    app: AppHandle<R>,
    main_thread_id: ThreadId,
}

impl<R: Runtime> PermissionFlowPlugin<R> {
    pub fn start_flow(&self, payload: StartFlowRequest) -> crate::Result<()> {
        #[cfg(target_os = "macos")]
        {
            self.run_on_main_thread(move || {
                CONTROLLER.with(|controller_cell| {
                    let mut controller_slot = controller_cell.borrow_mut();
                    if controller_slot.is_none() {
                        *controller_slot = Some(PermissionFlowController::new()?);
                    }

                    let app_path = CString::new(payload.app_path)?;
                    let mut options =
                        StartFlowOptions::new(permission_to_native(payload.permission), app_path);
                    if !payload.use_click_source_frame {
                        options = options.without_click_source_frame();
                    }

                    controller_slot
                        .as_ref()
                        .expect("controller should exist after initialization")
                        .start_flow(options)?;

                    Ok(())
                })
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = payload;
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn stop_current_flow(&self) -> crate::Result<()> {
        #[cfg(target_os = "macos")]
        {
            self.run_on_main_thread(|| {
                CONTROLLER.with(|controller_cell| {
                    let controller_slot = controller_cell.borrow();
                    if let Some(controller) = controller_slot.as_ref() {
                        controller.stop_current_flow()?;
                    }

                    Ok(())
                })
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    fn run_on_main_thread<T, F>(&self, action: F) -> crate::Result<T>
    where
        T: Send + 'static,
        F: FnOnce() -> crate::Result<T> + Send + 'static,
    {
        if thread::current().id() == self.main_thread_id {
            return action();
        }

        let (sender, receiver) = sync_channel(1);
        self.app.run_on_main_thread(move || {
            let _ = sender.send(action());
        })?;

        receiver
            .recv()
            .map_err(|_| Error::MainThreadChannelClosed)?
    }
}

#[cfg(target_os = "macos")]
fn permission_to_native(permission: Permission) -> NativePermission {
    match permission {
        Permission::Accessibility => NativePermission::ACCESSIBILITY,
        Permission::InputMonitoring => NativePermission::INPUT_MONITORING,
        Permission::ScreenRecording => NativePermission::SCREEN_RECORDING,
        Permission::AppManagement => NativePermission::APP_MANAGEMENT,
        Permission::Bluetooth => NativePermission::BLUETOOTH,
        Permission::DeveloperTools => NativePermission::DEVELOPER_TOOLS,
        Permission::FullDiskAccess => NativePermission::FULL_DISK_ACCESS,
        Permission::MediaAppleMusic => NativePermission::MEDIA_APPLE_MUSIC,
    }
}
