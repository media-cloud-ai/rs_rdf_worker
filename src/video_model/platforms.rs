use crate::video_model::platform::Platform;

#[derive(Debug, Deserialize)]
pub struct Platforms {
    ftv: Option<Platform>,
    free: Option<Platform>,
    orange: Option<Platform>,
    bouygues: Option<Platform>,
    sfr: Option<Platform>,
    canalsat: Option<Platform>,
    numericable: Option<Platform>,
    molotov: Option<Platform>,
}
