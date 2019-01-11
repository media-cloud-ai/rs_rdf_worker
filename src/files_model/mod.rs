
#[derive(Debug, Deserialize)]
pub struct FtvSiFile {
  pub bitrate_kbps: Option<u64>,
  #[serde(default)]
  pub video_tracks: Vec<VideoTrack>,
  #[serde(default)]
  pub audio_tracks: Vec<AudioTrack>,
  #[serde(default)]
  pub text_tracks: Vec<TextTrack>,
  pub id: String,
  pub format: Format,
  pub storage: String,
  pub path: Option<String>,
  pub filename: Option<String>,
  pub md5_checksum: Option<String>,
  pub filesize_bytes: Option<u64>,
  pub external_ids: ExternalIds,
  #[serde(default)]
  pub tags: Vec<String>,
  pub created_via: String,
  pub version: Option<String>,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
  pub url: Option<String>,
  pub lang: Option<String>,
  pub ratio: Option<String>,
  pub width: Option<u16>,
  pub height: Option<u16>,
  pub index: Option<u64>,
  pub copyright: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VideoTrack {
  pub codec_rfc6381: String,
  pub bitrate_kbps: u64,
  pub width: u16,
  pub height: u16,
  pub frame_rate_fps: u8,
}

#[derive(Debug, Deserialize)]
pub struct AudioTrack {
  pub codec_rfc6381: String,
  pub bitrate_kbps: u64,
  pub sample_rate_hz: u16,
  pub lang: String,
}

#[derive(Debug, Deserialize)]
pub struct TextTrack {
}

#[derive(Debug, Deserialize)]
pub struct Format {
  pub id: String,
  pub label: String,
  #[serde(rename="type")]
  pub kind: String,
  pub mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalIds {
  pub video_id: Option<String>,
  pub legacy_id: Option<String>,
  pub group_id: Option<String>,
  pub job_id: Option<String>,
  pub remote_id: Option<String>,
}
