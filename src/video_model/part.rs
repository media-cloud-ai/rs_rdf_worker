#[derive(Debug, Deserialize)]
pub struct Part {
    title: Option<String>,
    start: Option<String>,
    duration: Option<String>,
    index: u32,
    #[serde(default)]
    tags: Vec<String>,
}
