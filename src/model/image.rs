
use model::format::Format;

#[derive(Debug, Deserialize)]
pub struct Image {
  pub id: String,
  pub created_at: Option<String>,
  created_via: String,
  pub updated_at: Option<String>,
  pub format: Format,
  storage: String,
  path: Option<String>,
  filename: Option<String>,
  pub filesize_bytes: Option<u32>,
  pub md5_checksum: Option<String>,
  copyright: Option<String>,
  #[serde(default)]
  tags: Vec<String>,
  ratio: Option<String>,
  pub width: u32,
  pub height: u32,
  index: Option<u32>,
  pub url: String,
}
