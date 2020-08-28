use crate::convert::ToRdf;
use crate::namespaces::*;
use crate::video_model::audio_track::AudioTrack;
use crate::video_model::category::Category;
use crate::video_model::channel::Channel;
use crate::video_model::country::Country;
use crate::video_model::group::Group;
use crate::video_model::image::Image;
use crate::video_model::kind::Kind;
use crate::video_model::part::Part;
use crate::video_model::people::People;
use crate::video_model::platforms::Platforms;
use crate::video_model::rating::Rating;
use crate::video_model::tag::Tag;
use crate::video_model::text_track::TextTrack;
use rdf::graph::Graph;

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
  fn to_rdf(&self, graph: &mut Graph) {
    let s_root = "http://resources.idfrancetv.fr/medias/".to_string() + &self.id;
    let p_ftv_audio = "http://resources.idfrancetv.fr/ontologies/audio#";
    let p_ftv_sous_titre = "http://resources.idfrancetv.fr/ontologies/sous-titre#";
    let p_ftv_csa = "http://resources.idfrancetv.fr/ontologies/csa#";

    let p_alternative_title = EBUCORE_NAMESPACE.to_owned() + "alternativeTitle";
    let p_abstract = EBUCORE_NAMESPACE.to_owned() + "abstract";
    let p_character_name = EBUCORE_NAMESPACE.to_owned() + "characterName";
    let p_date_created = EBUCORE_NAMESPACE.to_owned() + "dateCreated";
    let p_date_modified = EBUCORE_NAMESPACE.to_owned() + "dateModified";
    let p_date_broadcast = EBUCORE_NAMESPACE.to_owned() + "dateBroadcast";
    let p_duration = EBUCORE_NAMESPACE.to_owned() + "durationNormalPlayTime";
    let p_duration_normal_play_time = EBUCORE_NAMESPACE.to_owned() + "durationNormalPlayTime";
    let p_episode_number = EBUCORE_NAMESPACE.to_owned() + "episodeNumber";
    let p_family_name = EBUCORE_NAMESPACE.to_owned() + "familyName";
    let p_first_showing = EBUCORE_NAMESPACE.to_owned() + "firstShowing";
    let p_first_showing_this_service = EBUCORE_NAMESPACE.to_owned() + "firstShowingThisService";
    let p_free = EBUCORE_NAMESPACE.to_owned() + "free";
    let p_given_name = EBUCORE_NAMESPACE.to_owned() + "givenName";
    let p_group_description = EBUCORE_NAMESPACE.to_owned() + "groupDescription";
    let p_group_id = EBUCORE_NAMESPACE.to_owned() + "groupId";
    let p_group_name = EBUCORE_NAMESPACE.to_owned() + "groupName";
    let p_has_audio_programme_type = EBUCORE_NAMESPACE.to_owned() + "hasAudioProgrammeType";
    let p_has_contributor = EBUCORE_NAMESPACE.to_owned() + "hasContributor";
    let p_has_creator = EBUCORE_NAMESPACE.to_owned() + "hasCreator";
    let p_has_editorial_object_type = EBUCORE_NAMESPACE.to_owned() + "hasEditorialObjectType";
    let p_has_genre = EBUCORE_NAMESPACE.to_owned() + "hasGenre";
    let p_has_owner = EBUCORE_NAMESPACE.to_owned() + "hasOwner";
    let p_has_publication_event = EBUCORE_NAMESPACE.to_owned() + "hasPublicationEvent";
    let p_has_publication_event_type = EBUCORE_NAMESPACE.to_owned() + "hasPublicationEventType";
    let p_has_related_audio_programme = EBUCORE_NAMESPACE.to_owned() + "hasRelatedAudioProgramme";
    let p_has_related_text_line = EBUCORE_NAMESPACE.to_owned() + "hasRelatedTextLine";
    let p_has_role = EBUCORE_NAMESPACE.to_owned() + "hasRole";
    let p_has_season = EBUCORE_NAMESPACE.to_owned() + "hasSeason";
    let p_has_target_audience = EBUCORE_NAMESPACE.to_owned() + "hasTargetAudience";
    let p_has_text_line_type = EBUCORE_NAMESPACE.to_owned() + "hasTextLineType";
    let p_has_topic = EBUCORE_NAMESPACE.to_owned() + "hasTopic";
    let p_is_agent = EBUCORE_NAMESPACE.to_owned() + "isAgent";
    let p_is_character = EBUCORE_NAMESPACE.to_owned() + "isCharacter";
    let p_is_member_of = EBUCORE_NAMESPACE.to_owned() + "isMemberOf";
    let p_live = EBUCORE_NAMESPACE.to_owned() + "live";
    let p_mid_roll_ad_allowed = EBUCORE_NAMESPACE.to_owned() + "midRollAdAllowed";
    let p_organisation_name = EBUCORE_NAMESPACE.to_owned() + "organisationName";
    let p_original_title = EBUCORE_NAMESPACE.to_owned() + "originalTitle";
    let p_publication_channel = EBUCORE_NAMESPACE.to_owned() + "publicationChannel";
    let p_publication_channel_id = EBUCORE_NAMESPACE.to_owned() + "publicationChannelId";
    let p_publication_channel_name = EBUCORE_NAMESPACE.to_owned() + "publicationChannelName";
    let p_publication_start_date_time = EBUCORE_NAMESPACE.to_owned() + "publicationStartDateTime";
    let p_references = EBUCORE_NAMESPACE.to_owned() + "references";
    let p_resource_id = EBUCORE_NAMESPACE.to_owned() + "resourceId";
    let p_season_number = EBUCORE_NAMESPACE.to_owned() + "seasonNumber";
    let p_synopsis = EBUCORE_NAMESPACE.to_owned() + "synopsis";
    let p_title = EBUCORE_NAMESPACE.to_owned() + "title";

    let p_type = RDF_NAMESPACE.to_owned() + "type";

    let p_pref_label = SKOS_NAMESPACE.to_owned() + "prefLabel";
    let p_definition = SKOS_NAMESPACE.to_owned() + "definition";

    let o_audio_programme = EBUCORE_NAMESPACE.to_owned() + "AudioProgramme";
    let o_cast = EBUCORE_NAMESPACE.to_owned() + "Cast";
    let o_character = EBUCORE_NAMESPACE.to_owned() + "Character";
    let o_editorial_object = EBUCORE_NAMESPACE.to_owned() + "EditorialObject";
    let o_group = EBUCORE_NAMESPACE.to_owned() + "Group";
    let o_organisation = EBUCORE_NAMESPACE.to_owned() + "Organisation";
    let o_person = EBUCORE_NAMESPACE.to_owned() + "Person";
    let o_publication_channel = EBUCORE_NAMESPACE.to_owned() + "PublicationChannel";
    let o_publication_event = EBUCORE_NAMESPACE.to_owned() + "PublicationEvent";
    let o_season = EBUCORE_NAMESPACE.to_owned() + "Season";
    let o_tag = EBUCORE_NAMESPACE.to_owned() + "Tag";
    let o_text_line = EBUCORE_NAMESPACE.to_owned() + "TextLine";

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
        &("http://resources.idfrancetv.fr/medias/".to_string() + &parent_id),
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
        Some(XML_NAMESPACE.to_owned() + "duration"),
        false,
      );
    }

    add_link(
      graph,
      &subject,
      &p_has_editorial_object_type,
      &("http://resources.idfrancetv.fr/ontologies/editorial_object_type#".to_string()
        + &self.kind.id),
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
      Some(XML_NAMESPACE.to_owned() + "dateTime"),
      false,
    );
    add_link(
      graph,
      &subject,
      &p_date_modified,
      &self.updated_at,
      None,
      Some(XML_NAMESPACE.to_owned() + "dateTime"),
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
        Some(XML_NAMESPACE.to_owned() + "dateTime"),
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
        Some(XML_NAMESPACE.to_owned() + "boolean"),
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
      &("urn::uuid:".to_owned() + &self.id),
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
        &(p_ftv_audio.to_owned() + "complet_2.0_" + &audio_track.id),
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
        &(p_ftv_sous_titre.to_owned() + &text_track.id),
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
        Some(XML_NAMESPACE.to_owned() + "dateTime"),
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
        Some(XML_NAMESPACE.to_owned() + "duration"),
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
        Some(XML_NAMESPACE.to_owned() + "boolean"),
        false,
      );
    }
    add_link(
      graph,
      &s_publication_event,
      &p_free,
      &self.drm.to_string(),
      None,
      Some(XML_NAMESPACE.to_owned() + "boolean"),
      false,
    );
    if let Some(previously_broadcasted) = self.previously_broadcasted {
      add_link(
        graph,
        &s_publication_event,
        &p_first_showing,
        &previously_broadcasted.to_string(),
        None,
        Some(XML_NAMESPACE.to_owned() + "boolean"),
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
        Some(XML_NAMESPACE.to_owned() + "boolean"),
        false,
      );
    }
    if let Some(ref rating) = self.rating {
      add_link(
        graph,
        &s_publication_event,
        &p_has_target_audience,
        &(p_ftv_csa.to_owned() + &rating.id),
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
      Some(XML_NAMESPACE.to_owned() + "boolean"),
      false,
    );
    if let Some(ref broadcasted_at) = self.broadcasted_at {
      add_link(
        graph,
        &s_publication_event,
        &p_date_broadcast,
        &broadcasted_at,
        None,
        Some(XML_NAMESPACE.to_owned() + "dateTime"),
        false,
      );
    }

    // category
    if let Some(ref category) = self.category {
      add_link(
        graph,
        &subject,
        &p_has_genre,
        &(FTV_GENRE_NAMESPACE.to_string() + &category.id),
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
        &(FTV_ROLE_NAMESPACE.to_string() + &people.role.id),
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

    self.resources.to_rdf(graph);
  }
}
