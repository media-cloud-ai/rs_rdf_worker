use reqwest::StatusCode;
use uuid::Uuid;

use crate::resource_model::{ExternalIds, Format, Resource, Resources};
use crate::video_model::metadata::Metadata;
use crate::{Order, RdfWorkerParameters};
use mcai_worker_sdk::job::{JobResult, JobStatus};
use mcai_worker_sdk::{McaiChannel, MessageError, Result};

use crate::convert::convert_into_rdf;
use crate::perfect_memory::{publish_to_perfect_memory, PmConfig};
use reqwest::blocking::Client;

pub fn process(
  _channel: Option<McaiChannel>,
  parameters: RdfWorkerParameters,
  job_result: JobResult,
) -> Result<JobResult> {
  let config = PmConfig::from(parameters.clone());

  let n_triples = parameters.ntriples.unwrap_or(false);
  let pm_event_name = parameters
    .perfect_memory_event_name
    .unwrap_or("push_rdf_infos".to_string());
  let reference = parameters.reference;
  let url_prefix = parameters.url_prefix;
  let storage = parameters.storage;
  let input_paths = parameters.input_paths;
  let order = parameters.order;

  let rdf_triples = match order.unwrap_or_default() {
    Order::PublishDashAndTtml => {
      let input_paths = input_paths
        .ok_or_else(|| MessageError::RuntimeError("Missing input_paths parameter".to_string()))?;

      let storage = storage.unwrap_or("akamai-video-prod".to_string());

      let url_prefix =
        url_prefix.unwrap_or("http://videos-pmd.francetv.fr/innovation/SubTil/".to_string());

      get_dash_and_ttml_rdf(
        job_result.clone(),
        input_paths,
        &reference,
        &storage,
        &url_prefix,
        n_triples,
      )?
    }
    Order::PublishMetadata => {
      let input_paths = input_paths.unwrap_or(vec![]);
      get_metadata_rdf(job_result.clone(), input_paths, &reference, n_triples)?
    }
  };

  info!("Publish to PerfectMemory");
  info!("rdf_triples: {}", rdf_triples);

  publish_to_perfect_memory(
    job_result.clone(),
    &config.client_id,
    &pm_event_name,
    &config.api_key,
    &config.endpoint,
    &rdf_triples,
  )?;
  info!("Completed");
  let job_result = job_result.with_status(JobStatus::Completed);
  Ok(job_result)
}

fn get_dash_and_ttml_rdf(
  job_result: JobResult,
  input_paths: Vec<String>,
  reference: &str,
  storage: &str,
  url_prefix: &str,
  n_triples: bool,
) -> Result<String> {
  let mut references = vec![];

  let mut ttml_paths: Vec<Resource> = input_paths
    .iter()
    .filter(|path| path.ends_with(".ttml"))
    .map(|path| Resource {
      id: Uuid::new_v4().to_urn().to_string(),
      created_via: "Media-IO".to_string(),
      format: Format {
        id: "caption-ttml".to_string(),
        label: "caption/ttml".to_string(),
        kind: "caption".to_string(),
        mime_type: "urn:mimetype:application/xml+ttml".to_string(),
      },
      storage: storage.to_string(),
      path: None,
      filename: Some(path.to_string()),
      audio_tracks: vec![],
      text_tracks: vec![],
      video_tracks: vec![],
      created_at: None,
      updated_at: None,
      ratio: None,
      width: None,
      height: None,
      index: None,
      copyright: None,
      filesize_bytes: None,
      bitrate_kbps: None,
      md5_checksum: None,
      tags: vec![],
      url: Some(format!("{}{}", url_prefix, path)),
      version: None,
      lang: None,
      external_ids: ExternalIds {
        video_id: Some(reference.to_string()),
        legacy_id: None,
        group_id: None,
        job_id: None,
        remote_id: None,
      },
    })
    .collect();

  let mut dash_manifest_paths: Vec<Resource> = input_paths
    .iter()
    .filter(|path| path.ends_with(".mpd"))
    .map(|path| Resource {
      id: Uuid::new_v4().to_urn().to_string(),
      created_via: "Media-IO".to_string(),
      format: Format {
        id: "playlist-dash".to_string(),
        label: "playlist/dash".to_string(),
        kind: "playlist".to_string(),
        mime_type: "urn:mimetype:application/dash+xml".to_string(),
      },
      storage: storage.to_string(),
      path: None,
      filename: Some(path.to_string()),
      audio_tracks: vec![],
      text_tracks: vec![],
      video_tracks: vec![],
      created_at: None,
      updated_at: None,
      ratio: None,
      width: None,
      height: None,
      index: None,
      copyright: None,
      filesize_bytes: None,
      bitrate_kbps: None,
      md5_checksum: None,
      tags: vec![],
      url: Some(format!("{}{}", url_prefix, path)),
      version: None,
      lang: None,
      external_ids: ExternalIds {
        video_id: Some(reference.to_string()),
        legacy_id: None,
        group_id: None,
        job_id: None,
        remote_id: None,
      },
    })
    .collect();

  references.append(&mut ttml_paths);
  references.append(&mut dash_manifest_paths);

  let resources = Resources { items: references };

  convert_into_rdf(&resources, n_triples).map_err(|rdf_error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&rdf_error.to_string()),
    )
  })
}

fn get_metadata_rdf(
  job_result: JobResult,
  input_paths: Vec<String>,
  reference: &str,
  n_triples: bool,
) -> Result<String> {
  info!("Get video metadata");
  let mut video_metadata = get_video_metadata(reference).map_err(|error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&error),
    )
  })?;
  info!("Get files");

  let mut si_video_files = get_files(reference).map_err(|error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&error),
    )
  })?;

  for path in input_paths {
    let format = if path.ends_with(".ttml") {
      Format {
        id: "caption-ttml".to_string(),
        label: "caption/ttml".to_string(),
        kind: "caption".to_string(),
        mime_type: "urn:mimetype:application/xml+ttml".to_string(),
      }
    } else {
      Format {
        id: "video-mp4".to_string(),
        label: "video/mp4".to_string(),
        kind: "video".to_string(),
        mime_type: "urn:mimetype:video/mp4".to_string(),
      }
    };

    let url_prefix = "https://ftv.video.media-io.com/";

    let mut tags = vec!["lts".to_string()];

    if path.ends_with("-qaa.mp4") {
      tags.push("qaa".to_string())
    }
    if path.ends_with("-qad.mp4") {
      tags.push("qad".to_string())
    }

    si_video_files.push(Resource {
      id: Uuid::new_v4().to_urn().to_string(),
      created_via: "Media-IO".to_string(),
      format,
      storage: "ftv.video.media-io.com".to_string(),
      path: None,
      filename: Some(path.to_owned()),
      audio_tracks: vec![],
      text_tracks: vec![],
      video_tracks: vec![],
      created_at: None,
      updated_at: None,
      ratio: None,
      width: None,
      height: None,
      index: None,
      copyright: None,
      filesize_bytes: None,
      bitrate_kbps: None,
      md5_checksum: None,
      tags,
      url: Some(format!("{}{}", url_prefix, path)),
      version: None,
      lang: None,
      external_ids: ExternalIds {
        video_id: Some(reference.to_string()),
        legacy_id: None,
        group_id: None,
        job_id: None,
        remote_id: None,
      },
    });
  }

  video_metadata.resources = Resources {
    items: si_video_files,
  };
  info!("Convert");
  convert_into_rdf(&video_metadata, n_triples).map_err(|rdf_error| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&rdf_error.to_string()),
    )
  })
}

pub fn get_video_metadata(reference: &str) -> std::result::Result<Metadata, String> {
  let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/videos/".to_owned() + reference;

  let client = Client::builder().build().unwrap();

  let response = client.get(url.as_str()).send().map_err(|e| e.to_string())?;

  let status = response.status();

  if !(status == StatusCode::OK) {
    error!("{:?}", response);
    return Err(format!("Bad response status: {:?}", status));
  }

  response.json().map_err(|e| e.to_string())
}

pub fn get_files(reference: &str) -> std::result::Result<Vec<Resource>, String> {
  let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/files?external_ids.video_id="
    .to_owned()
    + reference;

  let client = Client::builder().build().unwrap();

  let response = client.get(url.as_str()).send().map_err(|e| e.to_string())?;

  let status = response.status();

  if !(status == StatusCode::OK) {
    error!("{:?}", response);
    return Err(format!("Bad response status: {:?}", status));
  }

  response.json().map_err(|e| e.to_string())
}
