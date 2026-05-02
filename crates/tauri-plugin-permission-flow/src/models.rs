use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    Accessibility,
    InputMonitoring,
    ScreenRecording,
    AppManagement,
    Bluetooth,
    DeveloperTools,
    FullDiskAccess,
    MediaAppleMusic,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartFlowRequest {
    pub permission: Permission,
    pub app_path: String,
    #[serde(default = "default_use_click_source_frame")]
    pub use_click_source_frame: bool,
}

const fn default_use_click_source_frame() -> bool {
    true
}
