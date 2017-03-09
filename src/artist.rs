use release_group::ReleaseGroup;
use uuid::Uuid;
use enums::{PersonType, AlbumType};
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use json::JsonValue;
use life_span::LifeSpan;
use area::Area;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub gender: String,
    pub gender_id: Uuid,
    pub artist_type: PersonType,
    pub artist_type_id: Uuid,
    pub tags: Vec<String>,
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
}

impl Artist {
    pub fn new(id: Uuid, 
               name: String, 
               gender: String, 
               gender_id: Uuid,
               artist_type: PersonType,
               artist_type_id: Uuid,
               tags: Vec<String>, 
               release_groups: Vec<ReleaseGroup>,
               disambiguation: String,
               sort_name: String,
               life_span: LifeSpan,
               country: String,
               area: Area,
               begin_area: Area,
               end_area: Area,
               isnis: Vec<String>,
               ipis: Vec<String>) -> Artist {
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
            ipis: ipis
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
            Vec::new()
        )
    }
    
    pub fn extract_artist(json_data: &JsonValue) -> Result<Artist, Error> {
        let artist_element = Artist::empty();

        // where the value is a string
        for element in vec!["name", 
                                "gender",
                                "disambiguation",
                                "sort_name",
                                "country"] {
            if !json_data[element].is_null() { 
                match json_data[element].as_str() {
                    Some(x) => artist_element = Artist { element: x,
                                                        .. artist_element },
                    None => return Err(Error::AsSlice) 
                }; 
            }
        }
    
        // where the value is an Uuid
        for element in vec!["id",
                             "gender_id",
                             "artist_type_id"] {
            if !json_data[element].is_null() {
                match json_data[element].as_str() {
                    Some(x) => {
                        match Uuid::parse_str(x) {
                            Ok(y) => artist_element = Artist { element: y,
                                                        .. artist_element },
                            Err(e) => return Err(Error::ParseUuid(e))
                        }
                    },
                    None => return Err(Error::AsSlice)
                }
            }
        }
    
        // where the value is a Vec<String>
        for element in vec!["tags",
                             "isnis",
                             "ipis"] {
            if !json_data[element].is_null() { 
                match json_data[element].as_str() {
                    Some(x) => artist_element = Artist { element: artist_element[element].push(x),
                                                        .. artist_element },
                    None => return Err(Error::AsSlice) 
                }; 
            }
        }
        
        /*
        if !json_data["type"].is_null() { 
            match json_data["type"].as_str() {
                Some(x) => artist_element.artist_type = x,
                None => return Err(Error::AsSlice)
            };
        }


        println!("AAA {:?}", json_data);

        let mut tags: Vec<String> = Vec::new();
        if !json_data["tags"].is_null() {
            for tag in json_data["tags"].members() {
                tags.push(tag["name"].to_string());
            }
        }

        let artist_id = match json_data["id"].as_str() {
            Some(x) => {
                match Uuid::parse_str(x) {
                    Ok(y) => y,
                    Err(e) => return Err(Error::ParseUuid(e))
                }
            },
            None => return Err(Error::AsSlice)
        };

        let mut artist_albums: Vec<ReleaseGroup> = Vec::new();
        if !json_data["release-groups"].is_null() {
            for album in json_data["release-groups"].members() {
                match ReleaseGroup::extract_release_group(album) {
                    Ok(x) => artist_albums.push(x),
                    Err(e) => return Err(Error::AsSlice)
                }
            }
        }

        Ok(Artist::new(
            artist_id,
            json_data["name"].to_string(),
            json_data["gender"].to_string(),
            artist_type,
            tags,
            artist_albums
        ))
        */

        Ok(artist_element)
    }
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
        writeln!(f, "{name} ({type})", name=self.name, type=self.artist_type);
        writeln!(f, "Id: {id}", id=self.id.hyphenated().to_string())
    }
}

impl Entity for Artist {
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {
        let data = match client.get("artist", params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        let count = data["count"].as_i32().unwrap();
        let mut results : Vec<Artist> = Vec::new();

        if count == 0 {
            return Ok(results);
        }

        let artists = &data["artists"];

        for artist in artists.members() {
            if !artist["score"].is_null() {
                if artist["score"] == "100" {
                    if !artist["name"].is_null() {
                        let name = artist["name"].to_string();
                        let gender = artist["gender"].to_string();

                        let id = match artist["id"].as_str() {
                            Some(x) => {
                                match Uuid::parse_str(x) {
                                    Ok(y) => y,
                                    Err(e) => return Err(Error::ParseUuid(e))
                                }
                            },
                            None => return Err(Error::AsSlice)
                        };

                        let mut tags: Vec<String> = Vec::new();
                        let release_groups: Vec<ReleaseGroup> = Vec::new();

                        for tag in artist["tags"].members() {
                            tags.push(tag["name"].to_string());
                        }

                        results.push(Artist::new(id, name, gender, PersonType::Other, tags, release_groups));
                    }
                }
            }
        }
        Ok(results)
    }

    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error> {
        let artist_data = match client.get(&format!("artist/{id}", id=entity_id), params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };

        if !artist_data["error"].is_null() {
            let error_msg = artist_data["error"].to_string();
            return Err(Error::Http(error_msg));
        }
    
        Artist::extract_artist(&artist_data)
    }
}


#[derive(Debug, Clone)]
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


