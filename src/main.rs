extern crate amqp_worker;
extern crate base64;
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
use log::Level;
use std::env;

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
  if env::var("VERBOSE").is_ok() {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  start_worker(&RDF_EVENT);
}
