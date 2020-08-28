use std::{thread, time};

use mcai_worker_sdk::{
  error,
  job::{JobResult, JobStatus},
  MessageError, Result,
};
use reqwest::header::{CACHE_CONTROL, CONTENT_TYPE, LOCATION};

use crate::RdfWorkerParameters;

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

#[derive(Debug)]
pub(crate) struct PmConfig {
  pub(crate) endpoint: String,
  pub(crate) client_id: String,
  pub(crate) api_key: String,
}

impl From<RdfWorkerParameters> for PmConfig {
  fn from(parameters: RdfWorkerParameters) -> Self {
    PmConfig {
      endpoint: parameters.perfect_memory_endpoint.clone(),
      client_id: parameters.perfect_memory_username.clone(),
      api_key: parameters.perfect_memory_password,
    }
  }
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

pub(crate) fn publish_to_perfect_memory(
  job_result: JobResult,
  pm_client_id: &str,
  pm_event_name: &str,
  pm_api_key: &str,
  pm_endpoint: &str,
  triples: &str,
) -> Result<()> {
  let url = pm_endpoint.to_owned() + "/v1/requests";

  let client = reqwest::blocking::Client::builder()
    .build()
    .map_err(|error| MessageError::ProcessingError(job_result.clone().with_error(error)))?;

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
      .unwrap_or_else(|_| "unknown reason.".to_string());
    error!("Unable to push to Perfect Memory: {}", text);
    return Err(MessageError::ProcessingError(
      job_result
        .with_status(JobStatus::Error)
        .with_message(&format!("Unable to push into Perfect Memory: {}", text)),
    ));
  }

  let location = response.headers().get(LOCATION).ok_or_else(|| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message("Unable get location to wait end of ingest"),
    )
  })?;
  let location_str = location.to_str().map_err(|e| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&e.to_string()),
    )
  })?;

  loop {
    let response = client
      .get(location_str)
      .header("X-Api-Key", pm_api_key)
      .send()
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

    let resp_body: PmResponseBody = response.json().map_err(|error| {
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
            .with_status(JobStatus::Error)
            .with_message("Error: Request/Process has finished with an error"),
        ));
      }
      401 => {
        return Err(MessageError::ProcessingError(
          job_result.with_status(JobStatus::Error).with_message(
            "Error on child process: Process has finished with an error on one of its children",
          ),
        ));
      }
      408 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Error Service: Process has finished with a specific error"),
        ));
      }
      410 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Item Disabled: The item is disabled"),
        ));
      }
      414 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Item Not Found: The item is not found"),
        ));
      }
      421 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Invalid Script: There was an error while running the script"),
        ));
      }
      422 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Invalid I/O: The input or the output is invalid"),
        ));
      }
      423 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Invalid Status: The process has been stopped with an invalid status"),
        ));
      }
      428 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Process disrupted: Process has been manually disrupted"),
        ));
      }
      500 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Unexpected error: Service process has finished with an unknow error"),
        ));
      }
      503 => {
        return Err(MessageError::ProcessingError(
          job_result
            .with_status(JobStatus::Error)
            .with_message("Service unreachable: Service could not be reached"),
        ));
      }
      _ => {}
    }

    let ten_seconds = time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
  }
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
