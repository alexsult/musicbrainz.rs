use uuid::Uuid;
use utils;
use error::Error;
use std::collections::HashMap;
use serde_json;
use traits::Entity;
use tag::Tag;
use alias::Alias;
use relation::Relations;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Area {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub sort_name: String,
    pub name: String,
    pub disambiguation: String,
    pub iso_3166_1_codes: Vec<String>,
    pub iso_3166_2_codes: Vec<String>,
    pub tags: Vec<Tag>,
    pub aliases: Vec<Alias>,
    pub score: String,
    pub relation_list: Vec<Relations>
}

impl Area {
    pub fn new(id: Uuid, 
               sort_name: String, 
               name: String, 
               disambiguation: String,
               iso_3166_1_codes: Vec<String>,
               iso_3166_2_codes: Vec<String>,
               tags: Vec<Tag>,
               aliases: Vec<Alias>,
               score: String,
               relation_list: Vec<Relations>) -> Area { 

        Area {
            id: id,
            sort_name: sort_name,
            name: name, 
            disambiguation: disambiguation,
            iso_3166_1_codes: iso_3166_1_codes,
            iso_3166_2_codes: iso_3166_2_codes,
            tags: tags,
            aliases: aliases,
            score: score,
            relation_list: relation_list
        }
    }

    pub fn empty() -> Area {
        Area::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            String::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            String::new(),
            Vec::new()
        )
    }
}

impl Default for Area {
    fn default() -> Area { Area::empty() }
}
