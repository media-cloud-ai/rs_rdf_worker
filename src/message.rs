use amqp_worker::job::Job;
use amqp_worker::*;
use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::writer::turtle_writer::TurtleWriter;
use reqwest;
use reqwest::header::*;
use reqwest::StatusCode;
use std::{thread, time};
use uuid::Uuid;

use namespaces::*;

use resource_model::{ExternalIds, Format, Resource, Resources};
use video_model::metadata::Metadata;

pub trait ToRdf {
    fn to_rdf(&self, graph: &mut Graph);
}

// #[derive(Debug, Deserialize)]
// struct SessionResponseBody {
//     access_token: String,
// }

// #[derive(Debug, Deserialize)]
// struct DataResponseBody {
//     id: u32,
//     key: String,
//     value: String,
//     inserted_at: String,
// }

// #[derive(Debug, Deserialize)]
// struct ValueResponseBody {
//     data: DataResponseBody,
// }

#[derive(Debug)]
struct PmConfig {
    endpoint: String,
    client_id: String,
    api_key: String,
}

fn get_perfect_memory_config(job: &Job) -> Result<PmConfig, MessageError> {
    let endpoint = job
        .get_credential_parameter("perfect_memory_endpoint")
        .map(|credential| credential.request_value(job));
    let client_id = job
        .get_credential_parameter("perfect_memory_username")
        .map(|credential| credential.request_value(job));
    let api_key = job
        .get_credential_parameter("perfect_memory_password")
        .map(|credential| credential.request_value(job));

    if endpoint.is_none() {
      return Err(MessageError::ProcessingError(
            job.job_id,
            "Missing perfect_memory_endpoint parameter".to_string(),
        ));
    }
    if client_id.is_none() {
      return Err(MessageError::ProcessingError(
            job.job_id,
            "Missing perfect_memory_username parameter".to_string(),
        ));
    }
    if api_key.is_none() {
      return Err(MessageError::ProcessingError(
            job.job_id,
            "Missing perfect_memory_password parameter".to_string(),
        ));
    }

    Ok(PmConfig {
        endpoint: endpoint.unwrap()?,
        client_id: client_id.unwrap()?,
        api_key: api_key.unwrap()?,
    })
}

pub fn process(message: &str) -> Result<u64, MessageError> {
    let job = Job::new(message)?;

    warn!("{:?}", job);
    let ntriples = job.get_boolean_parameter("ntriples").unwrap_or(false);
    let pm_event_name = job
        .get_string_parameter("perfect_memory_event_name")
        .unwrap_or("push_rdf_infos".to_string());
    let reference = job.get_string_parameter("reference");
    if reference.is_none() {
        return Err(MessageError::ProcessingError(
            job.job_id,
            "Missing reference parameter".to_string(),
        ));
    }
    let reference = reference.unwrap();

    let rdf_triples = match job
        .get_string_parameter("order")
        .unwrap_or("publish_metadata".to_string())
        .as_str()
    {
        "publish_dash_and_ttml" => {
            let paths = job.get_paths_parameter("input_paths");
            if paths.is_none() {
                return Err(MessageError::ProcessingError(
                    job.job_id,
                    "Missing input_paths parameter".to_string(),
                ));
            }
            let paths = paths.unwrap();

            let storage = job
                .get_string_parameter("storage")
                .unwrap_or("akamai-video-prod".to_string());
            let url_prefix = job
                .get_string_parameter("url_prefix")
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

            convert_into_rdf(job.job_id.clone(), &resources, ntriples)?
        }
        "publish_metadata" => {
            info!("Get video metadata");
            let mut video_metadata = get_video_metadata(job.job_id.clone(), &reference)?;
            info!("Get files");

            let mut si_video_files = get_files(job.job_id.clone(), &reference)?;

            for path in job.get_paths_parameter("input_paths").unwrap_or(vec![]) {
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
            convert_into_rdf(job.job_id.clone(), &video_metadata, ntriples)?
        }
        _ => {
            error!("Unimplement job order: {:?}", job);
            return Err(MessageError::ProcessingError(
                job.job_id,
                "Unimplemented".to_string(),
            ));
        }
    };

    info!("Publish to PerfectMemory");
    info!("{}", rdf_triples);
    let config = get_perfect_memory_config(&job)?;

    publish_to_perfect_memory(
        job.job_id.clone(),
        &config.client_id,
        &pm_event_name,
        &config.api_key,
        &config.endpoint,
        &rdf_triples,
    )?;
    info!("Completed");
    Ok(job.job_id)
}

pub fn get_video_metadata(job_id: u64, reference: &str) -> Result<Metadata, MessageError> {
    let url =
        "https://gatewayvf.webservices.francetelevisions.fr/v1/videos/".to_owned() + reference;

    let client = reqwest::Client::builder().build().unwrap();

    let mut response = client
        .get(url.as_str())
        .send()
        .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

    let status = response.status();

    if !(status == StatusCode::OK) {
        error!("{:?}", response);
        return Err(MessageError::ProcessingError(
            job_id,
            "bad response status".to_string(),
        ));
    }

    response
        .json()
        .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
}

pub fn get_files(job_id: u64, reference: &str) -> Result<Vec<Resource>, MessageError> {
    let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/files?external_ids.video_id="
        .to_owned()
        + reference;

    let client = reqwest::Client::builder().build().unwrap();

    let mut response = client
        .get(url.as_str())
        .send()
        .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

    let status = response.status();

    if !(status == StatusCode::OK) {
        error!("{:?}", response);
        return Err(MessageError::ProcessingError(
            job_id,
            "bad response status".to_string(),
        ));
    }

    response
        .json()
        .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
}

pub fn convert_into_rdf<T: ToRdf>(
    job_id: u64,
    item: &T,
    ntriples: bool,
) -> Result<String, MessageError> {
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
        writer
            .write_to_string(&graph)
            .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
    } else {
        let writer = TurtleWriter::new(graph.namespaces());
        writer
            .write_to_string(&graph)
            .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
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

pub fn publish_to_perfect_memory(
    job_id: u64,
    pm_client_id: &str,
    pm_event_name: &str,
    pm_api_key: &str,
    pm_endpoint: &str,
    triples: &str,
) -> Result<(), MessageError> {
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

    let mut response = client
        .post(url.as_str())
        .header(CACHE_CONTROL, "no-cache")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", pm_api_key)
        .json(&body)
        .send()
        .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

    if response.status() != 201 {
        let text = response.text().unwrap_or("unknown reason.".to_owned());
        error!("unable to push to Perfect Memory: {:?}\n{}", response, text);
        return Err(MessageError::ProcessingError(
            job_id,
            format!("Unable to push into Perfect Memory: {}", text),
        ));
    }

    if let Some(location) = response.headers().get(LOCATION) {
        loop {
            let mut response = client
                .get(location.to_str().unwrap())
                .header("X-Api-Key", pm_api_key)
                .send()
                .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

            if response.status() != 200 {
                let ten_seconds = time::Duration::from_secs(10);
                thread::sleep(ten_seconds);
                continue;
            }

            let r: Result<PmResponseBody, _> = response.json();
            error!("Perfect Memory response: {:?}", r);
            if let Ok(resp_body) = r {
                match resp_body.status {
                    200 | 300 => {
                        return Ok(());
                    }
                    100 | 110 | 120 => {}
                    400 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Error: Request/Process has finished with an error".to_owned(),
                        ));
                    }
                    401 => {
                        return Err(MessageError::ProcessingError(job_id, "Error on child process: Process has finished with an error on one of its children".to_owned()));
                    }
                    408 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Error Service: Process has finished with a specific error".to_owned(),
                        ));
                    }
                    410 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Item Disabled: The item is disabled".to_owned(),
                        ));
                    }
                    414 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Item Not Found: The item is not found".to_owned(),
                        ));
                    }
                    421 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Invalid Script: There was an error while running the script"
                                .to_owned(),
                        ));
                    }
                    422 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Invalid I/O: The input or the output is invalid".to_owned(),
                        ));
                    }
                    423 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Invalid Status: The process has been stopped with an invalid status"
                                .to_owned(),
                        ));
                    }
                    428 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Process disrupted: Process has been manually disrupted".to_owned(),
                        ));
                    }
                    500 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Unexpected error: Service process has finished with an unknow error"
                                .to_owned(),
                        ));
                    }
                    503 => {
                        return Err(MessageError::ProcessingError(
                            job_id,
                            "Service unreachable: Service could not be reached".to_owned(),
                        ));
                    }
                    _ => {}
                }
            }

            let ten_seconds = time::Duration::from_secs(10);
            thread::sleep(ten_seconds);
        }
    } else {
        return Err(MessageError::ProcessingError(
            job_id,
            format!("Unable get location to wait end of ingest"),
        ));
    }
}

#[test]
fn test_mapping_video() {
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use video_model::metadata::Metadata;

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

    let rdf_triples = convert_into_rdf(666, &video_metadata, true).unwrap();

    let mut ntriple_struct = String::new();
    let mut ntriple_file = File::open("tests/triples.nt").unwrap();
    let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
    println!("{}", rdf_triples);
    assert!(rdf_triples == ntriple_struct);
}

#[test]
fn test_mapping_resource() {
    use resource_model::Resource;
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

    let rdf_triples = convert_into_rdf(666, &resource, true).unwrap();

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
