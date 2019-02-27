use video_model::role::Role;

#[derive(Debug, Deserialize)]
pub struct People {
    pub first_name: Option<String>,
    pub last_name: String,
    pub role: Role,
    pub character: Option<String>,
}
