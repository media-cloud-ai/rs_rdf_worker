#[derive(Debug, Deserialize)]
pub struct Format {
    id: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    pub mime_type: String,
}
