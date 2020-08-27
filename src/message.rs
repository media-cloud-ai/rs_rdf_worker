use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::writer::turtle_writer::TurtleWriter;
use reqwest::header::*;
use reqwest::StatusCode;
use std::{thread, time};
use uuid::Uuid;

use crate::namespaces::*;

use crate::resource_model::{ExternalIds, Format, Resource, Resources};
use crate::video_model::metadata::Metadata;
use crate::{Order, RdfWorkerParameters};
use mcai_worker_sdk::job::{JobResult, JobStatus};
use mcai_worker_sdk::{McaiChannel, MessageError, Result};

use futures::executor::block_on;

pub trait ToRdf {
    fn to_rdf(&self, graph: &mut Graph);
}

pub fn process(
    _channel: Option<McaiChannel>,
    parameters: RdfWorkerParameters,
    job_result: JobResult,
) -> Result<JobResult> {
    let config = parameters.get_perfect_memory_config();
    let ntriples = parameters.ntriples.unwrap_or(false);
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
            let paths = input_paths.ok_or_else(|| {
                MessageError::RuntimeError("Missing input_paths parameter".to_string())
            })?;

            let storage = storage.unwrap_or("akamai-video-prod".to_string());

            let url_prefix = url_prefix
                .unwrap_or("http://videos-pmd.francetv.fr/innovation/SubTil/".to_string());

            let mut references = vec![];

            let mut ttml_paths: Vec<Resource> = paths
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
                    storage: storage.clone(),
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
                        video_id: Some(reference.clone()),
                        legacy_id: None,
                        group_id: None,
                        job_id: None,
                        remote_id: None,
                    },
                })
                .collect();

            let mut dash_manifest_paths: Vec<Resource> = paths
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
                    storage: storage.clone(),
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
                        video_id: Some(reference.clone()),
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

            convert_into_rdf(job_result.clone(), &resources, ntriples)?
        }
        Order::PublishMetadata => {
            info!("Get video metadata");
            let mut video_metadata = block_on(get_video_metadata(job_result.clone(), &reference))?;
            info!("Get files");

            let mut si_video_files = block_on(get_files(job_result.clone(), &reference))?;

            for path in input_paths.unwrap_or(vec![]) {
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
                        video_id: Some(reference.clone()),
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
            convert_into_rdf(job_result.clone(), &video_metadata, ntriples)?
        }
    };

    info!("Publish to PerfectMemory");
    info!("rdf_triples: {}", rdf_triples);

    block_on(publish_to_perfect_memory(
        job_result.clone(),
        &config.client_id,
        &pm_event_name,
        &config.api_key,
        &config.endpoint,
        &rdf_triples,
    ))?;
    info!("Completed");
    let job_result = job_result.with_status(JobStatus::Completed);
    Ok(job_result)
}

pub async fn get_video_metadata(job_result: JobResult, reference: &str) -> Result<Metadata> {
    let url =
        "https://gatewayvf.webservices.francetelevisions.fr/v1/videos/".to_owned() + reference;

    let client = reqwest::Client::builder().build().unwrap();

    let response = client
        .get(url.as_str())
        .send()
        .await
        .map_err(|e| MessageError::ProcessingError(job_result.clone().with_error(e)))?;

    let status = response.status();

    if !(status == StatusCode::OK) {
        error!("{:?}", response);
        return Err(MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_message("bad response status"),
        ));
    }

    response.json().await.map_err(|e| {
        MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_message(&e.to_string()),
        )
    })
}

pub async fn get_files(job_result: JobResult, reference: &str) -> Result<Vec<Resource>> {
    let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/files?external_ids.video_id="
        .to_owned()
        + reference;

    let client = reqwest::Client::builder().build().unwrap();

    let response = client.get(url.as_str()).send().await.map_err(|e| {
        MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_error(e),
        )
    })?;

    let status = response.status();

    if !(status == StatusCode::OK) {
        error!("{:?}", response);
        return Err(MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_message("bad response status"),
        ));
    }

    response.json().await.map_err(|e| {
        MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_message(&e.to_string()),
        )
    })
}

pub fn convert_into_rdf<T: ToRdf>(
    job_result: JobResult,
    item: &T,
    ntriples: bool,
) -> Result<String> {
    let mut graph = Graph::new(None);
    graph.add_namespace(&Namespace::new(
        "rdf".to_string(),
        Uri::new(RDF_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "rdfs".to_string(),
        Uri::new(RDFS_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "owl".to_string(),
        Uri::new(OWL_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "ebucore".to_string(),
        Uri::new(EBUCORE_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "francetv".to_string(),
        Uri::new(FRANCETV_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "xsi".to_string(),
        Uri::new(XSI_NAMESPACE.to_owned()),
    ));
    graph.add_namespace(&Namespace::new(
        "default".to_string(),
        Uri::new(DEFAULT_NAMESPACE.to_owned()),
    ));

    item.to_rdf(&mut graph);
    if ntriples {
        let writer = NTriplesWriter::new();
        writer.write_to_string(&graph).map_err(|e| {
            MessageError::ProcessingError(
                job_result
                    .with_status(JobStatus::Error)
                    .with_message(&e.to_string()),
            )
        })
    } else {
        let writer = TurtleWriter::new(graph.namespaces());
        writer.write_to_string(&graph).map_err(|e| {
            MessageError::ProcessingError(
                job_result
                    .with_status(JobStatus::Error)
                    .with_message(&e.to_string()),
            )
        })
    }
}

#[derive(Serialize)]
struct InfosGraph {
    value: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Serialize)]
struct Inputs {
    infos_graph: InfosGraph,
}

#[derive(Serialize)]
struct PmRequestBody {
    client_id: String,
    name: String,
    inputs: Inputs,
}

#[derive(Debug, Deserialize)]
struct PmResponseBody {
    id: String,
    chain_id: String,
    process_id: String,
    status: u32,
    updated_at: String,
}

pub async fn publish_to_perfect_memory(
    job_result: JobResult,
    pm_client_id: &str,
    pm_event_name: &str,
    pm_api_key: &str,
    pm_endpoint: &str,
    triples: &str,
) -> Result<()> {
    let url = pm_endpoint.to_owned() + "/v1/requests";

    let client = reqwest::Client::builder().build().unwrap();

    let body = PmRequestBody {
        client_id: pm_client_id.to_owned(),
        name: pm_event_name.to_owned(),
        inputs: Inputs {
            infos_graph: InfosGraph {
                value: base64::encode(triples),
                kind: "binary".to_string(),
            },
        },
    };

    let response = client
        .post(url.as_str())
        .header(CACHE_CONTROL, "no-cache")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", pm_api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            MessageError::ProcessingError(
                job_result
                    .clone()
                    .with_status(JobStatus::Error)
                    .with_message(&e.to_string()),
            )
        })?;

    if response.status() != 201 {
        let text = response
            .text()
            .await
            .unwrap_or("unknown reason.".to_owned());
        error!("Unable to push to Perfect Memory: {}", text);
        return Err(MessageError::ProcessingError(
            job_result
                .clone()
                .with_status(JobStatus::Error)
                .with_message(&format!("Unable to push into Perfect Memory: {}", text)),
        ));
    }

    if let Some(location) = response.headers().get(LOCATION) {
        loop {
            let response = client
                .get(location.to_str().unwrap())
                .header("X-Api-Key", pm_api_key)
                .send()
                .await
                .map_err(|e| {
                    MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message(&e.to_string()),
                    )
                })?;

            if response.status() != 200 {
                let ten_seconds = time::Duration::from_secs(10);
                thread::sleep(ten_seconds);
                continue;
            }

            let resp_body: PmResponseBody = response.json().await.map_err(|error| {
                MessageError::ProcessingError(
                    job_result
                        .clone()
                        .with_status(JobStatus::Error)
                        .with_message(&format!(
                        "Unknown error: unable to get status from Perfect Memory platform: {:?}",
                        error
                    )),
                )
            })?;
            error!("Perfect Memory response: {:?}", resp_body);
            match resp_body.status {
                200 | 300 => {
                    return Ok(());
                }
                100 | 110 | 120 => {}
                400 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Error: Request/Process has finished with an error"),
                    ));
                }
                401 => {
                    return Err(MessageError::ProcessingError(job_result.clone().with_status(JobStatus::Error).with_message("Error on child process: Process has finished with an error on one of its children")));
                }
                408 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message(
                                "Error Service: Process has finished with a specific error",
                            ),
                    ));
                }
                410 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Item Disabled: The item is disabled"),
                    ));
                }
                414 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Item Not Found: The item is not found"),
                    ));
                }
                421 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message(
                                "Invalid Script: There was an error while running the script",
                            ),
                    ));
                }
                422 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Invalid I/O: The input or the output is invalid"),
                    ));
                }
                423 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message(
                            "Invalid Status: The process has been stopped with an invalid status",
                        ),
                    ));
                }
                428 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Process disrupted: Process has been manually disrupted"),
                    ));
                }
                500 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message(
                            "Unexpected error: Service process has finished with an unknow error",
                        ),
                    ));
                }
                503 => {
                    return Err(MessageError::ProcessingError(
                        job_result
                            .clone()
                            .with_status(JobStatus::Error)
                            .with_message("Service unreachable: Service could not be reached"),
                    ));
                }
                _ => {}
            }

            let ten_seconds = time::Duration::from_secs(10);
            thread::sleep(ten_seconds);
        }
    } else {
        return Err(MessageError::ProcessingError(
            job_result
                .with_status(JobStatus::Error)
                .with_message(&format!("Unable get location to wait end of ingest")),
        ));
    }
}

#[test]
fn test_mapping_video() {
    use crate::video_model::metadata::Metadata;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

    let mut video_struct = String::new();
    let mut video_file = File::open("tests/video.json").unwrap();
    let _ = video_file.read_to_string(&mut video_struct).unwrap();

    let mut sifiles_struct = String::new();
    let mut sifiles_file = File::open("tests/files.json").unwrap();
    let _ = sifiles_file.read_to_string(&mut sifiles_struct).unwrap();

    let mut video_metadata: Metadata = serde_json::from_str(&video_struct).unwrap();
    let ftv_resources: Vec<Resource> = serde_json::from_str(&sifiles_struct).unwrap();
    video_metadata.resources = Resources {
        items: ftv_resources,
    };

    let rdf_triples = convert_into_rdf(JobResult::new(666), &video_metadata, true).unwrap();

    let mut ntriple_struct = String::new();
    let mut ntriple_file = File::open("tests/triples.nt").unwrap();
    let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
    println!("{}", rdf_triples);
    assert_eq!(rdf_triples, ntriple_struct);
}

#[test]
fn test_mapping_resource() {
    use crate::resource_model::Resource;
    use std::fs::File;
    use std::io::Read;

    let resource = Resource {
        id: "000000-1111-2222-3333-44444444".to_string(),
        created_via: "Media-IO".to_string(),
        format: Format {
            id: "playlist-hls".to_string(),
            label: "playlist/hls".to_string(),
            kind: "playlist".to_string(),
            mime_type: "urn:mimetype:application/dash+xml".to_string(),
        },
        storage: "akamai-video-prod".to_string(),
        path: None,
        filename: Some("/path/to/manifest.mpd".to_string()),
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
        url: Some(format!(
            "{}{}",
            "http://videos-pmd.francetv.fr/innovation/SubTil/", "/path/to/manifest.mpd"
        )),
        version: None,
        lang: None,
        external_ids: ExternalIds {
            video_id: Some("44444444-3333-2222-1111-000000".to_string()),
            legacy_id: None,
            group_id: None,
            job_id: None,
            remote_id: None,
        },
    };

    let rdf_triples = convert_into_rdf(JobResult::new(666), &resource, true).unwrap();

    let mut ntriple_struct = String::new();
    let mut ntriple_file = File::open("tests/triples_resource.nt").unwrap();
    let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
    println!("{}", rdf_triples);
    assert!(rdf_triples == ntriple_struct);
}

// #[test]
// fn test_publish() {
//   use std::fs::File;
//   use std::io::Read;

//   let mut ntriple_struct = String::new();
//   let mut ntriple_file = File::open("tests/triples.nt").unwrap();
//   let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();

//   let pm_client_id = "5ab4ca78dd37d3000c64912e";
//   let pm_api_key = "mxzzM934dGxxojcjNYxi";
//   let pm_endpoint = "https://exchange-manager-api.platform.labs.pm";
//   publish_to_perfect_memory(666, pm_client_id, pm_api_key, pm_endpoint, &ntriple_struct).unwrap();
// }
