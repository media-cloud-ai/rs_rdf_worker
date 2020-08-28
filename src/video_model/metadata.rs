use crate::convert::ToRdf;
use crate::namespaces::*;
use crate::video_model::{
  audio_track::AudioTrack, category::Category, channel::Channel, country::Country, group::Group,
  image::Image, kind::Kind, part::Part, people::People, platforms::Platforms, rating::Rating,
  tag::Tag, text_track::TextTrack,
};
use rdf::{error::Error, graph::Graph};

use crate::rdf_graph::{add_link, add_related_node, add_triple, insert_identifier};
use crate::resource_model::Resources;

#[derive(Debug, Deserialize)]
pub struct Metadata {
  created_at: String,
  created_by: Option<String>,
  created_via: Option<String>, // MISSING
  id: String,
  updated_at: String,
  additional_title: Option<String>,
  allocine_press_stars: Option<f32>, // MISSING
  #[serde(default)]
  audio_tracks: Vec<AudioTrack>,
  broadcasted_at: Option<String>,
  broadcasted_live: Option<bool>,
  category: Option<Category>,
  channel: Option<Channel>,
  copyright: Option<String>,
  credits: Vec<People>,
  description: Option<String>,
  drm: bool,
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
  platforms: Option<Platforms>, // MISSING
  plurimedia_broadcast_id: Option<u32>,
  #[serde(default)]
  plurimedia_collection_ids: Vec<u32>,
  plurimedia_program_id: Option<u32>,
  previously_broadcasted: Option<bool>,
  previously_broadcasted_on_this_channel: Option<bool>,
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
  #[serde(rename = "type")]
  kind: Kind,

  #[serde(skip)]
  pub resources: Resources,
}

impl ToRdf for Metadata {
  fn to_rdf(&self, graph: &mut Graph) -> Result<(), Error> {
    let s_root = format!("http://resources.idfrancetv.fr/medias/{}", self.id);
    let p_ftv_audio = "http://resources.idfrancetv.fr/ontologies/audio#";
    let p_ftv_sous_titre = "http://resources.idfrancetv.fr/ontologies/sous-titre#";
    let p_ftv_csa = "http://resources.idfrancetv.fr/ontologies/csa#";

    let p_alternative_title = format!("{}alternativeTitle", EBUCORE_NAMESPACE);
    let p_abstract = format!("{}abstract", EBUCORE_NAMESPACE);
    let p_character_name = format!("{}characterName", EBUCORE_NAMESPACE);
    let p_date_created = format!("{}dateCreated", EBUCORE_NAMESPACE);
    let p_date_modified = format!("{}dateModified", EBUCORE_NAMESPACE);
    let p_date_broadcast = format!("{}dateBroadcast", EBUCORE_NAMESPACE);
    let p_duration = format!("{}durationNormalPlayTime", EBUCORE_NAMESPACE);
    let p_duration_normal_play_time = format!("{}durationNormalPlayTime", EBUCORE_NAMESPACE);
    let p_episode_number = format!("{}episodeNumber", EBUCORE_NAMESPACE);
    let p_family_name = format!("{}familyName", EBUCORE_NAMESPACE);
    let p_first_showing = format!("{}firstShowing", EBUCORE_NAMESPACE);
    let p_first_showing_this_service = format!("{}firstShowingThisService", EBUCORE_NAMESPACE);
    let p_free = format!("{}free", EBUCORE_NAMESPACE);
    let p_given_name = format!("{}givenName", EBUCORE_NAMESPACE);
    let p_group_description = format!("{}groupDescription", EBUCORE_NAMESPACE);
    let p_group_id = format!("{}groupId", EBUCORE_NAMESPACE);
    let p_group_name = format!("{}groupName", EBUCORE_NAMESPACE);
    let p_has_audio_programme_type = format!("{}hasAudioProgrammeType", EBUCORE_NAMESPACE);
    let p_has_contributor = format!("{}hasContributor", EBUCORE_NAMESPACE);
    let p_has_creator = format!("{}hasCreator", EBUCORE_NAMESPACE);
    let p_has_editorial_object_type = format!("{}hasEditorialObjectType", EBUCORE_NAMESPACE);
    let p_has_genre = format!("{}hasGenre", EBUCORE_NAMESPACE);
    let p_has_owner = format!("{}hasOwner", EBUCORE_NAMESPACE);
    let p_has_publication_event = format!("{}hasPublicationEvent", EBUCORE_NAMESPACE);
    let p_has_publication_event_type = format!("{}hasPublicationEventType", EBUCORE_NAMESPACE);
    let p_has_related_audio_programme = format!("{}hasRelatedAudioProgramme", EBUCORE_NAMESPACE);
    let p_has_related_text_line = format!("{}hasRelatedTextLine", EBUCORE_NAMESPACE);
    let p_has_role = format!("{}hasRole", EBUCORE_NAMESPACE);
    let p_has_season = format!("{}hasSeason", EBUCORE_NAMESPACE);
    let p_has_target_audience = format!("{}hasTargetAudience", EBUCORE_NAMESPACE);
    let p_has_text_line_type = format!("{}hasTextLineType", EBUCORE_NAMESPACE);
    let p_has_topic = format!("{}hasTopic", EBUCORE_NAMESPACE);
    let p_is_agent = format!("{}isAgent", EBUCORE_NAMESPACE);
    let p_is_character = format!("{}isCharacter", EBUCORE_NAMESPACE);
    let p_is_member_of = format!("{}isMemberOf", EBUCORE_NAMESPACE);
    let p_live = format!("{}live", EBUCORE_NAMESPACE);
    let p_mid_roll_ad_allowed = format!("{}midRollAdAllowed", EBUCORE_NAMESPACE);
    let p_organisation_name = format!("{}organisationName", EBUCORE_NAMESPACE);
    let p_original_title = format!("{}originalTitle", EBUCORE_NAMESPACE);
    let p_publication_channel = format!("{}publicationChannel", EBUCORE_NAMESPACE);
    let p_publication_channel_id = format!("{}publicationChannelId", EBUCORE_NAMESPACE);
    let p_publication_channel_name = format!("{}publicationChannelName", EBUCORE_NAMESPACE);
    let p_publication_start_date_time = format!("{}publicationStartDateTime", EBUCORE_NAMESPACE);
    let p_references = format!("{}references", EBUCORE_NAMESPACE);
    let p_resource_id = format!("{}resourceId", EBUCORE_NAMESPACE);
    let p_season_number = format!("{}seasonNumber", EBUCORE_NAMESPACE);
    let p_synopsis = format!("{}synopsis", EBUCORE_NAMESPACE);
    let p_title = format!("{}title", EBUCORE_NAMESPACE);

    let p_type = format!("{}type", RDF_NAMESPACE);

    let p_pref_label = format!("{}prefLabel", SKOS_NAMESPACE);
    let p_definition = format!("{}definition", SKOS_NAMESPACE);

    let o_audio_programme = format!("{}AudioProgramme", EBUCORE_NAMESPACE);
    let o_cast = format!("{}Cast", EBUCORE_NAMESPACE);
    let o_character = format!("{}Character", EBUCORE_NAMESPACE);
    let o_editorial_object = format!("{}EditorialObject", EBUCORE_NAMESPACE);
    let o_group = format!("{}Group", EBUCORE_NAMESPACE);
    let o_organisation = format!("{}Organisation", EBUCORE_NAMESPACE);
    let o_person = format!("{}Person", EBUCORE_NAMESPACE);
    let o_publication_channel = format!("{}PublicationChannel", EBUCORE_NAMESPACE);
    let o_publication_event = format!("{}PublicationEvent", EBUCORE_NAMESPACE);
    let o_season = format!("{}Season", EBUCORE_NAMESPACE);
    let o_tag = format!("{}Tag", EBUCORE_NAMESPACE);
    let o_text_line = format!("{}TextLine", EBUCORE_NAMESPACE);

    let subject = add_triple(graph, &s_root, &p_type, &o_editorial_object);

    add_link(
      graph,
      &subject,
      &p_title,
      &self.title,
      Some("fr"),
      None,
      false,
    );

    if let Some(ref parent_id) = self.parent_id {
      let s_references = add_related_node(graph, &subject, &p_references);
      add_link(
        graph,
        &s_references,
        &p_type,
        &o_editorial_object,
        None,
        None,
        true,
      );
      add_link(
        graph,
        &s_references,
        &p_resource_id,
        &format!("http://resources.idfrancetv.fr/medias/{}", parent_id),
        None,
        None,
        true,
      );
    }

    if let Some(ref original_title) = self.original_title {
      add_link(
        graph,
        &subject,
        &p_original_title,
        &original_title,
        Some("fr"),
        None,
        false,
      );
    }

    if let Some(ref additional_title) = self.additional_title {
      add_link(
        graph,
        &subject,
        &p_alternative_title,
        &additional_title,
        Some("fr"),
        None,
        false,
      );
    }

    if let Some(ref description) = self.description {
      add_link(
        graph,
        &subject,
        &p_synopsis,
        &description,
        Some("fr"),
        None,
        false,
      );
    }

    if let Some(ref short_description) = self.short_description {
      add_link(
        graph,
        &subject,
        &p_abstract,
        &short_description,
        Some("fr"),
        None,
        false,
      );
    }

    if let Some(ref duration) = self.duration {
      add_link(
        graph,
        &subject,
        &p_duration,
        &duration,
        None,
        Some(format!("{}duration", XML_NAMESPACE)),
        false,
      );
    }

    add_link(
      graph,
      &subject,
      &p_has_editorial_object_type,
      &format!(
        "http://resources.idfrancetv.fr/ontologies/editorial_object_type#{}",
        self.kind.id
      ),
      None,
      None,
      true,
    );

    add_link(
      graph,
      &subject,
      &p_date_created,
      &self.created_at,
      None,
      Some(format!("{}dateTime", XML_NAMESPACE)),
      false,
    );
    add_link(
      graph,
      &subject,
      &p_date_modified,
      &self.updated_at,
      None,
      Some(format!("{}dateTime", XML_NAMESPACE)),
      false,
    );

    // broadcasted at
    if let Some(ref broadcasted_at) = self.broadcasted_at {
      add_link(
        graph,
        &subject,
        &p_date_broadcast,
        &broadcasted_at,
        None,
        Some(format!("{}dateTime", XML_NAMESPACE)),
        false,
      );
    }

    // created by
    if let Some(ref created_by) = self.created_by {
      add_link(
        graph,
        &subject,
        &p_has_creator,
        &created_by,
        None,
        None,
        false,
      );
    }

    // live
    if let Some(broadcasted_live) = self.broadcasted_live {
      add_link(
        graph,
        &subject,
        &p_live,
        &broadcasted_live.to_string(),
        None,
        Some(format!("{}boolean", XML_NAMESPACE)),
        false,
      );
    }

    // copyright
    if let Some(ref copyright) = self.copyright {
      let s_has_owner = add_related_node(graph, &subject, &p_has_owner);
      add_link(
        graph,
        &s_has_owner,
        &p_type,
        &o_organisation,
        None,
        None,
        true,
      );
      add_link(
        graph,
        &s_has_owner,
        &p_organisation_name,
        &copyright,
        None,
        None,
        false,
      );
    }

    // identifiers
    insert_identifier(
      graph,
      &subject,
      "SIvideo",
      &format!("urn::uuid:{}", self.id),
    );
    if let Some(ref oscar_id) = self.oscar_id {
      insert_identifier(graph, &subject, "Oscar_ID", &oscar_id);
    }

    if let Some(ref plurimedia_broadcast_id) = self.plurimedia_broadcast_id {
      insert_identifier(
        graph,
        &subject,
        "Plurimedia_broadcast_id",
        &plurimedia_broadcast_id.to_string(),
      );
    }

    if let Some(ref plurimedia_program_id) = self.plurimedia_program_id {
      insert_identifier(
        graph,
        &subject,
        "Plurimedia_programme_id",
        &plurimedia_program_id.to_string(),
      );
    }

    for plurimedia_collection_id in &self.plurimedia_collection_ids {
      insert_identifier(
        graph,
        &subject,
        "Plurimedia_collection_ids",
        &plurimedia_collection_id.to_string(),
      );
    }

    if let Some(ref ftvcut_id) = self.ftvcut_id {
      insert_identifier(graph, &subject, "FTVCUT", &ftvcut_id);
    }

    // episode
    if let Some(ref episode_number) = self.episode_number {
      add_link(
        graph,
        &subject,
        &p_episode_number,
        &episode_number.to_string(),
        None,
        None,
        false,
      );
    }

    for group in &self.groups {
      let s_group = add_related_node(graph, &subject, &p_is_member_of);
      add_link(graph, &s_group, &p_type, &o_group, None, None, true);
      add_link(
        graph,
        &s_group,
        &p_group_id,
        &format!("urn:uuid:{}", group.id),
        None,
        None,
        false,
      );
      add_link(
        graph,
        &s_group,
        &p_group_name,
        &group.label,
        None,
        None,
        false,
      );
      if let Some(ref description) = group.description {
        add_link(
          graph,
          &s_group,
          &p_group_description,
          &description,
          None,
          None,
          false,
        );
      }
      if let Some(ref season_number) = group.season_number {
        let s_has_season = add_related_node(graph, &s_group, &p_has_season);
        add_link(graph, &s_has_season, &p_type, &o_season, None, None, true);
        add_link(
          graph,
          &s_has_season,
          &p_season_number,
          &format!("{}", season_number),
          None,
          None,
          false,
        );
      }
    }

    // audio_tracks
    for audio_track in &self.audio_tracks {
      let s_has_related_audio_programme =
        add_related_node(graph, &subject, &p_has_related_audio_programme);
      add_link(
        graph,
        &s_has_related_audio_programme,
        &p_type,
        &o_audio_programme,
        None,
        None,
        true,
      );
      add_link(
        graph,
        &s_has_related_audio_programme,
        &p_has_audio_programme_type,
        &format!("{}complet_2.0_{}", p_ftv_audio, audio_track.id),
        None,
        None,
        true,
      );
    }

    // text_tracks
    for text_track in &self.text_tracks {
      let s_has_related_text_line = add_related_node(graph, &subject, &p_has_related_text_line);
      add_link(
        graph,
        &s_has_related_text_line,
        &p_type,
        &o_text_line,
        None,
        None,
        true,
      );
      add_link(
        graph,
        &s_has_related_text_line,
        &p_has_text_line_type,
        &format!("{}{}", p_ftv_sous_titre, text_track.id),
        None,
        None,
        true,
      );
    }

    // publication event
    let s_publication_event = add_related_node(graph, &subject, &p_has_publication_event);
    add_link(
      graph,
      &s_publication_event,
      &p_type,
      &o_publication_event,
      None,
      None,
      true,
    );
    add_link(
      graph,
      &s_publication_event,
      &p_has_publication_event_type,
      "http://resources.idfrancetv.fr/ontologies/publication#diffusion",
      None,
      None,
      true,
    );
    if let Some(ref channel) = self.channel {
      let s_publication_channel =
        add_related_node(graph, &s_publication_event, &p_publication_channel);
      add_link(
        graph,
        &s_publication_channel,
        &p_type,
        &o_publication_channel,
        None,
        None,
        false,
      );
      add_link(
        graph,
        &s_publication_channel,
        &p_publication_channel_id,
        &channel.id,
        None,
        None,
        false,
      );
      add_link(
        graph,
        &s_publication_channel,
        &p_publication_channel_name,
        &channel.label,
        None,
        None,
        false,
      );
    }
    if let Some(ref expected_at) = self.expected_at {
      add_link(
        graph,
        &s_publication_event,
        &p_publication_start_date_time,
        &expected_at,
        None,
        Some(format!("{}dateTime", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(ref duration) = self.duration {
      add_link(
        graph,
        &s_publication_event,
        &p_duration_normal_play_time,
        &duration,
        None,
        Some(format!("{}duration", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(broadcasted_live) = self.broadcasted_live {
      add_link(
        graph,
        &s_publication_event,
        &p_live,
        &broadcasted_live.to_string(),
        None,
        Some(format!("{}boolean", XML_NAMESPACE)),
        false,
      );
    }
    add_link(
      graph,
      &s_publication_event,
      &p_free,
      &self.drm.to_string(),
      None,
      Some(format!("{}boolean", XML_NAMESPACE)),
      false,
    );
    if let Some(previously_broadcasted) = self.previously_broadcasted {
      add_link(
        graph,
        &s_publication_event,
        &p_first_showing,
        &previously_broadcasted.to_string(),
        None,
        Some(format!("{}boolean", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(previously_broadcasted_on_this_channel) =
      self.previously_broadcasted_on_this_channel
    {
      add_link(
        graph,
        &s_publication_event,
        &p_first_showing_this_service,
        &previously_broadcasted_on_this_channel.to_string(),
        None,
        Some(format!("{}boolean", XML_NAMESPACE)),
        false,
      );
    }
    if let Some(ref rating) = self.rating {
      add_link(
        graph,
        &s_publication_event,
        &p_has_target_audience,
        &format!("{}{}", p_ftv_csa, rating.id),
        None,
        None,
        true,
      );
    }
    add_link(
      graph,
      &s_publication_event,
      &p_mid_roll_ad_allowed,
      &format!("{}", self.midrollable),
      None,
      Some(format!("{}boolean", XML_NAMESPACE)),
      false,
    );
    if let Some(ref broadcasted_at) = self.broadcasted_at {
      add_link(
        graph,
        &s_publication_event,
        &p_date_broadcast,
        &broadcasted_at,
        None,
        Some(format!("{}dateTime", XML_NAMESPACE)),
        false,
      );
    }

    // category
    if let Some(ref category) = self.category {
      add_link(
        graph,
        &subject,
        &p_has_genre,
        &(format!("{}{}", FTV_GENRE_NAMESPACE, category.id)),
        None,
        None,
        true,
      );
    }

    // topics
    for tag in &self.tags {
      let s_has_topic = add_related_node(graph, &subject, &p_has_topic);
      add_link(graph, &s_has_topic, &p_type, &o_tag, None, None, true);
      add_link(
        graph,
        &s_has_topic,
        &p_pref_label,
        &tag.label,
        None,
        None,
        false,
      );
      add_link(graph, &s_has_topic, &p_definition, "Tag", None, None, true);
    }

    // credits
    for people in &self.credits {
      let s_has_contributor = add_related_node(graph, &subject, &p_has_contributor);
      add_link(
        graph,
        &s_has_contributor,
        &p_type,
        &o_cast,
        None,
        None,
        true,
      );
      add_link(
        graph,
        &s_has_contributor,
        &p_has_role,
        &(format!("{}{}", FTV_ROLE_NAMESPACE, people.role.id)),
        None,
        None,
        true,
      );

      let s_is_agent = add_related_node(graph, &s_has_contributor, &p_is_agent);
      add_link(graph, &s_is_agent, &p_type, &o_person, None, None, true);
      if let Some(ref first_name) = people.first_name {
        add_link(
          graph,
          &s_is_agent,
          &p_given_name,
          &first_name,
          None,
          None,
          false,
        );
      }
      add_link(
        graph,
        &s_is_agent,
        &p_family_name,
        &people.last_name,
        None,
        None,
        false,
      );

      if let Some(ref character) = people.character {
        let s_is_character = add_related_node(graph, &subject, &p_is_character);
        add_link(
          graph,
          &s_is_character,
          &p_type,
          &o_character,
          None,
          None,
          true,
        );
        add_link(
          graph,
          &s_is_character,
          &p_character_name,
          &character,
          None,
          None,
          false,
        );
      }
    }

    self.resources.to_rdf(graph)
  }
}
