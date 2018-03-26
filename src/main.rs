#![recursion_limit = "192"]

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate rdf_translator;
extern crate serde;
// #[macro_use]
// extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use futures::{future, Future, Stream};

use hyper::{Post, Get, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

mod config;
mod convert;

struct RdfConverter;

impl Service for RdfConverter {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item = Response, Error = hyper::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let (method, uri, _version, headers, body) = req.deconstruct();
    match (method, uri.path()) {
      (Post, "/convert") => {
        let mut res = Response::new();

        let vec;
        if let Some(len) = headers.get::<ContentLength>() {
          vec = Vec::with_capacity(**len as usize);
        } else {
          vec = vec![];
        }

        let response =
          body.fold(vec, |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
          }).and_then(move |value| {
            let response_body = convert::process(&value);

            println!("CONVERSION COMPLETED");

            future::ok(res
              .with_header(ContentLength(response_body.len() as u64))
              .with_header(ContentType::text_utf8())
              .with_body(response_body))
          });
        Box::new(response)
      }
      (Get, "/") => {
        let version_major: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
        let version_minor: &'static str = env!("CARGO_PKG_VERSION_MINOR");
        let version_patch: &'static str = env!("CARGO_PKG_VERSION_PATCH");
        let json_version = json!({
          "service": "rdf_worker",
          "version": {
            "major": version_major,
            "minor": version_minor,
            "patch": version_patch
          }
        });
        let response_body = serde_json::to_string(&json_version).unwrap();
        let response =
          Response::new()
          .with_header(ContentLength(response_body.len() as u64))
          .with_header(ContentType::json())
          .with_body(response_body);
        Box::new(future::ok(response))
      },
      _ => {
        let response =
          Response::new()
          .with_status(StatusCode::NotFound);

        Box::new(future::ok(response))
      }
    }
  }
}

fn main() {
  pretty_env_logger::init().unwrap();

  let server_address = config::get_hostname() + ":" + config::get_port().as_str();
  let addr = server_address.parse().unwrap();

  let server = Http::new().bind(&addr, || Ok(RdfConverter)).unwrap();
  println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
  server.run().unwrap();
}
