#[derive(Debug, Deserialize)]
pub struct Part {
  title: Option<String>,
  start: Option<String>,
  duration: Option<String>,
  index: Option<u32>,
  tags: Option<Vec<String>>,
}
