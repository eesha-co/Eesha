use std::path::PathBuf;

use dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

// Note: the reason why we didn't send `IpcSender` in those messages is because it panics on MacOS,
// see https://github.com/eesha-browser/eesha/pull/222#discussion_r1939111585,
// the work around is let eesha send back the message through the initial sender and we map them back manually

// Can't use `PipelineId` directly or else we need to pull in servo as a dependency
type SerializedPipelineId = Vec<u8>;

/// Message sent from the controller to eesha
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ToEeshaMessage {
    /// Initial configs for eesha
    /// this will be the first message sent to Eesha once we received the sender from [`ToControllerMessage::SetToEeshaSender`]
    SetConfig(ConfigFromController),
    /// Exit
    Exit,
    /// Register a listener on eesha for getting notified on close requested from the OS,
    /// eesha will send a [`ToControllerMessage::OnCloseRequested`] when that happens
    ListenToOnCloseRequested,
    /// Navigate to this URL
    NavigateTo(url::Url),
    /// Reload the current webview
    Reload,
    /// Register a listener on eesha for getting notified on navigation starting,
    /// eesha will send a [`ToControllerMessage::OnNavigationStarting`] when that happens
    ListenToOnNavigationStarting,
    /// Response to a [`ToControllerMessage::OnNavigationStarting`] message from eesha
    OnNavigationStartingResponse(SerializedPipelineId, bool),
    /// Execute JavaScript
    ExecuteScript(String),
    /// Register a listener on eesha for getting notified on web resource requests
    ListenToWebResourceRequests,
    /// Response to a [`ToControllerMessage::OnWebResourceRequested`] message from eesha
    WebResourceRequestResponse(WebResourceRequestResponse),
    /// Sets the webview window's size
    SetSize(Size),
    /// Sets the webview window's position
    SetPosition(Position),
    /// Maximize or unmaximize the window
    SetMaximized(bool),
    /// Minimize or unminimize the window
    SetMinimized(bool),
    /// Sets the window to fullscreen or back
    SetFullscreen(bool),
    /// Show or hide the window
    SetVisible(bool),
    /// Moves the window with the left mouse button until the button is released
    StartDragging,
    /// Bring the window to the front, and capture input focus
    Focus,
    /// Get the window's size, need a response with [`ToControllerMessage::GetSizeResponse`]
    GetSize(uuid::Uuid, SizeType),
    /// Get the window's position, need a response with [`ToControllerMessage::GetPositionResponse`]
    GetPosition(uuid::Uuid, PositionType),
    /// Get if the window is currently maximized or not, need a response with [`ToControllerMessage::GetMaximizedResponse`]
    GetMaximized(uuid::Uuid),
    /// Get if the window is currently minimized or not, need a response with [`ToControllerMessage::GetMinimizedResponse`]
    GetMinimized(uuid::Uuid),
    /// Get if the window is currently fullscreen or not, need a response with [`ToControllerMessage::GetFullscreenResponse`]
    GetFullscreen(uuid::Uuid),
    /// Get the visibility of the window, need a response with [`ToControllerMessage::GetVisibleResponse`]
    GetVisible(uuid::Uuid),
    /// Get the scale factor of the window, need a response with [`ToControllerMessage::GetScaleFactorResponse`]
    GetScaleFactor(uuid::Uuid),
    /// Get the current URL of the webview, need a response with [`ToControllerMessage::GetCurrentUrlResponse`]
    GetCurrentUrl(uuid::Uuid),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionType {
    Inner,
    Outer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SizeType {
    Inner,
    Outer,
}

/// Message sent from eesha to the controller
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ToControllerMessage {
    /// IPC sender for the controller to send commands to eesha,
    /// this will be the first message sent to the controller once connected
    SetToEeshaSender(ipc::IpcSender<ToEeshaMessage>),
    /// Sent on a new navigation starting, need a response with [`ToEeshaMessage::OnNavigationStartingResponse`]
    OnNavigationStarting(SerializedPipelineId, url::Url),
    /// Sent on a new web resource request, need a response with [`ToEeshaMessage::WebResourceRequestResponse`]
    OnWebResourceRequested(WebResourceRequest),
    /// Response to a [`ToEeshaMessage::GetSize`]
    GetSizeResponse(uuid::Uuid, PhysicalSize<u32>),
    /// Response to a [`ToEeshaMessage::GetPosition`]
    GetPositionResponse(uuid::Uuid, Option<PhysicalPosition<i32>>),
    /// Response to a [`ToEeshaMessage::GetMaximized`]
    GetMaximizedResponse(uuid::Uuid, bool),
    /// Response to a [`ToEeshaMessage::GetMinimized`]
    GetMinimizedResponse(uuid::Uuid, bool),
    /// Response to a [`ToEeshaMessage::GetFullscreen`]
    GetFullscreenResponse(uuid::Uuid, bool),
    /// Response to a [`ToEeshaMessage::GetVisible`]
    GetVisibleResponse(uuid::Uuid, bool),
    /// Response to a [`ToEeshaMessage::GetScaleFactor`]
    GetScaleFactorResponse(uuid::Uuid, f64),
    /// Response to a [`ToEeshaMessage::GetCurrentUrl`]
    GetCurrentUrlResponse(uuid::Uuid, url::Url),
    /// Eesha have recieved a close request from the OS
    OnCloseRequested,
}

/// Configuration of Eesha instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigFromController {
    /// URL to load initially.
    pub url: Option<url::Url>,
    /// Should launch without or without control panel
    pub with_panel: bool,
    /// Window size for the initial winit window
    pub inner_size: Option<Size>,
    /// Window position for the initial winit window
    pub position: Option<Position>,
    /// Launch maximized or not for the initial winit window
    pub maximized: bool,
    /// Launch visible or not for the initial winit window
    pub visible: bool,
    /// Launch fullscreen or not for the initial winit window
    pub fullscreen: bool,
    /// Launch focused or not for the initial winit window
    pub focused: bool,
    /// Launch decorated or not for the initial winit window
    pub decorated: bool,
    /// Launch transparent or not for the initial winit window
    pub transparent: bool,
    /// Title of the initial winit window in the title bar.
    pub title: Option<String>,
    /// Window icon of the initial winit window.
    pub icon: Option<Icon>,
    /// Port number to start a server to listen to remote Firefox devtools connections. 0 for random port.
    pub devtools_port: Option<u16>,
    /// Servo time profile settings
    pub profiler_settings: Option<ProfilerSettings>,
    /// Override the user agent
    pub user_agent: Option<String>,
    /// Script to run on document started to load
    pub user_scripts: Vec<UserScript>,
    /// Initial window's zoom level
    pub zoom_level: Option<f32>,
    /// Path to resource directory. If None, Eesha will try to get default directory. And if that
    /// still doesn't exist, all resource configuration will set to default values.
    pub resources_directory: Option<PathBuf>,
}

impl Default for ConfigFromController {
    fn default() -> Self {
        Self {
            url: None,
            with_panel: false,
            inner_size: None,
            position: None,
            maximized: false,
            visible: true,
            focused: true,
            decorated: false,
            transparent: false,
            title: None,
            icon: None,
            fullscreen: false,
            devtools_port: None,
            profiler_settings: None,
            user_agent: None,
            user_scripts: Vec::new(),
            zoom_level: None,
            resources_directory: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Icon {
    /// RGBA bytes of the icon.
    pub rgba: Vec<u8>,
    /// Icon width.
    pub width: u32,
    /// Icon height.
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserScript {
    pub script: String,
    pub source_file: Option<PathBuf>,
}

impl<T: Into<String>> From<T> for UserScript {
    fn from(script: T) -> Self {
        UserScript {
            script: script.into(),
            source_file: None,
        }
    }
}

/// Servo time profile settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProfilerSettings {
    /// Servo time profile settings
    pub output_options: OutputOptions,
    /// When servo profiler is enabled, this is an optional path to dump a self-contained HTML file
    /// visualizing the traces as a timeline.
    pub trace_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OutputOptions {
    /// Database connection config (hostname, name, user, pass)
    FileName(String),
    Stdout(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResourceRequest {
    pub id: uuid::Uuid,
    #[serde(with = "http_serde_ext::request")]
    pub request: http::Request<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResourceRequestResponse {
    pub id: uuid::Uuid,
    #[serde(with = "http_serde_ext::response::option")]
    pub response: Option<http::Response<Vec<u8>>>,
}
