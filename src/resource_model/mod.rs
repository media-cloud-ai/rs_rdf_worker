
use message::ToRdf;
use namespaces::*;
use rdf::graph::Graph;
use rdf::node::Node;
use rdf::triple::Triple;
use rdf::uri::Uri;

#[derive(Debug)]
pub struct Resource {
  pub id: String,
  pub creator: Option<String>,
  pub mime_type: String,
  pub locators: Vec<String>
}

impl ToRdf for Resource {
  fn to_rdf(&self, graph: &mut Graph) {
    let root = "http://ressources.idfrancetv.fr/medias/".to_string() + &self.id;
    let p_type = RDF_NAMESPACE.to_owned() + "type";
    let p_has_creator = EBUCORE_NAMESPACE.to_owned() + "hasCreator";
    let p_has_related_resource = EBUCORE_NAMESPACE.to_owned() + "hasRelatedResource";
    let p_has_format = EBUCORE_NAMESPACE.to_owned() + "hasFormat";
    let p_has_mime_type = EBUCORE_NAMESPACE.to_owned() + "hasMimeType";
    let p_locator = EBUCORE_NAMESPACE.to_owned() + "locator";
    let o_media_resource = EBUCORE_NAMESPACE.to_owned() + "MediaResource";

    let s_root = graph.create_uri_node(&Uri::new(root));
    let s_has_related_resource = self.add_related_node(graph, &s_root, &p_has_related_resource);

    self.add_link(graph, &s_has_related_resource, &p_type, &o_media_resource, None, None, true);
    let s_has_format = self.add_related_node(graph, &s_has_related_resource, &p_has_format);

    self.add_link(graph, &s_has_format, &p_has_mime_type, &self.mime_type, None, None, false);

    if let Some(ref creator) = self.creator {
      self.add_link(graph, &s_has_related_resource, &p_has_creator, &creator, None, None, false);
    }
    for locator in &self.locators {
      self.add_link(graph, &s_has_related_resource, &p_locator, &locator, None, None, false);
    }
  }
}


impl Resource {
  fn add_link(&self, graph: &mut Graph, subject_node: &Node, predicate: &str, object: &str, language: Option<&str>, datatype: Option<String>, uri: bool) {
    let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));
    let object_node =
      if let Some(l) = language {
        graph.create_literal_node_with_language(object.to_string(), l.to_string())
      } else {
        if let Some(ref dt) = datatype {
          graph.create_literal_node_with_data_type(object.to_string(), &Uri::new(dt.to_string()))
        } else {
          if uri {
            graph.create_uri_node(&Uri::new(object.to_string()))
          } else {
            graph.create_literal_node(object.to_string())
          }
        }
      };

    let triple = Triple::new(&subject_node, &predicate_node, &object_node);
    graph.add_triple(&triple);
  }

  fn add_related_node(&self, graph: &mut Graph, subject_node: &Node, predicate: &str) -> Node {
    let blank = graph.create_blank_node();
    let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));

    let triple = Triple::new(&subject_node, &predicate_node, &blank);
    graph.add_triple(&triple);
    blank
  }
}
