
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
      let video_metadata = get_video_metadata(content.job_id.clone(), &parameters.reference)?;
      let _rdf_triples = convert_into_rdf(content.job_id.clone(), &video_metadata)?;

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
    println!("ERROR {:?}", response);
    return Err(MessageError::ProcessingError(job_id, "bad response status".to_string()));
  }

  response.json()
  .map_err(|e|
    MessageError::ProcessingError(job_id, e.to_string())
  )
}

pub fn convert_into_rdf(job_id: u64, metadata: &Metadata) -> Result<String, MessageError> {
  let mut graph = Graph::new(None);
  graph.add_namespace(&Namespace::new("rdf".to_string(), Uri::new(RDF_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("rdfs".to_string(), Uri::new(RDFS_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("owl".to_string(), Uri::new(OWL_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("ebucore".to_string(), Uri::new(EBUCORE_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("francetv".to_string(), Uri::new(FRANCETV_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("xsi".to_string(), Uri::new(XSI_NAMESPACE.to_owned())));
  graph.add_namespace(&Namespace::new("default".to_string(), Uri::new(DEFAULT_NAMESPACE.to_owned())));

  metadata.to_rdf(&mut graph);
  let writer = NTriplesWriter::new();
  writer.write_to_string(&graph).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))
}


#[test]
fn test_mapping() {
  let video_metadata = get_video_metadata(666, "a570fa6f-d2c9-455d-bacf-f21b4957809f").unwrap();
  let _rdf_triples = convert_into_rdf(666, &video_metadata).unwrap();
}
