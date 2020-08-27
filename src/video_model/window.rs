#[derive(Debug, Deserialize)]
pub struct Window {
  #[serde(rename = "type")]
  kind: Option<String>,
  start: Option<String>,
  end: Option<String>,
  price_sd: Option<String>,
  price_hd: Option<String>,
}
