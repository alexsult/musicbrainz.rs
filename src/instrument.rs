use alias::Alias;
use uuid::Uuid;
use tag::Tag;
use serde_json;
use utils;
use brainz_macros;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Instrument {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub score: String,
    pub aliases: Vec<Alias>,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub tags: Vec<Tag>
}

impl Instrument {
    pub fn new(id: Uuid,
               score: String,
               aliases: Vec<Alias>,
               instrument_type: String,
               tags: Vec<Tag>) -> Instrument {
        Instrument {
            id: id,
            score: score,
            aliases: aliases,
            instrument_type: instrument_type,
            tags: tags
        }
    }

    pub fn empty() -> Instrument { 
        Instrument::new(
            Uuid::nil(),
            String::new(),
            Vec::new(),
            String::new(),
            Vec::new()
        )
    }
}

impl Default for Instrument {
    fn default() -> Instrument { Instrument::empty() }
}
