extern crate amqp_worker;
extern crate base64;
extern crate clap;
#[macro_use]
extern crate log;
extern crate rdf;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
extern crate uuid;

use amqp_worker::*;
use clap::{Arg, App};
use log::Level;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod message;
mod namespaces;
mod resource_model;
mod video_model;

#[derive(Debug)]
struct RdfEvent {}

impl MessageEvent for RdfEvent {
  fn process(&self, message: &str) -> Result<u64, MessageError> {
    message::process(message)
  }
}

static RDF_EVENT: RdfEvent = RdfEvent {};

fn main() {
  let matches =
      App::new("RDF Worker")
      .version("0.1.3")
      .author("Marc-Antoine Arnaud <maarnaud@media-io.com>")
      .arg(Arg::with_name("reference")
        .short("r")
        .help("video reference UUID")
        .required(true)
        .index(1))
      .get_matches();

  if env::var("VERBOSE").is_ok() {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  if env::var("COMMAND_LINE").is_ok() {
    let ntriples = true;
    let reference = matches.value_of("reference").expect("missing reference parameter");
    let job_id = 666;

    let mut video_metadata = message::get_video_metadata(job_id, &reference).expect("unable to get video metadata");
    let si_video_files = message::get_files(job_id, &reference).expect("unable to get files for this video");
    video_metadata.resources = resource_model::Resources {
        items: si_video_files,
    };

    let rdf = message::convert_into_rdf(job_id, &video_metadata, ntriples).expect("unable to convert into rdf");

    let extension =
      if ntriples {
        ".nt"
      } else {
        ".turtle"
      };

    let mut file = File::create(reference.to_string() + extension).expect("unable to open output file");
    file.write_all(&rdf.into_bytes()).expect("error during writing RDF informations into the output file");

    println!("RDF generated for reference {}{}", reference, extension);
  } else {
  start_worker(&RDF_EVENT);
  }
}
