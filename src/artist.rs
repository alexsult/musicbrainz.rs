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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Artist {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub name: String,
    pub gender: String,
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub gender_id: Uuid,
	#[serde(rename = "type")]
    pub artist_type: PersonType,
	#[serde(rename = "type-id")]
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub artist_type_id: Uuid,
    pub tags: Vec<Tag>,
    pub release_groups: Vec<ReleaseGroup>,
    pub disambiguation: String,
    pub sort_name: String,
    pub life_span: LifeSpan,
    pub country: String,
    pub area: Area,
    pub begin_area: Area,
    pub end_area: Area,
    pub isnis: Vec<String>,
    pub ipis: Vec<String>,
    pub score: i32,
}

impl Artist {
    pub fn new(id: Uuid, 
               name: String, 
               gender: String, 
               gender_id: Uuid,
               artist_type: PersonType,
               artist_type_id: Uuid,
               tags: Vec<Tag>, 
               release_groups: Vec<ReleaseGroup>,
               disambiguation: String,
               sort_name: String,
               life_span: LifeSpan,
               country: String,
               area: Area,
               begin_area: Area,
               end_area: Area,
               isnis: Vec<String>,
               ipis: Vec<String>,
               score: i32) -> Artist {
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
            score: score
        }
    }

    pub fn empty() -> Artist {
        Artist::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            Uuid::nil(),
            PersonType::Other,
            Uuid::nil(),
            Vec::new(),
            Vec::new(),
            String::new(),
            String::new(),
            LifeSpan::empty(),
            String::new(),
            Area::empty(),
            Area::empty(),
            Area::empty(),
            Vec::new(),
            Vec::new(),
            0
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
        writeln!(f, "{name} ({type})", name=self.name, type=self.artist_type)
    }
}

impl Entity for Artist {
    fn search(&self, 
              client: &super::MusicBrainz, 
              params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {

        let data = match client.get("artist", params) {
            Ok(x) => x,
            Err(e) => return Err(Error::AsSlice) 
        };

        let search_results: ArtistSearchResult = match serde_json::from_str(&data) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        Ok(search_results.artists)
    }

    fn lookup(&self, 
              client: &super::MusicBrainz, 
              entity_id: &Uuid, 
              params: &mut HashMap<&str, &str>) -> Result<Self, Error> {

        let artist_data = match client.get(&format!("artist/{id}", id=entity_id), params) {
            Ok(x) => x,
            Err(e) => return Err(Error::AsSlice) 
        };
    
        let artist_struct: Artist = match serde_json::from_str(&artist_data) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        Ok(artist_struct)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: String,
    pub artist: Artist
}

impl ArtistCredit {
    pub fn new(name: String, joinphrase: String, artist: Artist) -> ArtistCredit {
        ArtistCredit {
            name: name,
            joinphrase: joinphrase,
            artist: artist
        }
    }

    pub fn empty() -> ArtistCredit {
        ArtistCredit::new(
            String::new(),
            String::new(),
            Artist::empty()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtistSearchResult {
    pub created: String,
    pub count: i32,
    pub offset: i32,
    pub artists: Vec<Artist>
}

impl ArtistSearchResult {
    pub fn new(created: String,
               count: i32,
               offset: i32,
               artists: Vec<Artist>) -> ArtistSearchResult {

        ArtistSearchResult {
            created: created,
            count: count,
            offset: offset,
            artists: artists
        }
    }
    
    pub fn empty() -> ArtistSearchResult {
        ArtistSearchResult::new(
            String::new(),
            0,
            0,
            Vec::new()
        )
    }
}

impl Default for ArtistSearchResult {
    fn default() -> ArtistSearchResult { ArtistSearchResult::empty() }
}
