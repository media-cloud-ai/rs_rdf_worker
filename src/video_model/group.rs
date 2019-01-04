
#[derive(Debug, Deserialize)]
pub struct Group {
  pub id: String,
  created_at: Option<String>,
  updated_at: Option<String>,
  pub label: String,
  pub season_number: Option<u32>,
  pub number_of_episodes: Option<u32>,
  pub description: Option<String>,
  created_by: Option<String>,
  created_via: Option<String>,
}
