
#[derive(Debug, Deserialize)]
pub struct Tag {
  pub id: String,
  pub label: String,
  description: Option<String>,
}
