// use rdf_translator::Converter;
// use rdf_translator::Mapper;
// use serde_json;
// use config::get_mapping_file;
//
// pub fn process(body: &Vec<u8>) -> String {
//   let doc : serde_json::Value = serde_json::from_slice(body).unwrap();
//   println!("PROCESS");
//   println!("{}", doc);
//
//   let mut converter = Converter::new();
//   let mapper = Mapper::load(&get_mapping_file());
//   mapper.process(&doc, &mut converter);
//   let content = converter.to_turtle_string();
//   content
// }

use crate::namespaces::*;
use rdf::{
  error::Error,
  graph::Graph,
  namespace::Namespace,
  uri::Uri,
  writer::{n_triples_writer::NTriplesWriter, rdf_writer::RdfWriter, turtle_writer::TurtleWriter},
};

pub trait ToRdf {
  fn to_rdf(&self, graph: &mut Graph) -> Result<(), Error>;
}

pub fn convert_into_rdf<T: ToRdf>(item: &T, n_triples: bool) -> Result<String, Error> {
  let mut graph = Graph::new(None);
  graph.add_namespace(&Namespace::new(
    "rdf".to_string(),
    Uri::new(RDF_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "rdfs".to_string(),
    Uri::new(RDFS_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "owl".to_string(),
    Uri::new(OWL_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "ebucore".to_string(),
    Uri::new(EBUCORE_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "francetv".to_string(),
    Uri::new(FRANCETV_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "xsi".to_string(),
    Uri::new(XSI_NAMESPACE.to_owned()),
  ));
  graph.add_namespace(&Namespace::new(
    "default".to_string(),
    Uri::new(DEFAULT_NAMESPACE.to_owned()),
  ));

  item.to_rdf(&mut graph)?;
  if n_triples {
    let writer = NTriplesWriter::new();
    writer.write_to_string(&graph)
  } else {
    let writer = TurtleWriter::new(graph.namespaces());
    writer.write_to_string(&graph)
  }
}

#[cfg(test)]
use crate::resource_model::{ExternalIds, Format, Resource, Resources};

#[test]
fn test_mapping_video() {
  use crate::video_model::metadata::Metadata;
  use serde_json;
  use std::fs::File;
  use std::io::Read;

  let mut video_struct = String::new();
  let mut video_file = File::open("tests/video.json").unwrap();
  let _ = video_file.read_to_string(&mut video_struct).unwrap();

  let mut sifiles_struct = String::new();
  let mut sifiles_file = File::open("tests/files.json").unwrap();
  let _ = sifiles_file.read_to_string(&mut sifiles_struct).unwrap();

  let mut video_metadata: Metadata = serde_json::from_str(&video_struct).unwrap();
  let ftv_resources: Vec<Resource> = serde_json::from_str(&sifiles_struct).unwrap();
  video_metadata.resources = Resources {
    items: ftv_resources,
  };

  let rdf_triples = convert_into_rdf(&video_metadata, true).unwrap();

  let mut ntriple_struct = String::new();
  let mut ntriple_file = File::open("tests/triples.nt").unwrap();
  let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
  println!("{}", rdf_triples);
  assert_eq!(rdf_triples, ntriple_struct);
}

#[test]
fn test_mapping_resource() {
  use crate::resource_model::Resource;
  use std::fs::File;
  use std::io::Read;

  let resource = Resource {
    id: "000000-1111-2222-3333-44444444".to_string(),
    created_via: "Media-IO".to_string(),
    format: Format {
      id: "playlist-hls".to_string(),
      label: "playlist/hls".to_string(),
      kind: "playlist".to_string(),
      mime_type: "urn:mimetype:application/dash+xml".to_string(),
    },
    storage: "akamai-video-prod".to_string(),
    path: None,
    filename: Some("/path/to/manifest.mpd".to_string()),
    audio_tracks: vec![],
    text_tracks: vec![],
    video_tracks: vec![],
    created_at: None,
    updated_at: None,
    ratio: None,
    width: None,
    height: None,
    index: None,
    copyright: None,
    filesize_bytes: None,
    bitrate_kbps: None,
    md5_checksum: None,
    tags: vec![],
    url: Some(format!(
      "{}{}",
      "http://videos-pmd.francetv.fr/innovation/SubTil/", "/path/to/manifest.mpd"
    )),
    version: None,
    lang: None,
    external_ids: ExternalIds {
      video_id: Some("44444444-3333-2222-1111-000000".to_string()),
      legacy_id: None,
      group_id: None,
      job_id: None,
      remote_id: None,
    },
  };

  let rdf_triples = convert_into_rdf(&resource, true).unwrap();

  let mut ntriple_struct = String::new();
  let mut ntriple_file = File::open("tests/triples_resource.nt").unwrap();
  let _ = ntriple_file.read_to_string(&mut ntriple_struct).unwrap();
  println!("{}", rdf_triples);
  assert!(rdf_triples == ntriple_struct);
}
