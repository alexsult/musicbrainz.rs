use artist::ArtistCredit;
use uuid::Uuid;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Recording {
    pub title: String,
    pub isrcs: Vec<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub disambiguation: String,
    pub artist_credit: ArtistCredit
}

impl Recording {
    pub fn new(title: String,
               isrcs: Vec<String>,
               id: Uuid,
               disambiguation: String,
               artist_credit: ArtistCredit) -> Recording {

        Recording {
            title: title,
            isrcs: isrcs,
            id: id,
            disambiguation: disambiguation,
            artist_credit: artist_credit
        }
    }

    pub fn empty() -> Recording {
        Recording::new(
            String::new(),
            Vec::new(),
            Uuid::nil(),
            String::new(),
            ArtistCredit::empty()
        )
    }
}

impl Default for Recording {
    fn default() -> Recording { Recording::empty() }
}
