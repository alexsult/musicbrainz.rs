use release_group::ReleaseGroup;
use uuid::Uuid;
use enums::{PersonType, AlbumType};
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use life_span::LifeSpan;
use area::Area;
use tag::Tag;
use serde_json;
use utils;
use brainz_macros;
use alias::Alias;
//use super::get_endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
#[serde(skip_serializing_if = "")]
pub struct Artist {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub gender_id: Option<Uuid>,
	#[serde(rename = "type")]
    pub artist_type: PersonType,
	#[serde(rename = "type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub artist_type_id: Uuid,
    pub tags: Vec<Tag>,
    pub release_groups: Vec<ReleaseGroup>,
    pub disambiguation: Option<String>,
    pub sort_name: Option<String>,
    pub life_span: LifeSpan,
    pub country: Option<String>,
    pub area: Area,
    pub begin_area: Area,
    pub end_area: Area,
    pub isnis: Vec<String>,
    pub ipis: Vec<String>,
    pub score: i32,
    pub aliases: Vec<Alias>
}

impl Artist {
    pub fn new(id: Uuid, 
               name: Option<String>, 
               gender: Option<String>, 
               gender_id: Option<Uuid>,
               artist_type: PersonType,
               artist_type_id: Uuid,
               tags: Vec<Tag>, 
               release_groups: Vec<ReleaseGroup>,
               disambiguation: Option<String>,
               sort_name: Option<String>,
               life_span: LifeSpan,
               country: Option<String>,
               area: Area,
               begin_area: Area,
               end_area: Area,
               isnis: Vec<String>,
               ipis: Vec<String>,
               score: i32,
               aliases: Vec<Alias>) -> Artist {
        Artist {
            id: id,
            name: name,
            gender: gender,
            gender_id: gender_id,
            artist_type: artist_type,
            artist_type_id: artist_type_id,
            tags: tags,
            release_groups: release_groups,
            disambiguation: disambiguation,
            sort_name: sort_name,
            life_span: life_span,
            country: country,
            area: area,
            begin_area: begin_area,
            end_area: end_area,
            isnis: isnis,
            ipis: ipis,
            score: score,
            aliases: aliases
        }
    }

    pub fn empty() -> Artist {
        Artist::new(
            Uuid::nil(),
            None,
            None,
            None,
            PersonType::Other,
            Uuid::nil(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            LifeSpan::empty(),
            None,
            Area::empty(),
            Area::empty(),
            Area::empty(),
            Vec::new(),
            Vec::new(),
            0,
            Vec::new()
        )
    }
}

impl Default for Artist {
    fn default() -> Artist { Artist::empty() }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.id == other.id &&
        self.name == other.name
    }

    fn ne(&self, other: &Artist) -> bool {
        self.id != other.id
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{name} ({type})", name=self.name.as_ref().unwrap(), type=self.artist_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtistCredit {
    pub name: String,
    pub sort_name: String,
    pub joinphrase: String,
    pub artist: Artist
}

impl ArtistCredit {
    pub fn new(name: String, 
               sort_name: String,
               joinphrase: String, 
               artist: Artist) -> ArtistCredit {
        ArtistCredit {
            name: name,
            sort_name: sort_name,
            joinphrase: joinphrase,
            artist: artist
        }
    }

    pub fn empty() -> ArtistCredit {
        ArtistCredit::new(
            String::new(),
            String::new(),
            String::new(),
            Artist::empty()
        )
    }
}

impl Default for ArtistCredit {
    fn default() -> ArtistCredit { ArtistCredit::empty() }
}
