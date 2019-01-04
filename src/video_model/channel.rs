
use video_model::region::Region;

#[derive(Debug, Deserialize)]
pub struct Channel {
  pub id: String,
  pub label: String,
  region: Option<Region>,
  #[serde(default)]
  tags: Vec<String>,
}
