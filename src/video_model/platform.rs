use crate::video_model::window::Window;

#[derive(Debug, Deserialize)]
pub struct Platform {
    status: Option<String>,
    #[serde(default)]
    exploitation_windows: Option<Vec<Window>>,
    remote_id: Option<String>,
}
