
use amqp_worker::*;
use config;
use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::turtle_writer::TurtleWriter;
use rdf::writer::rdf_writer::RdfWriter;
use reqwest;
use reqwest::StatusCode;
use reqwest::header::*;
use serde_json;
use std::{thread, time};

use model::metadata::Metadata;
use namespaces::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameter {
  #[serde(rename="type")]
  kind: String,
  id: String,
  default: Option<String>,
  value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
  job_id: u64,
  parameters: Vec<Parameter>
}

fn get_parameter(params: &Vec<Parameter>, key: &str) -> Option<String> {
  for param in params.iter() {
    if param.id == key {
      if let Some(ref value) = param.value {
        return Some(value.clone())
      } else {
        return param.default.clone()
      }
    }
  }
  None
}

#[derive(Debug, Serialize)]
struct Session {
  email: String,
  password: String
}

#[derive(Debug, Serialize)]
struct SessionBody {
  session: Session
}

#[derive(Debug, Deserialize)]
struct SessionResponseBody {
  access_token: String
}

#[derive(Debug, Deserialize)]
struct DataResponseBody {
  id: u32,
  key: String,
  value: String,
  inserted_at: String,
}

#[derive(Debug, Deserialize)]
struct ValueResponseBody {
  data: DataResponseBody
}

fn get_value(client: &reqwest::Client, job_id: u64, key: &str) -> Result<String, MessageError> {
  let backend_endpoint = config::get_backend_hostname();
  let backend_username = config::get_backend_username();
  let backend_password = config::get_backend_password();

  let session_body = SessionBody {
    session: Session {
      email: backend_username,
      password: backend_password
    }
  };

  let mut response =
    client
    .post(&(backend_endpoint.clone() + "/sessions"))
    .json(&session_body)
    .send()
    .map_err(|e|
      MessageError::ProcessingError(job_id, e.to_string())
    )?;

  let r : SessionResponseBody = response.json().unwrap();
  let token = r.access_token;

  let mut response =
    client
    .get(&(backend_endpoint + "/credentials/" + key))
    // .bearer_auth(token)
    .header("Authorization", token)
    .send()
    .map_err(|e|
      MessageError::ProcessingError(job_id, e.to_string())
    )?;

  let resp_value : ValueResponseBody = response.json().unwrap();
  Ok(resp_value.data.value)
}

#[derive(Debug)]
struct PmConfig {
  endpoint: String,
  client_id: String,
  api_key: String,
}

fn get_perfect_memory_config(job_id: u64, parameters: &Vec<Parameter>) -> Result<PmConfig, MessageError> {
  let endpoint_key = get_parameter(&parameters, "perfect_memory_endpoint").unwrap_or("PERFECT_MEMORY_ENDPOINT".to_string());
  let username_key = get_parameter(&parameters, "perfect_memory_username").unwrap_or("PERFECT_MEMORY_USERNAME".to_string());
  let password_key = get_parameter(&parameters, "perfect_memory_password").unwrap_or("PERFECT_MEMORY_PASSWORD".to_string());

  let client = reqwest::Client::builder()
    .build()
    .unwrap();

  let endpoint = get_value(&client, job_id, &endpoint_key)?;
  let client_id = get_value(&client, job_id, &username_key)?;
  let api_key = get_value(&client, job_id, &password_key)?;

  Ok(PmConfig {
    endpoint,
    client_id,
    api_key,
  })
}

pub fn process(message: &str) -> Result<u64, MessageError> {
  let parsed: Result<Job, _> = serde_json::from_str(message);

  match parsed {
    Ok(content) => {
      println!("{:?}", content);
      let reference = get_parameter(&content.parameters, "reference").unwrap();
      let video_metadata = get_video_metadata(content.job_id.clone(), &reference)?;
      let rdf_triples = convert_into_rdf(content.job_id.clone(), &video_metadata, false)?;

      info!("{}", rdf_triples);
      let config = get_perfect_memory_config(content.job_id.clone(), &content.parameters)?;

      publish_to_perfect_memory(content.job_id.clone(), &config.client_id, &config.api_key, &config.endpoint, &rdf_triples)?;
      Ok(content.job_id)
    },
    Err(msg) => {
      println!("ERROR {:?}", msg);
      return Err(MessageError::RuntimeError("bad input message".to_string()));
    }
  }
}

pub fn get_video_metadata(job_id: u64, reference: &str) -> Result<Metadata, MessageError> {
  let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/videos/".to_owned() + reference;

  let client = reqwest::Client::builder()
    .build()
    .unwrap();

  let mut response =
    client
    .get(url.as_str())
    .send()
    .map_err(|e|
      MessageError::ProcessingError(job_id, e.to_string())
    )?;

  let status = response.status();

  if !(status == StatusCode::OK) {
    error!("{:?}", response);
    return Err(MessageError::ProcessingError(job_id, "bad response status".to_string()));
  }

  response.json()
  .map_err(|e|
    MessageError::ProcessingError(job_id, e.to_string())
  )
}

pub fn convert_into_rdf(job_id: u64, metadata: &Metadata, ntriples: bool) -> Result<String, MessageError> {
  let mut graph = Graph::new(None);
  graph.add_namespace(&Namespace::new("rdf".to_string(), Uri::new(RDF_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("rdfs".to_string(), Uri::new(RDFS_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("owl".to_string(), Uri::new(OWL_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("ebucore".to_string(), Uri::new(EBUCORE_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("francetv".to_string(), Uri::new(FRANCETV_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("xsi".to_string(), Uri::new(XSI_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("default".to_string(), Uri::new(DEFAULT_NAMESPACE.to_owned())));

  metadata.to_rdf(&mut graph);
  if ntriples {
    let writer = NTriplesWriter::new();
    writer.write_to_string(&graph).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
  } else {
    let writer = TurtleWriter::new(graph.namespaces());
    writer.write_to_string(&graph).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
  }
}

#[derive(Serialize)]
struct InfosGraph {
  value: String,
  #[serde(rename="type")]
  kind: String
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

pub fn publish_to_perfect_memory(job_id: u64, pm_client_id: &str, pm_api_key: &str, pm_endpoint: &str, triples: &str) -> Result<(), MessageError> {
  let url = pm_endpoint.to_owned() + "/v1/requests";

  let client = reqwest::Client::builder()
    .build()
    .unwrap();

  let body = PmRequestBody {
    client_id: pm_client_id.to_owned(),
    name: "push_rdf_infos".to_string(),
    inputs: Inputs {
      infos_graph: InfosGraph {
        value: base64::encode(triples),
        kind: "binary".to_string()
      }
    }
  };

  let mut response =
    client
    .post(url.as_str())
    .header(CACHE_CONTROL, "no-cache")
    .header(CONTENT_TYPE, "application/json")
    .header("X-Api-Key", pm_api_key)
    .json(&body)
    .send()
    .map_err(|e|
      MessageError::ProcessingError(job_id, e.to_string())
    )?;

  if response.status() != 201 {
    let text = response.text().unwrap_or("unknown reason.".to_owned());
    error!("unable to push to Perfect Memory: {:?}\n{}", response, text);
    return Err(MessageError::ProcessingError(job_id, format!("Unable to push into Perfect Memory: {}", text)));
  }

  if let Some(location) = response.headers().get(LOCATION) {
    loop {
      let mut response =
        client
        .get(location.to_str().unwrap())
        .header("X-Api-Key", pm_api_key)
        .send()
        .map_err(|e|
          MessageError::ProcessingError(job_id, e.to_string())
        )?;

      if response.status() != 200 {
        let ten_seconds = time::Duration::from_secs(10);
        thread::sleep(ten_seconds);
        continue;
      }

      let r: Result<PmResponseBody, _> = response.json();
      error!("Perfect Memory response: {:?}", r);
      if let Ok(resp_body) = r {
        match resp_body.status {
          200 | 300 => {return Ok(()); },
          100 | 110 | 120 => {},
          400 => {return Err(MessageError::ProcessingError(job_id, "Error: Request/Process has finished with an error".to_owned())); },
          401 => {return Err(MessageError::ProcessingError(job_id, "Error on child process: Process has finished with an error on one of its children".to_owned())); },
          408 => {return Err(MessageError::ProcessingError(job_id, "Error Service: Process has finished with a specific error".to_owned())); },
          410 => {return Err(MessageError::ProcessingError(job_id, "Item Disabled: The item is disabled".to_owned())); },
          414 => {return Err(MessageError::ProcessingError(job_id, "Item Not Found: The item is not found".to_owned())); },
          421 => {return Err(MessageError::ProcessingError(job_id, "Invalid Script: There was an error while running the script".to_owned())); },
          422 => {return Err(MessageError::ProcessingError(job_id, "Invalid I/O: The input or the output is invalid".to_owned())); },
          423 => {return Err(MessageError::ProcessingError(job_id, "Invalid Status: The process has been stopped with an invalid status".to_owned())); },
          428 => {return Err(MessageError::ProcessingError(job_id, "Process disrupted: Process has been manually disrupted".to_owned())); },
          500 => {return Err(MessageError::ProcessingError(job_id, "Unexpected error: Service process has finished with an unknow error".to_owned())); },
          503 => {return Err(MessageError::ProcessingError(job_id, "Service unreachable: Service could not be reached".to_owned())); },
          _ => {},
        }
      }

      let ten_seconds = time::Duration::from_secs(10);
      thread::sleep(ten_seconds);
    }
  } else {
    return Err(MessageError::ProcessingError(job_id, format!("Unable get location to wait end of ingest")));
  }
}


#[test]
fn test_mapping() {
  use std::fs::File;
  use std::io::Read;
  use serde_json;

  let mut video_struct = String::new();
  let mut video_file = File::open("tests/video.json").unwrap();
  let _ = video_file.read_to_string(&mut video_struct).unwrap();

  let video_metadata = serde_json::from_str(&video_struct).unwrap();
  let rdf_triples = convert_into_rdf(666, &video_metadata, true).unwrap();

  let mut ntriple_struct = String::new();
  let mut ntriple_file = File::open("tests/triples.nt").unwrap();
  let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
  assert!(rdf_triples == ntriple_struct);
}

#[test]
fn test_publish() {
  use std::fs::File;
  use std::io::Read;

  let mut ntriple_struct = String::new();
  let mut ntriple_file = File::open("tests/triples.nt").unwrap();
  let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();

  let pm_client_id = "5ab4ca78dd37d3000c64912e";
  let pm_api_key = "mxzzM934dGxxojcjNYxi";
  let pm_endpoint = "https://exchange-manager-api.platform.labs.pm";
  publish_to_perfect_memory(666, pm_client_id, pm_api_key, pm_endpoint, &ntriple_struct).unwrap();
}


