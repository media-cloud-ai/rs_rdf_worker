
use rdf_translator::Converter;
use rdf_translator::Mapper;
use serde_json;

pub fn process(body: &Vec<u8>) -> String {
  println!("PROCESS");

  let doc : serde_json::Value = serde_json::from_slice(body).unwrap();

  let mut converter = Converter::new();
  let mapper = Mapper::load("/Users/marco/dev/mediaio/rdf_translator/tests/mapping.json");
  mapper.process(doc, &mut converter);
  let content = converter.to_turtle_string();
  content
}
