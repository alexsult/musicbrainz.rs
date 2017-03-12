use recording::Recording;
use artist::ArtistCredit;
use uuid::Uuid;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Track {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub title: String,
    pub length: i32,
    pub recording: Recording,
    pub artist_credit: ArtistCredit,
    pub number: i32
}

impl Track {
    pub fn new(id: Uuid,
               title: String,
               length: i32,
               recording: Recording,
               artist_credit: ArtistCredit,
               number: i32) -> Track {
        
        Track {
            id: id,
            title: title,
            length: length,
            recording: recording,
            artist_credit: artist_credit,
            number: number
        }
    }

    pub fn empty() -> Track {
        Track::new(
            Uuid::nil(),
            String::new(),
            0,
            Recording::empty(),
            ArtistCredit::empty(),
            0
        )
    }
}

impl Default for Track {
    fn default() -> Track { Track::empty() }
}