
use model::audio_track::AudioTrack;
use model::kind::Kind;
use model::tag::Tag;
use model::text_track::TextTrack;
use model::rating::Rating;
use model::country::Country;
use model::platforms::Platforms;
use model::part::Part;
use model::image::Image;
use model::group::Group;
use model::people::People;
use model::channel::Channel;
use model::category::Category;
use namespaces::*;

use rdf::graph::Graph;
use rdf::node::Node;
use rdf::triple::Triple;
use rdf::uri::Uri;

#[derive(Debug, Deserialize)]
pub struct Metadata {
  created_at: String,
  created_by: Option<String>, // MISSING
  created_via: Option<String>, // MISSING
  id: String,
  updated_at: String,
  additional_title: Option<String>,
  allocine_press_stars: Option<f32>, // MISSING
  #[serde(default)]
  audio_tracks: Vec<AudioTrack>,
  broadcasted_at: Option<String>,
  broadcasted_live: Option<bool>,
  category: Category,
  channel: Option<Channel>,
  copyright: Option<String>, // MISSING
  credits: Vec<People>,
  description: Option<String>,
  drm: bool, // MISSING
  duration: Option<String>,
  embeddable: bool, // MISSING
  episode_number: Option<u32>,
  expected_at: Option<String>,
  expected_duration: Option<String>, // MISSING
  ftvcut_id: Option<String>,
  #[serde(default)]
  groups: Vec<Group>,
  #[serde(default)]
  images: Vec<Image>,
  licensing: Option<String>, // MISSING
  midrollable: bool,
  original_title: Option<String>,
  oscar_id: Option<String>,
  parent_id: Option<String>,
  #[serde(default)]
  parts: Vec<Part>, // NOT YET ?
  platforms: Platforms, // MISSING
  plurimedia_broadcast_id: Option<u32>,
  #[serde(default)]
  plurimedia_collection_ids: Vec<u32>, // MISSING
  plurimedia_program_id: Option<u32>,
  previously_broadcasted: bool,
  previously_broadcasted_on_this_channel: bool,
  produced_at: Option<u32>, // integer ??? specification is string // MISSING
  #[serde(default)]
  production_countries: Vec<Country>, // MISSING
  rating: Option<Rating>,
  #[serde(default)]
  restricted_countries: Option<Vec<Country>>, // MISSING
  season_number: Option<u32>,
  short_description: Option<String>,
  #[serde(default)]
  tags: Vec<Tag>,
  #[serde(default)]
  text_tracks: Vec<TextTrack>,
  title: String,
  token: bool, // MISSING
  #[serde(rename="type")]
  kind: Kind // MISSING
}

impl Metadata {
  pub fn to_rdf(&self, graph: &mut Graph) {
    let s_root = "http://ressources.idfrancetv.fr/medias/".to_string() + &self.id;
    let p_ftv_audio = "http://ressources.idfrancetv.fr/ontologies/audio#";
    let p_ftv_sous_titre = "http://ressources.idfrancetv.fr/ontologies/sous-titre#";
    let p_ftv_csa = "http://ressources.idfrancetv.fr/ontologies/csa#";
    let p_title = EBUCORE_NAMESPACE.to_owned() + "title";
    let p_alternative_title = EBUCORE_NAMESPACE.to_owned() + "alternativeTitle";
    let p_abstract = EBUCORE_NAMESPACE.to_owned() + "abstract";
    let p_synopsis = EBUCORE_NAMESPACE.to_owned() + "synopsis";
    let p_label = RDF_NAMESPACE.to_owned() + "label";
    let p_id = RDF_NAMESPACE.to_owned() + "id";
    let p_type = RDF_NAMESPACE.to_owned() + "type";
    let p_duration = RDF_NAMESPACE.to_owned() + "durationNormalPlayTime";
    let p_date_created = EBUCORE_NAMESPACE.to_owned() + "dateCreated";
    let p_date_modified = EBUCORE_NAMESPACE.to_owned() + "dateModified";
    let p_references = EBUCORE_NAMESPACE.to_owned() + "References";
    let p_resource_id = EBUCORE_NAMESPACE.to_owned() + "resourceId";
    let p_has_contributor = EBUCORE_NAMESPACE.to_owned() + "hasContributor";
    let p_has_topic = EBUCORE_NAMESPACE.to_owned() + "hasTopic";
    let p_has_genre = EBUCORE_NAMESPACE.to_owned() + "hasGenre";
    let p_has_publication_event = EBUCORE_NAMESPACE.to_owned() + "hasPublicationEvent";
    let p_has_related_image = EBUCORE_NAMESPACE.to_owned() + "hasRelatedImage";
    let p_has_related_audio_programme = EBUCORE_NAMESPACE.to_owned() + "hasRelatedAudioProgramme";
    let p_has_related_text_line = EBUCORE_NAMESPACE.to_owned() + "hasRelatedTextLine";
    let p_has_part = EBUCORE_NAMESPACE.to_owned() + "hasPart";
    let p_has_target_audience = EBUCORE_NAMESPACE.to_owned() + "hasTargetAudience";
    let p_first_showing = EBUCORE_NAMESPACE.to_owned() + "firstShowing";
    let p_first_showing_this_service = EBUCORE_NAMESPACE.to_owned() + "firstShowingThisService";
    let p_audio_programme = EBUCORE_NAMESPACE.to_owned() + "audioProgramme";
    let p_text_line = EBUCORE_NAMESPACE.to_owned() + "TextLine";
    let p_original_title = EBUCORE_NAMESPACE.to_owned() + "originalTitle";
    let p_publication_channel = EBUCORE_NAMESPACE.to_owned() + "publicationChannel";
    let p_publication_channel_id = EBUCORE_NAMESPACE.to_owned() + "publicationChannelId";
    let p_publication_channel_name = EBUCORE_NAMESPACE.to_owned() + "publicationChannelName";
    let p_date_broadcast = EBUCORE_NAMESPACE.to_owned() + "dateBroadcast";
    let p_publication_start_date_time = EBUCORE_NAMESPACE.to_owned() + "publicationStartDateTime";
    let p_duration_normal_play_time = EBUCORE_NAMESPACE.to_owned() + "durationNormalPlayTime";
    let p_live = EBUCORE_NAMESPACE.to_owned() + "live";
    let p_is_agent = EBUCORE_NAMESPACE.to_owned() + "isAgent";
    let p_is_character = EBUCORE_NAMESPACE.to_owned() + "isCharacter";
    let p_given_name = EBUCORE_NAMESPACE.to_owned() + "givenName";
    let p_family_name = EBUCORE_NAMESPACE.to_owned() + "familyName";
    let p_character_name = EBUCORE_NAMESPACE.to_owned() + "characterName";
    let p_episode_number = EBUCORE_NAMESPACE.to_owned() + "episodeNumber";
    let p_is_member_of = EBUCORE_NAMESPACE.to_owned() + "isMemberOf";
    let p_group_id = EBUCORE_NAMESPACE.to_owned() + "groupId";
    let p_group_name = EBUCORE_NAMESPACE.to_owned() + "groupName";
    let p_group_description = EBUCORE_NAMESPACE.to_owned() + "groupDescription";
    let p_season_number = EBUCORE_NAMESPACE.to_owned() + "seasonNumber";
    let p_has_season = EBUCORE_NAMESPACE.to_owned() + "hasSeason";
    let p_mid_roll_ad_allowed = EBUCORE_NAMESPACE.to_owned() + "midRollAdAllowed";
    let p_file_size = EBUCORE_NAMESPACE.to_owned() + "fileSize";
    let p_has_format = EBUCORE_NAMESPACE.to_owned() + "hasFormat";
    let p_height = EBUCORE_NAMESPACE.to_owned() + "height";
    let p_width = EBUCORE_NAMESPACE.to_owned() + "width";
    let p_hash_value = EBUCORE_NAMESPACE.to_owned() + "hashValue";
    let p_locator = EBUCORE_NAMESPACE.to_owned() + "locator";

    let o_editorial_object = EBUCORE_NAMESPACE.to_owned() + "EditorialObject";
    let o_publication_event = EBUCORE_NAMESPACE.to_owned() + "PublicationEvent";
    let o_person = EBUCORE_NAMESPACE.to_owned() + "Person";
    let o_character = EBUCORE_NAMESPACE.to_owned() + "Character";
    let o_group = EBUCORE_NAMESPACE.to_owned() + "Group";
    let o_season = EBUCORE_NAMESPACE.to_owned() + "Season";
    let o_image = EBUCORE_NAMESPACE.to_owned() + "Image";
    let o_part = EBUCORE_NAMESPACE.to_owned() + "Part";

    let subject = self.add_triple(graph, &s_root, &p_type, &o_editorial_object);

    self.add_link(graph, &subject, &p_title, &self.title, Some("fr"), None, false);

    if let Some(ref parent_id) = self.parent_id {
      let s_references = self.add_related_node(graph, &subject, &p_references);
      self.add_link(graph, &s_references, &p_type, &o_editorial_object, None, None, false);
      self.add_link(graph, &s_references, &p_resource_id, &("http://ressources.idfrancetv.fr/medias/".to_string() + &parent_id), None, None, true);
    }

    if let Some(ref original_title) = self.original_title {
      self.add_link(graph, &subject, &p_original_title, &original_title, Some("fr"), None, false);
    }

    if let Some(ref additional_title) = self.additional_title {
      self.add_link(graph, &subject, &p_alternative_title, &additional_title, Some("fr"), None, false);
    }

    if let Some(ref description) = self.description {
      self.add_link(graph, &subject, &p_synopsis, &description, Some("fr"), None, false);
    }

    if let Some(ref short_description) = self.short_description {
      self.add_link(graph, &subject, &p_abstract, &short_description, Some("fr"), None, false);
    }

    if let Some(ref duration) = self.duration {
      self.add_link(graph, &subject, &p_duration, &duration, None, Some(XML_NAMESPACE.to_owned() + "duration"), false);
    }

    self.add_link(graph, &subject, &p_date_created, &self.created_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);
    self.add_link(graph, &subject, &p_date_modified, &self.updated_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);

    //live
    if let Some(broadcasted_live) = self.broadcasted_live {
      self.add_link(graph, &subject, &p_live, &broadcasted_live.to_string(), None, Some(XML_NAMESPACE.to_owned() + "boolean"), false);
    }

    // identifiers
    self.insert_identifier(graph, &subject, "SIVideo", &("urn::uuid:".to_owned() + &self.id));
    if let Some(ref oscar_id) = self.oscar_id {
      self.insert_identifier(graph, &subject, "OSCAR", &oscar_id);
    }

    if let Some(ref plurimedia_broadcast_id) = self.plurimedia_broadcast_id {
      self.insert_identifier(graph, &subject, "PLURIMEDIA_BROADCAST", &plurimedia_broadcast_id.to_string());
    }

    if let Some(ref plurimedia_program_id) = self.plurimedia_program_id {
      self.insert_identifier(graph, &subject, "PLURIMEDIA_PROGRAM", &plurimedia_program_id.to_string());
    }

    if let Some(ref ftvcut_id) = self.ftvcut_id {
      self.insert_identifier(graph, &subject, "FTVCUT", &ftvcut_id);
    }

    // episode
    if let Some(ref episode_number) = self.episode_number {
      self.add_link(graph, &subject, &p_episode_number, &episode_number.to_string(), None, None, false);
    }

    for group in &self.groups {
      let s_group = self.add_related_node(graph, &subject, &p_is_member_of);
      self.add_link(graph, &s_group, &p_type, &o_group, None, None, true);
      self.add_link(graph, &s_group, &p_group_id, &group.id, None, None, false);
      self.add_link(graph, &s_group, &p_group_name, &group.label, None, None, false);
      if let Some(ref description) = group.description {
        self.add_link(graph, &s_group, &p_group_description, &description, None, None, false);
      }
      if let Some(ref season_number) = group.season_number {
        let s_has_season = self.add_related_node(graph, &s_group, &p_has_season);
        self.add_link(graph, &s_has_season, &p_type, &o_season, None, None, true);
        self.add_link(graph, &s_has_season, &p_season_number, &format!("{}", season_number), None, None, false);
      }
    }

    // audio_tracks
    for audio_track in &self.audio_tracks {
      let s_has_related_audio_programme = self.add_related_node(graph, &subject, &p_has_related_audio_programme);
      let s_audio_programme = self.add_related_node(graph, &s_has_related_audio_programme, &p_audio_programme);
      self.add_link(graph, &s_audio_programme, &p_type, &(p_ftv_audio.to_owned() + "complet_2.0_" + &audio_track.id), None, None, true);
    }

    // text_tracks
    for text_track in &self.text_tracks {
      let s_has_related_text_line = self.add_related_node(graph, &subject, &p_has_related_text_line);
      let s_text_line = self.add_related_node(graph, &s_has_related_text_line, &p_text_line);
      self.add_link(graph, &s_text_line, &p_type, &(p_ftv_sous_titre.to_owned() + &text_track.id), None, None, true);
    }

    // publication event
    let s_publication_event = self.add_related_node(graph, &subject, &p_has_publication_event);
    self.add_link(graph, &s_publication_event, &p_type, &o_publication_event, None, None, false);
    self.add_link(graph, &s_publication_event, &p_mid_roll_ad_allowed, &format!("{}", self.midrollable), None, Some(XML_NAMESPACE.to_owned() + "boolean"), false);
    self.add_link(graph, &s_publication_event, &p_first_showing, &format!("{}", self.previously_broadcasted), None, Some(XML_NAMESPACE.to_owned() + "boolean"), false);
    self.add_link(graph, &s_publication_event, &p_first_showing_this_service, &format!("{}", self.previously_broadcasted_on_this_channel), None, Some(XML_NAMESPACE.to_owned() + "boolean"), false);
    
    if let Some(ref rating) = self.rating {
      self.add_link(graph, &s_publication_event, &p_has_target_audience, &(p_ftv_csa.to_owned() + &rating.id), None, None, true);
    }

    if let Some(ref channel) = self.channel {
      let s_publication_channel = self.add_related_node(graph, &s_publication_event, &p_publication_channel);
      self.add_link(graph, &s_publication_channel, &p_publication_channel_id, &channel.id, None, None, false);
      self.add_link(graph, &s_publication_channel, &p_publication_channel_name, &channel.label, None, None, false);
    }

    if let Some(ref broadcasted_at) = self.broadcasted_at {
      self.add_link(graph, &s_publication_event, &p_date_broadcast, &broadcasted_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);
    }
    if let Some(ref expected_at) = self.expected_at {
      self.add_link(graph, &s_publication_event, &p_publication_start_date_time, &expected_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);
    }
    if let Some(ref duration) = self.duration {
      self.add_link(graph, &s_publication_event, &p_duration_normal_play_time, &duration, None, Some(XML_NAMESPACE.to_owned() + "duration"), false);
    }

    // category
    let s_category = self.add_related_node(graph, &subject, &p_has_genre);
    self.add_link(graph, &s_category, &p_type, &(FTV_GENRE_NAMESPACE.to_string() + &self.category.id), None, None, true);

    // topics
    for tag in &self.tags {
      let s_has_topic = self.add_related_node(graph, &subject, &p_has_topic);
      self.add_link(graph, &s_has_topic, &p_type, &(FTV_TAG_NAMESPACE.to_string() + "Tag"), None, None, true);
      self.add_link(graph, &s_has_topic, &p_id, &tag.id, None, None, false);
      self.add_link(graph, &s_has_topic, &p_label, &tag.label, None, None, false);
    }

    // credits
    for people in &self.credits {
      let s_has_contributor = self.add_related_node(graph, &subject, &p_has_contributor);
      self.add_link(graph, &s_has_contributor, &p_type, &(FTV_ROLE_NAMESPACE.to_string() + &people.role.id), None, None, true);

      let s_is_agent = self.add_related_node(graph, &subject, &p_is_agent);
      self.add_link(graph, &s_is_agent, &p_type, &o_person, None, None, true);
      if let Some(ref first_name) = people.first_name {
        self.add_link(graph, &s_is_agent, &p_given_name, &first_name, None, None, false);
      }
      self.add_link(graph, &s_is_agent, &p_family_name, &people.last_name, None, None, false);

      if let Some(ref character) = people.character {
        let s_is_character = self.add_related_node(graph, &subject, &p_is_character);
        self.add_link(graph, &s_is_character, &p_type, &o_character, None, None, true);
        self.add_link(graph, &s_is_character, &p_character_name, &character, None, None, false);
      }
    }

    // images
    for image in &self.images {
      if image.format.mime_type != "image/jpeg" {
        continue;
      }

      let s_has_related_image = self.add_related_node(graph, &subject, &p_has_related_image);
      self.add_link(graph, &s_has_related_image, &p_type, &o_image, None, None, true);
      self.add_link(graph, &s_has_related_image, &p_resource_id, &("urn::uuid:".to_owned() + &image.id), None, None, false);

      if let Some(ref filesize_bytes) = image.filesize_bytes {
        self.add_link(graph, &s_has_related_image, &p_file_size, &format!("{}", filesize_bytes), None, Some(XML_NAMESPACE.to_owned() + "unsignedLong"), false);
      }
      self.add_link(graph, &s_has_related_image, &p_has_format, &format!("urn:mimetype:{}", image.format.mime_type), None, None, false);
      self.add_link(graph, &s_has_related_image, &p_height, &format!("{}", image.height), None, Some(XML_NAMESPACE.to_owned() + "integer"), false);
      self.add_link(graph, &s_has_related_image, &p_width, &format!("{}", image.width), None, Some(XML_NAMESPACE.to_owned() + "integer"), false);
      if let Some(ref md5_checksum) = image.md5_checksum {
        self.add_link(graph, &s_has_related_image, &p_hash_value, &("urn::md5:".to_owned() + &md5_checksum), None, None, false);
      }
      self.add_link(graph, &s_has_related_image, &p_locator, &image.url, None, None, false);
      if let Some(ref updated_at) = image.updated_at {
        self.add_link(graph, &s_has_related_image, &p_date_modified, &updated_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);
      }
      if let Some(ref created_at) = image.created_at {
        self.add_link(graph, &s_has_related_image, &p_date_created, &created_at, None, Some(XML_NAMESPACE.to_owned() + "dateTime"), false);
      }
    }
  }

  fn insert_identifier(&self, graph: &mut Graph, subject_node: &Node, identifier_type: &str, value: &str) {
    let p_has_idenfitier = EBUCORE_NAMESPACE.to_owned() + "hasIdentifier";
    let p_idenfitier_value = EBUCORE_NAMESPACE.to_owned() + "identifierValue";
    let p_label = RDFS_NAMESPACE.to_owned() + "label";
    let p_type = RDF_NAMESPACE.to_owned() + "type";

    let o_identifier = EBUCORE_NAMESPACE.to_owned() + "Identifier";

    let s_identifier = self.add_related_node(graph, &subject_node, &p_has_idenfitier);
    self.add_link(graph, &s_identifier, &p_type, &o_identifier, None, None, false);
    self.add_link(graph, &s_identifier, &p_label, identifier_type, None, None, false);
    self.add_link(graph, &s_identifier, &p_idenfitier_value, value, None, None, false);
  }

  fn add_triple(&self, graph: &mut Graph, subject: &str, predicate: &str, object: &str) -> Node {
    let subject_node = graph.create_uri_node(&Uri::new(subject.to_string()));
    let predicate_node = graph.create_uri_node(&Uri::new(predicate.to_string()));
    let object_node = graph.create_uri_node(&Uri::new(object.to_string()));

    let triple = Triple::new(&subject_node, &predicate_node, &object_node);
    graph.add_triple(&triple);
    subject_node
  }

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
