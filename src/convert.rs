
use rdf_translator::Converter;
use rdf_translator::Mapper;
use serde_json;

use config::get_mapping_file;

pub fn process(body: &Vec<u8>) -> String {
  let doc : serde_json::Value = serde_json::from_slice(body).unwrap();
  println!("PROCESS");
  println!("{}", doc);

  let mut converter = Converter::new();
  let mapper = Mapper::load(&get_mapping_file());
  mapper.process(&doc, &mut converter);
  let content = converter.to_turtle_string();
  content
}
