use uuid::Uuid;
use enums::*;
use std::collections::HashMap;
use std::fmt;
use traits::Entity;
use error::Error;
use artist::{Artist,  ArtistCredit};
use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ReleaseGroup {
    pub title: Option<String>,
    pub release_date: Option<String>,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub artist: Uuid,
    pub artist_credit: Vec<ArtistCredit>,
    pub primary_type: AlbumType,
    pub secondary_types: Vec<AlbumType>
}

impl ReleaseGroup {
    pub fn new(title: Option<String>, 
               release_date: Option<String>, 
               id: Uuid, 
               artist: Uuid, 
               artist_credit: Vec<ArtistCredit>,
               primary_type: AlbumType, 
               secondary_types: Vec<AlbumType>) -> ReleaseGroup {

        ReleaseGroup {
            title: title,
            release_date: release_date,
            id: id,
            artist: artist,
            artist_credit: artist_credit,
            primary_type: primary_type,
            secondary_types: secondary_types
        }
    }

    pub fn empty() -> ReleaseGroup {
        ReleaseGroup::new(
            None,
            None,
            Uuid::nil(),
            Uuid::nil(),
            Vec::new(),
            AlbumType::Other,
            Vec::new()
        )
    }
}

impl Default for ReleaseGroup {
    fn default() -> ReleaseGroup { ReleaseGroup::empty() }
}

impl PartialEq for ReleaseGroup {
    fn eq(&self, other: &ReleaseGroup) -> bool {
        self.id == other.id && self.artist == other.artist
    }
}

impl fmt::Display for ReleaseGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{primary} {name}", primary=self.primary_type, name=self.title.as_ref().unwrap());
        writeln!(f, "Id: {id}", id=self.id)
    }
}

