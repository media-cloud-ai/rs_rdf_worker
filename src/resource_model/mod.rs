use crate::convert::ToRdf;
use crate::namespaces::*;
use crate::rdf_graph::{add_link, add_related_node};
use rdf::{
  error::{Error, ErrorType},
  graph::Graph,
  uri::Uri,
};

#[derive(Debug, Default, Deserialize)]
pub struct Resources {
  #[serde(default)]
  pub items: Vec<Resource>,
}

impl ToRdf for Resources {
  fn to_rdf(&self, graph: &mut Graph) -> Result<(), Error> {
    for item in &self.items {
      item.to_rdf(graph)?;
    }
    Ok(())
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
  fn to_rdf(&self, graph: &mut Graph) -> Result<(), Error> {
    let video_id = self.external_ids.video_id.clone().ok_or_else(|| {
      Error::new(
        ErrorType::InvalidWriterOutput,
        "Expected a video id in external ids to convert resource to RDF".to_string(),
      )
    })?;
    let s_root = format!("http://resources.idfrancetv.fr/medias/{}", video_id);
    let root_node = graph.create_uri_node(&Uri::new(s_root));

    let p_has_creator = format!("{}hasCreator", EBUCORE_NAMESPACE);
    let p_date_created = format!("{}dateCreated", EBUCORE_NAMESPACE);
    let p_date_modified = format!("{}dateModified", EBUCORE_NAMESPACE);
    let p_file_size = format!("{}fileSize", EBUCORE_NAMESPACE);
    let p_filename = format!("{}filename", EBUCORE_NAMESPACE);
    let p_has_format = format!("{}hasFormat", EBUCORE_NAMESPACE);
    let p_has_language = format!("{}hasLanguage", EBUCORE_NAMESPACE);
    let p_has_related_image = format!("{}hasRelatedImage", EBUCORE_NAMESPACE);
    let p_has_related_resource = format!("{}hasRelatedResource", EBUCORE_NAMESPACE);
    let p_has_topic = format!("{}hasTopic", EBUCORE_NAMESPACE);
    let p_hash_value = format!("{}hashValue", EBUCORE_NAMESPACE);
    let p_height = format!("{}height", EBUCORE_NAMESPACE);
    let p_height_unit = format!("{}heightUnit", EBUCORE_NAMESPACE);
    let p_is_issued_by = format!("{}isIssuedby", EBUCORE_NAMESPACE);
    let p_locator = format!("{}locator", EBUCORE_NAMESPACE);
    let p_organisation_name = format!("{}organisationName", EBUCORE_NAMESPACE);
    let p_resource_id = format!("{}resourceId", EBUCORE_NAMESPACE);
    let p_storage_id = format!("{}storageId", EBUCORE_NAMESPACE);
    let p_width = format!("{}width", EBUCORE_NAMESPACE);
    let p_width_unit = format!("{}widthUnit", EBUCORE_NAMESPACE);

    let p_type = format!("{}type", RDF_NAMESPACE);

    let p_label = format!("{}label", SKOS_NAMESPACE);
    let p_pref_label = format!("{}prefLabel", SKOS_NAMESPACE);
    let p_definition = format!("{}definition", SKOS_NAMESPACE);

    let o_language = format!("{}Language", EBUCORE_NAMESPACE);
    let o_organisation = format!("{}Organisation", EBUCORE_NAMESPACE);
    let o_media_resource = format!("{}MediaResource", EBUCORE_NAMESPACE);
    let o_picture = format!("{}Picture", EBUCORE_NAMESPACE);
    let o_tag = format!("{}Tag", EBUCORE_NAMESPACE);

    let s_has_related_object = match self.format.mime_type.as_str() {
      "image/jpeg" => {
        let node = add_related_node(graph, &root_node, &p_has_related_image);
        add_link(graph, &node, &p_type, &o_picture, None, None, true);

        node
      }
      _ => {
        let node = add_related_node(graph, &root_node, &p_has_related_resource);
        add_link(graph, &node, &p_type, &o_media_resource, None, None, true);
        node
      }
    };

    add_link(
      graph,
      &s_has_related_object,
      &p_resource_id,
      &format!("urn:uuid:{}", self.id),
      None,
      None,
      false,
    );
    if let Some(ref url) = self.url {
      add_link(
        graph,
        &s_has_related_object,
        &p_locator,
        &url,
        None,
        None,
        false,
      );
    }
    add_link(
      graph,
      &s_has_related_object,
      &p_has_creator,
      &self.created_via,
      None,
      None,
      false,
    );

    if let Some(ref created_at) = self.created_at {
      add_link(
        graph,
        &s_has_related_object,
        &p_date_created,
        &created_at,
        None,
        Some(format!("{}dateTime", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(ref updated_at) = self.updated_at {
      add_link(
        graph,
        &s_has_related_object,
        &p_date_modified,
        &updated_at,
        None,
        Some(format!("{}dateTime", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(ref filesize_bytes) = self.filesize_bytes {
      add_link(
        graph,
        &s_has_related_object,
        &p_file_size,
        &format!("{}", filesize_bytes),
        None,
        Some(format!("{}unsignedLong", XML_NAMESPACE)),
        false,
      );
    }
    add_link(
      graph,
      &s_has_related_object,
      &p_has_format,
      &format!("urn:mimetype:{}", self.format.mime_type),
      None,
      None,
      false,
    );

    if let Some(ref height) = self.height {
      add_link(
        graph,
        &s_has_related_object,
        &p_height,
        &format!("{}", height),
        None,
        Some(format!("{}integer", XML_NAMESPACE)),
        false,
      );
      add_link(
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
      add_link(
        graph,
        &s_has_related_object,
        &p_width,
        &format!("{}", width),
        None,
        Some(format!("{}integer", XML_NAMESPACE)),
        false,
      );
      add_link(
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
        add_link(
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
        add_link(
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
    add_link(
      graph,
      &s_has_related_object,
      &p_storage_id,
      &self.storage,
      None,
      None,
      false,
    );

    if let Some(ref md5_checksum) = self.md5_checksum {
      add_link(
        graph,
        &s_has_related_object,
        &p_hash_value,
        &format!("urn:md5:{}", md5_checksum),
        None,
        None,
        false,
      );
    }

    if let Some(ref bitrate_kbps) = self.bitrate_kbps {
      add_link(
        graph,
        &s_has_related_object,
        &p_hash_value,
        &format!("{}", bitrate_kbps * 1000),
        None,
        Some(format!("{}nonNegativeInteger", XML_NAMESPACE)),
        false,
      );
    }

    if let Some(ref lang) = self.lang {
      let s_has_language = add_related_node(graph, &s_has_related_object, &p_has_language);
      add_link(
        graph,
        &s_has_language,
        &p_type,
        &o_language,
        None,
        None,
        true,
      );
      add_link(graph, &s_has_language, &p_label, &lang, None, None, false);
    }

    let s_is_issued_by = add_related_node(graph, &s_has_related_object, &p_is_issued_by);
    add_link(
      graph,
      &s_is_issued_by,
      &p_type,
      &o_organisation,
      None,
      None,
      true,
    );
    add_link(
      graph,
      &s_is_issued_by,
      &p_organisation_name,
      &self.created_via,
      None,
      None,
      false,
    );

    for tag in &self.tags {
      let s_has_topic = add_related_node(graph, &s_has_related_object, &p_has_topic);
      add_link(graph, &s_has_topic, &p_type, &o_tag, None, None, true);
      add_link(graph, &s_has_topic, &p_pref_label, &tag, None, None, false);
      add_link(graph, &s_has_topic, &p_definition, "Tag", None, None, true);
    }
    Ok(())
  }
}
