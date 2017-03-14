use artist::Artist;
use place::Place;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Relation {
    #[serde(rename="type")]
    pub relation_type: String,
    pub direction: String,
    pub artist: Artist,
    pub place: Place,
    pub attributes: Vec<String>
}

impl Relation {
    pub fn new(relation_type: String,
               direction: String,
               artist: Artist,
               place: Place,
               attributes: Vec<String>) -> Relation {

        Relation {
            relation_type: relation_type,
            direction: direction,
            artist: artist,
            place: place,
            attributes: attributes
        }
    }

    pub fn empty() -> Relation {
        Relation::new(
            String::new(),
            String::new(),
            Artist::empty(),
            Place::empty(),
            Vec::new()
        )
    }
}

impl Default for Relation {
    fn default() -> Relation { Relation::empty() }
}
