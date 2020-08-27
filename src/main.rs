#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, ArgMatches};
use futures::executor::block_on;
use mcai_worker_sdk::job::JobResult;
use mcai_worker_sdk::MessageEvent;
use mcai_worker_sdk::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod message;
mod namespaces;
mod resource_model;
mod video_model;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Default)]
struct RdfEvent {}

#[derive(Debug)]
struct PmConfig {
    endpoint: String,
    client_id: String,
    api_key: String,
}

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

impl RdfWorkerParameters {
    fn get_perfect_memory_config(&self) -> PmConfig {
        PmConfig {
            endpoint: self.perfect_memory_endpoint.clone(),
            client_id: self.perfect_memory_username.clone(),
            api_key: self.perfect_memory_password.clone(),
        }
    }
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
    let job_result = JobResult::new(666);

    let mut video_metadata = block_on(message::get_video_metadata(job_result.clone(), &reference))
        .expect("unable to get video metadata");
    let si_video_files = block_on(message::get_files(job_result.clone(), &reference))
        .expect("unable to get files for this video");
    video_metadata.resources = resource_model::Resources {
        items: si_video_files,
    };

    let rdf = message::convert_into_rdf(job_result.clone(), &video_metadata, ntriples)
        .expect("unable to convert into rdf");

    let extension = if ntriples { ".nt" } else { ".turtle" };

    let mut file =
        File::create(reference.to_string() + extension).expect("unable to open output file");
    file.write_all(&rdf.into_bytes())
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
