use tauri::{AppHandle, Runtime, command};

use crate::PermissionFlowExt;
use crate::Result;
use crate::models::*;

#[command]
pub(crate) async fn start_flow<R: Runtime>(
    app: AppHandle<R>,
    payload: StartFlowRequest,
) -> Result<()> {
    app.permission_flow().start_flow(payload)
}

#[command]
pub(crate) async fn stop_current_flow<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.permission_flow().stop_current_flow()
}
