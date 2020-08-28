#[macro_use]
extern crate serde_derive;

use crate::convert::convert_into_rdf;
use clap::{App, Arg, ArgMatches};
use mcai_worker_sdk::job::JobResult;
use mcai_worker_sdk::MessageEvent;
use mcai_worker_sdk::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod convert;
mod message;
mod namespaces;
mod perfect_memory;
mod rdf_graph;
mod resource_model;
mod video_model;

pub mod built_info {
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Default)]
struct RdfEvent {}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub enum Order {
  #[serde(rename = "publish_metadata")]
  PublishMetadata,
  #[serde(rename = "publish_dash_and_ttml")]
  PublishDashAndTtml,
}

impl Default for Order {
  fn default() -> Self {
    Order::PublishMetadata
  }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct RdfWorkerParameters {
  input_paths: Option<Vec<String>>, // depends on the order?
  ntriples: Option<bool>,
  order: Option<Order>,
  perfect_memory_event_name: Option<String>,
  reference: String,
  storage: Option<String>,    // depends on the order?
  url_prefix: Option<String>, // depends on the order?
  perfect_memory_endpoint: String,
  perfect_memory_username: String,
  perfect_memory_password: String,
}

impl MessageEvent<RdfWorkerParameters> for RdfEvent {
  fn get_name(&self) -> String {
    "RDF Worker".to_string()
  }

  fn get_short_description(&self) -> String {
    "Worker to convert video metadata to RDF".to_string()
  }

  fn get_description(&self) -> String {
    r#"This worker retrieves the video metadata from the FTV video factory and converts its format from JSON to RDF.
It can be used as a worker or as a simple command line (if the `COMMAND_LINE` environment variable is set).
In worker mode, the RDF metadata can be published to the Perfect Memory system. In executable mode, the RDF is written into a file."#
      .to_string()
  }

  fn get_version(&self) -> Version {
    Version::parse(built_info::PKG_VERSION).expect("unable to locate Package version")
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    parameters: RdfWorkerParameters,
    job_result: JobResult,
  ) -> Result<JobResult> {
    message::process(channel, parameters, job_result)
  }
}

fn execute_command_line(matches: ArgMatches) {
  let ntriples = true;
  let reference = matches
    .value_of("reference")
    .expect("missing reference parameter");

  let mut video_metadata =
    message::get_video_metadata(&reference).expect("unable to get video metadata");
  let si_video_files = message::get_files(&reference).expect("unable to get files for this video");
  video_metadata.resources = resource_model::Resources {
    items: si_video_files,
  };

  let rdf = convert_into_rdf(&video_metadata, ntriples).expect("unable to convert into rdf");

  let extension = if ntriples { ".nt" } else { ".turtle" };

  let mut file =
    File::create(reference.to_string() + extension).expect("unable to open output file");
  file
    .write_all(&rdf.into_bytes())
    .expect("error during writing RDF informations into the output file");

  println!("RDF generated for reference {}{}", reference, extension);
}

fn main() {
  let message_event = RdfEvent::default();

  if env::var("COMMAND_LINE").is_ok() {
    let matches = App::new(message_event.get_name())
      .version(message_event.get_version().to_string().as_str())
      .author("Marc-Antoine Arnaud <maarnaud@media-io.com>")
      .arg(
        Arg::with_name("reference")
          .short("r")
          .help("video reference UUID")
          .required(true)
          .index(1),
      )
      .get_matches();

    execute_command_line(matches);
  } else {
    start_worker(message_event);
  }
}
