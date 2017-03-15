use artist::Artist;
use place::Place;
use uuid::Uuid;
use event::Event;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Relation {
    #[serde(rename="type")]
    pub relation_type: String,
    #[serde(rename="type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub relation_type_id: Uuid,
    pub direction: String,
    pub artist: Artist,
    pub place: Place,
    pub attributes: Vec<String>,
    pub begin: String,
    pub end: String,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub ended: bool,
    pub target_credit: String,
    pub event: Event,
    pub ordering_key: i32
}

impl Relation {
    pub fn new(relation_type: String,
               relation_type_id: Uuid,
               direction: String,
               artist: Artist,
               place: Place,
               attributes: Vec<String>,
               begin: String,
               end: String,
               ended: bool,
               target_credit: String,
               event: Event,
               ordering_key: i32) -> Relation {

        Relation {
            relation_type: relation_type,
            relation_type_id: relation_type_id,
            direction: direction,
            artist: artist,
            place: place,
            attributes: attributes,
            begin: begin,
            end: end,
            ended: ended,
            target_credit: target_credit,
            event: event,
            ordering_key: ordering_key
        }
    }

    pub fn empty() -> Relation {
        Relation::new(
            String::new(),
            Uuid::nil(),
            String::new(),
            Artist::empty(),
            Place::empty(),
            Vec::new(),
            String::new(),
            String::new(),
            false,
            String::new(),
            Event::empty(),
            0
        )
    }
}

impl Default for Relation {
    fn default() -> Relation { Relation::empty() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Relations {
    pub relations: Vec<Relation>
}

impl Relations {
    pub fn new(relations: Vec<Relation>) -> Relations {
        Relations {
            relations: relations
        }
    }

    pub fn empty() -> Relations {
        Relations::new(
            Vec::new()
        )
    }
}

impl Default for Relations {
    fn default() -> Relations { Relations::empty() }
}
