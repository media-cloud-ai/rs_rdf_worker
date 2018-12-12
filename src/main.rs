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

use amqp_worker::*;
use std::env;
use log::Level;

mod config;
mod message;
mod model;
mod namespaces;
mod resource_model;

#[derive(Debug)]
struct HttpEvent {
}

impl MessageEvent for HttpEvent {
  fn process(&self, message: &str) -> Result<u64, MessageError> {
    message::process(message)
  }
}

fn main() {
  if let Ok(_)= env::var("VERBOSE") {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  let http_event = HttpEvent{};
  start_worker(&http_event);
}
