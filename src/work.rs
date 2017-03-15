use serde_json;
use utils;
use brainz_macros;
use relation::Relation;
use uuid::Uuid;
use traits::Entity;
use error::Error;
use std::collections::HashMap;
use artist::ArtistCredit;
use tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Work {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    #[serde(rename="type")]
    pub work_type: String,
    pub title: String,
    pub artist_credit: ArtistCredit,
    pub disambiguation: String,
    pub iwcs: Vec<String>,
    pub relations: Vec<Relation>,
    pub tags: Vec<Tag>,
    pub language: String
}

impl Work {
    pub fn new(id: Uuid,
               work_type: String,
               title: String,
               artist_credit: ArtistCredit,
               disambiguation: String,
               iwcs: Vec<String>,
               relations: Vec<Relation>,
               tags: Vec<Tag>,
               language: String) -> Work {
        Work {
            id: id,
            work_type: work_type,
            title: title,
            artist_credit: artist_credit,
            disambiguation: disambiguation,
            iwcs: iwcs,
            relations: relations,
            tags: tags,
            language: language
        }
    }

    pub fn empty() -> Work {
        Work::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            ArtistCredit::empty(),
            String::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            String::new()
        )
    }
}

impl Default for Work {
    fn default() -> Work { Work::empty() } 
}
