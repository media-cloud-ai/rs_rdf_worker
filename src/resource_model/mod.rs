use crate::convert::ToRdf;
use crate::namespaces::*;
use rdf::graph::Graph;
use rdf::node::Node;
use rdf::triple::Triple;
use rdf::uri::Uri;

#[derive(Debug, Default, Deserialize)]
pub struct Resources {
  #[serde(default)]
  pub items: Vec<Resource>,
}

impl ToRdf for Resources {
  fn to_rdf(&self, graph: &mut Graph) {
    for item in &self.items {
      item.to_rdf(graph);
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct Resource {
  pub bitrate_kbps: Option<u64>,
  #[serde(default)]
  pub video_tracks: Vec<VideoTrack>,
  #[serde(default)]
  pub audio_tracks: Vec<AudioTrack>,
  #[serde(default)]
  pub text_tracks: Vec<TextTrack>,
  pub id: String,
  pub format: Format,
  pub storage: String,
  pub path: Option<String>,
  pub filename: Option<String>,
  pub md5_checksum: Option<String>,
  pub filesize_bytes: Option<u64>,
  pub external_ids: ExternalIds,
  #[serde(default)]
  pub tags: Vec<String>,
  pub created_via: String,
  pub version: Option<String>,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
  pub url: Option<String>,
  pub lang: Option<String>,
  pub ratio: Option<String>,
  pub width: Option<u16>,
  pub height: Option<u16>,
  pub index: Option<u64>,
  pub copyright: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VideoTrack {
  pub codec_rfc6381: Option<String>,
  pub bitrate_kbps: u64,
  pub width: u16,
  pub height: u16,
  pub frame_rate_fps: u8,
}

#[derive(Debug, Deserialize)]
pub struct AudioTrack {
  pub codec_rfc6381: Option<String>,
  pub bitrate_kbps: u64,
  pub sample_rate_hz: u16,
  pub lang: String,
}

#[derive(Debug, Deserialize)]
pub struct TextTrack {}

#[derive(Debug, Deserialize)]
pub struct Format {
  pub id: String,
  pub label: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalIds {
  pub video_id: Option<String>,
  pub legacy_id: Option<String>,
  pub group_id: Option<String>,
  pub job_id: Option<String>,
  pub remote_id: Option<String>,
}

impl ToRdf for Resource {
  fn to_rdf(&self, graph: &mut Graph) {
    if self.external_ids.video_id.is_none() {
      return;
    }

    let s_root = "http://resources.idfrancetv.fr/medias/".to_string()
      + &self.external_ids.video_id.clone().unwrap();
    let root_node = graph.create_uri_node(&Uri::new(s_root));

    let p_has_creator = EBUCORE_NAMESPACE.to_owned() + "hasCreator";
    let p_date_created = EBUCORE_NAMESPACE.to_owned() + "dateCreated";
    let p_date_modified = EBUCORE_NAMESPACE.to_owned() + "dateModified";
    let p_file_size = EBUCORE_NAMESPACE.to_owned() + "fileSize";
    let p_filename = EBUCORE_NAMESPACE.to_owned() + "filename";
    let p_has_format = EBUCORE_NAMESPACE.to_owned() + "hasFormat";
    let p_has_language = EBUCORE_NAMESPACE.to_owned() + "hasLanguage";
    let p_has_related_image = EBUCORE_NAMESPACE.to_owned() + "hasRelatedImage";
    let p_has_related_resource = EBUCORE_NAMESPACE.to_owned() + "hasRelatedResource";
    let p_has_topic = EBUCORE_NAMESPACE.to_owned() + "hasTopic";
    let p_hash_value = EBUCORE_NAMESPACE.to_owned() + "hashValue";
    let p_height = EBUCORE_NAMESPACE.to_owned() + "height";
    let p_height_unit = EBUCORE_NAMESPACE.to_owned() + "heightUnit";
    let p_is_issued_by = EBUCORE_NAMESPACE.to_owned() + "isIssuedby";
    let p_locator = EBUCORE_NAMESPACE.to_owned() + "locator";
    let p_organisation_name = EBUCORE_NAMESPACE.to_owned() + "organisationName";
    let p_resource_id = EBUCORE_NAMESPACE.to_owned() + "resourceId";
    let p_storage_id = EBUCORE_NAMESPACE.to_owned() + "storageId";
    let p_width = EBUCORE_NAMESPACE.to_owned() + "width";
    let p_width_unit = EBUCORE_NAMESPACE.to_owned() + "widthUnit";

    let p_type = RDF_NAMESPACE.to_owned() + "type";

    let p_label = SKOS_NAMESPACE.to_owned() + "label";
    let p_pref_label = SKOS_NAMESPACE.to_owned() + "prefLabel";
    let p_definition = SKOS_NAMESPACE.to_owned() + "definition";

    let o_language = EBUCORE_NAMESPACE.to_owned() + "Language";
    let o_organisation = EBUCORE_NAMESPACE.to_owned() + "Organisation";
    let o_media_resource = EBUCORE_NAMESPACE.to_owned() + "MediaResource";
    let o_picture = EBUCORE_NAMESPACE.to_owned() + "Picture";
    let o_tag = EBUCORE_NAMESPACE.to_owned() + "Tag";

    let s_has_related_object = match self.format.mime_type.as_str() {
      "image/jpeg" => {
        let node = self.add_related_node(graph, &root_node, &p_has_related_image);
        self.add_link(graph, &node, &p_type, &o_picture, None, None, true);

        node
      }
      _ => {
        let node = self.add_related_node(graph, &root_node, &p_has_related_resource);
        self.add_link(graph, &node, &p_type, &o_media_resource, None, None, true);
        node
      }
    };

    self.add_link(
      graph,
      &s_has_related_object,
      &p_resource_id,
      &("urn:uuid:".to_owned() + &self.id),
      None,
      None,
      false,
    );
    if let Some(ref url) = self.url {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_locator,
        &url,
        None,
        None,
        false,
      );
    }
    self.add_link(
      graph,
      &s_has_related_object,
      &p_has_creator,
      &self.created_via,
      None,
      None,
      false,
    );

    if let Some(ref created_at) = self.created_at {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_date_created,
        &created_at,
        None,
        Some(XML_NAMESPACE.to_owned() + "dateTime"),
        false,
      );
    }
    if let Some(ref updated_at) = self.updated_at {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_date_modified,
        &updated_at,
        None,
        Some(XML_NAMESPACE.to_owned() + "dateTime"),
        false,
      );
    }
    if let Some(ref filesize_bytes) = self.filesize_bytes {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_file_size,
        &format!("{}", filesize_bytes),
        None,
        Some(XML_NAMESPACE.to_owned() + "unsignedLong"),
        false,
      );
    }
    self.add_link(
      graph,
      &s_has_related_object,
      &p_has_format,
      &format!("urn:mimetype:{}", self.format.mime_type),
      None,
      None,
      false,
    );

    if let Some(ref height) = self.height {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_height,
        &format!("{}", height),
        None,
        Some(XML_NAMESPACE.to_owned() + "integer"),
        false,
      );
      self.add_link(
        graph,
        &s_has_related_object,
        &p_height_unit,
        "pixel",
        None,
        None,
        false,
      );
    }
    if let Some(ref width) = self.width {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_width,
        &format!("{}", width),
        None,
        Some(XML_NAMESPACE.to_owned() + "integer"),
        false,
      );
      self.add_link(
        graph,
        &s_has_related_object,
        &p_width_unit,
        "pixel",
        None,
        None,
        false,
      );
    }

    if self.filename.is_some() {
      if self.path.is_some() {
        self.add_link(
          graph,
          &s_has_related_object,
          &p_filename,
          &format!(
            "{}{}",
            self.path.clone().unwrap(),
            self.filename.clone().unwrap()
          ),
          None,
          None,
          false,
        );
      } else {
        self.add_link(
          graph,
          &s_has_related_object,
          &p_filename,
          &self.filename.clone().unwrap(),
          None,
          None,
          false,
        );
      }
    }
    self.add_link(
      graph,
      &s_has_related_object,
      &p_storage_id,
      &self.storage,
      None,
      None,
      false,
    );

    if let Some(ref md5_checksum) = self.md5_checksum {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_hash_value,
        &("urn:md5:".to_owned() + &md5_checksum),
        None,
        None,
        false,
      );
    }

    if let Some(ref bitrate_kbps) = self.bitrate_kbps {
      self.add_link(
        graph,
        &s_has_related_object,
        &p_hash_value,
        &format!("{}", bitrate_kbps * 1000),
        None,
        Some(XML_NAMESPACE.to_owned() + "nonNegativeInteger"),
        false,
      );
    }

    if let Some(ref lang) = self.lang {
      let s_has_language = self.add_related_node(graph, &s_has_related_object, &p_has_language);
      self.add_link(
        graph,
        &s_has_language,
        &p_type,
        &o_language,
        None,
        None,
        true,
      );
      self.add_link(graph, &s_has_language, &p_label, &lang, None, None, false);
    }

    let s_is_issued_by = self.add_related_node(graph, &s_has_related_object, &p_is_issued_by);
    self.add_link(
      graph,
      &s_is_issued_by,
      &p_type,
      &o_organisation,
      None,
      None,
      true,
    );
    self.add_link(
      graph,
      &s_is_issued_by,
      &p_organisation_name,
      &self.created_via,
      None,
      None,
      false,
    );

    for tag in &self.tags {
      let s_has_topic = self.add_related_node(graph, &s_has_related_object, &p_has_topic);
      self.add_link(graph, &s_has_topic, &p_type, &o_tag, None, None, true);
      self.add_link(graph, &s_has_topic, &p_pref_label, &tag, None, None, false);
      self.add_link(graph, &s_has_topic, &p_definition, "Tag", None, None, true);
    }
  }
}

impl Resource {
  fn add_link(
    &self,
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
