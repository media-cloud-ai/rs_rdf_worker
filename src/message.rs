
use amqp_worker::*;
use serde_json;
use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use reqwest;
use reqwest::StatusCode;

use model::metadata::Metadata;
use namespaces::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameters {
  reference: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
  job_id: u64,
  parameters: Parameters
}

pub fn process(message: &str) -> Result<u64, MessageError> {

  let parsed: Result<Job, _> = serde_json::from_str(message);

  match parsed {
    Ok(content) => {
      println!("{:?}", content);

      let parameters = content.parameters.clone();
      let url = "https://gatewayvf.webservices.francetelevisions.fr/v1/videos/".to_owned() + &parameters.reference;

      let client = reqwest::Client::builder()
        .build()
        .unwrap();

      let mut response =
        client
        .get(url.as_str())
        .send()
        .map_err(|e|
          MessageError::ProcessingError(content.job_id.clone(), e.to_string())
        )?;

      let status = response.status();

      if !(status == StatusCode::Ok) {
        println!("ERROR {:?}", response);
        return Err(MessageError::ProcessingError(content.job_id.clone(), "bad response status".to_string()));
      }

      let metadatas : Metadata =
        response.json()
        .map_err(|e|
          MessageError::ProcessingError(content.job_id.clone(), e.to_string())
        )?;

      println!("{:?}", metadatas);

      let mut graph = Graph::new(None);
      graph.add_namespace(&Namespace::new("rdf".to_string(), Uri::new(RDF_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("rdfs".to_string(), Uri::new(RDFS_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("owl".to_string(), Uri::new(OWL_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("ebucore".to_string(), Uri::new(EBUCORE_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("francetv".to_string(), Uri::new(FRANCETV_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("xsi".to_string(), Uri::new(XSI_NAMESPACE.to_owned())));
      graph.add_namespace(&Namespace::new("default".to_string(), Uri::new(DEFAULT_NAMESPACE.to_owned())));

      metadatas.to_rdf(&mut graph);
      let writer = NTriplesWriter::new();
      let dumped = writer.write_to_string(&graph).unwrap();
      println!("{}", dumped);

      Ok(content.job_id)
    },
    Err(msg) => {
      println!("ERROR {:?}", msg);
      return Err(MessageError::RuntimeError("bad input message".to_string()));
    }
  }
}

#[test]
fn test_mapping() {
  use amqp_worker::MessageError::{ProcessingError, RuntimeError};

  let job = "{\"job_id\": 666, \"parameters\": { \"reference\": \"a570fa6f-d2c9-455d-bacf-f21b4957809f\"}}";
  // let job = "{\"job_id\": 666, \"parameters\": { \"reference\": \"08c8dc14-b831-4bb1-b853-6d883f97c939\"}}";
  if let Err(error) = process(job) {
    match error {
      RuntimeError(msg) => {
        println!("ERROR {:?}", msg);
      },
      ProcessingError(_job_id, msg) => {
        println!("ERROR {:?}", msg);
      }
      _ => {},
    }
  }
  assert!(false);
}
