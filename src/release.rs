use area::Area;
use artist::ArtistCredit;
use cover_art_archive::CoverArtArchive;
use label::{LabelInfo, Label};
use media::Media;
use release_group::ReleaseGroup;
use text_representation::TextRepresentation;
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use uuid::Uuid;
use enums::*;
use utils;
use serde_json;
use brainz_macros;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReleaseEvent {
    pub area: Area,
    pub date: String
}

impl ReleaseEvent {
    pub fn new(area: Area, date: String) -> ReleaseEvent{ 
        ReleaseEvent {
            area: area,
            date: date
        }
    }

    pub fn empty() -> ReleaseEvent {
        ReleaseEvent::new(
            Area::empty(),
            String::new()
        )
    }
}

impl Default for ReleaseEvent {
    fn default() -> ReleaseEvent { ReleaseEvent::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Release {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub title: String,
    pub realease_events: Vec<ReleaseEvent>,
    pub asin: String, 
    pub cover_art_archive: CoverArtArchive,
    pub text_representation: TextRepresentation,
    pub packaging: Option<Packaging>,
    pub status: Option<ReleaseStatus>, 
    pub disambiguation: String,
    pub release_group: ReleaseGroup,
    pub quality: String,
    pub barcode: String,
    pub label_info: Vec<LabelInfo>,
    pub date: String,
    pub artist_credit: Vec<ArtistCredit>,
    pub country: String,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub status_id: Uuid,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub packaging_id: Uuid,
    pub media: Vec<Media>,
    pub label: Label,
    pub catalog_number: String,
    pub language: String,
    pub script: String,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub mbid: Uuid,
    pub annotation: String,
    pub score: u8
}

impl Release {
    pub fn new(id: Uuid, 
               title: String, 
               realease_events: Vec<ReleaseEvent>,
               asin: String,
               cover_art_archive: CoverArtArchive,
               text_representation: TextRepresentation,
               packaging: Option<Packaging>,
               status: Option<ReleaseStatus>,
               disambiguation: String,
               release_group: ReleaseGroup,
               quality: String,
               barcode: String,
               label_info: Vec<LabelInfo>,
               date: String,
               artist_credit: Vec<ArtistCredit>,
               country: String,
               status_id: Uuid,
               packaging_id: Uuid,
               media: Vec<Media>,
               label: Label,
               catalog_number: String,
               language: String,
               script: String,
               mbid: Uuid,
               annotation: String,
               score: u8) -> Release{ 

        Release {
            id: id,
            title: title,
            realease_events: realease_events,
            asin: asin,
            cover_art_archive: cover_art_archive,
            text_representation: text_representation,
            packaging: packaging,
            status: status,
            disambiguation: disambiguation,
            release_group: release_group,
            quality: quality,
            barcode: barcode,
            label_info: label_info,
            date: date,
            artist_credit: artist_credit,
            country: country,
            status_id: status_id,
            packaging_id: packaging_id,
            media: media,
            label: label,
            catalog_number: catalog_number,
            language: language,
            script: script,
            mbid: mbid,
            annotation: annotation,
            score: score
        }
    }

    pub fn empty() -> Release {
        Release::new(
            Uuid::nil(),
            String::new(),
            Vec::new(),
            String::new(),
            CoverArtArchive::empty(),
            TextRepresentation::empty(),
            None,
            None,
            String::new(),
            ReleaseGroup::empty(),
            String::new(),
            String::new(),
            Vec::new(),
            String::new(),
            Vec::new(),
            String::new(),
            Uuid::nil(),
            Uuid::nil(),
            Vec::new(),
            Label::empty(),
            String::new(),
            String::new(),
            String::new(),
            Uuid::nil(),
            String::new(),
            0
        )
    }
}

impl Default for Release {
    fn default() -> Release { Release::empty() }
}
