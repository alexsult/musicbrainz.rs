use uuid::Uuid;
use utils;
use error::Error;
use std::collections::HashMap;
use serde_json;
use traits::Entity;
use tag::Tag;
use alias::Alias;
use life_span::LifeSpan;
use relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Event {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    #[serde(rename="type")]
    pub event_type: String,
    pub name: String,
    pub life_span: LifeSpan,
    pub relations: Vec<Relation>
}

impl Event {
    pub fn new(id: Uuid,
               event_type: String,
               name: String,
               life_span: LifeSpan,
               relations: Vec<Relation>) -> Event {
        Event {
            id: id,
            event_type: event_type,
            name: name,
            life_span: life_span,
            relations: relations
        }
    }

    pub fn empty() -> Event {
        Event::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            LifeSpan::empty(),
            Vec::new()
        )
    }
}

impl Default for Event {
    fn default() -> Event { Event::empty() }
}
