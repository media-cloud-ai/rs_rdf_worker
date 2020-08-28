use crate::namespaces::*;
use rdf::{graph::Graph, node::Node, triple::Triple, uri::Uri};

pub(crate) fn add_link(
  graph: &mut Graph,
  subject_node: &Node,
  predicate: &str,
  object: &str,
  language: Option<&str>,
  datatype: Option<String>,
  uri: bool,
) {
  let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));
  let object_node = if let Some(l) = language {
    graph.create_literal_node_with_language(object.to_string(), l.to_string())
  } else if let Some(ref dt) = datatype {
    graph.create_literal_node_with_data_type(object.to_string(), &Uri::new(dt.to_string()))
  } else if uri {
    graph.create_uri_node(&Uri::new(object.to_string()))
  } else {
    graph.create_literal_node(object.to_string())
  };

  let triple = Triple::new(&subject_node, &predicate_node, &object_node);
  graph.add_triple(&triple);
}

pub(crate) fn add_related_node(graph: &mut Graph, subject_node: &Node, predicate: &str) -> Node {
  let blank = graph.create_blank_node();
  let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));

  let triple = Triple::new(&subject_node, &predicate_node, &blank);
  graph.add_triple(&triple);
  blank
}

pub(crate) fn insert_identifier(
  graph: &mut Graph,
  subject_node: &Node,
  identifier_type: &str,
  value: &str,
) {
  let p_has_idenfitier = EBUCORE_NAMESPACE.to_owned() + "hasIdentifier";
  let p_has_identifier_type = EBUCORE_NAMESPACE.to_owned() + "hasIdentifierType";
  let p_idenfitier_value = EBUCORE_NAMESPACE.to_owned() + "identifierValue";
  let p_type = RDF_NAMESPACE.to_owned() + "type";

  let o_identifier = EBUCORE_NAMESPACE.to_owned() + "Identifier";

  let s_identifier = add_related_node(graph, &subject_node, &p_has_idenfitier);
  add_link(
    graph,
    &s_identifier,
    &p_type,
    &o_identifier,
    None,
    None,
    true,
  );
  add_link(
    graph,
    &s_identifier,
    &p_idenfitier_value,
    value,
    None,
    None,
    false,
  );
  add_link(
    graph,
    &s_identifier,
    &p_has_identifier_type,
    &format!(
      "http://resources.idfrancetv.fr/identifiers/{}",
      identifier_type
    ),
    None,
    None,
    true,
  );
}

pub(crate) fn add_triple(graph: &mut Graph, subject: &str, predicate: &str, object: &str) -> Node {
  let subject_node = graph.create_uri_node(&Uri::new(subject.to_string()));
  let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));
  let object_node = graph.create_uri_node(&Uri::new(object.to_string()));

  let triple = Triple::new(&subject_node, &predicate_node, &object_node);
  graph.add_triple(&triple);
  subject_node
}
