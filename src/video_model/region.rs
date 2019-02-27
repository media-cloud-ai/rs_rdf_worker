#[derive(Debug, Deserialize)]
pub struct Region {
    id: String,
    label: String,
    code: Option<String>,
    timezone: Option<String>,
}
