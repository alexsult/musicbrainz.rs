extern crate hyper;
extern crate uuid;
extern crate url;
#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate brainz_macros;

use std::collections::HashMap;
use std::io::Read;
use url::{Url};
use error::Error;

//use traits::Bob;

#[derive(Debug)]
pub struct MusicBrainz {
    client: hyper::Client,
    user_agent: String
}

pub fn get_endpoint(struct_type: &str) -> Result<String,Error> {
    match struct_type {
        "Artist" => Ok(String::from("artist")),
        _ => Err(Error::AsSlice)
    }
}

impl MusicBrainz {
    /// Instantiates a new `MusicBrainz` struct.
    ///
    /// The `MusicBrainz` struct contains useful methods required by the library.
    /// It must be instantiated before using the implemented methods.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use musicbrainz::*;
    /// let musicbrainz = MusicBrainz::new();
    /// ```
    pub fn new() -> MusicBrainz {
        let user_agent = format!("{name}/{version} ( {homepage} )",
            name=env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION"),
            homepage=env!("CARGO_PKG_HOMEPAGE")
        );

        MusicBrainz {
            client: hyper::Client::new(),
            user_agent: user_agent
        }
    }

    //fn get(&self, url: &str, params: &HashMap<&str, &str>) -> json::Result<json::JsonValue> {
    fn get(&self, url: &str, params: &HashMap<&str, &str>) -> Result<String, Error> {
        let base_uri = "https://musicbrainz.org/ws/2";
        let mut endpoint = Url::parse(&format!("{}/{}", base_uri, url))
            .expect("error parsing URL");

        endpoint.query_pairs_mut().append_pair("fmt", "json");
        for (param, val) in params {
            endpoint.query_pairs_mut().append_pair(param, val);
        }

        let user_agent = self.user_agent.clone();
        let mut res = self.client.get(endpoint)
            .header(hyper::header::UserAgent(user_agent))
            .send()
            .expect(&format!("failed to get url '{}'", url));

        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("failed to read response body to string");

        Ok(buf)
    }

    pub fn artist(&self) -> artist::Artist {
        artist::Artist::empty()
    }

    pub fn release(&self) -> release::Release {
        release::Release::empty()
    }

}

pub mod utils;
pub mod area;
pub mod artist;
pub mod cover_art_archive;
pub mod disc;
pub mod label;
pub mod life_span;
pub mod media;
pub mod recording;
pub mod release;
pub mod release_group;
pub mod text_representation;
pub mod enums;
pub mod track;
pub mod traits;
pub mod error;
pub mod tag;

pub use traits::*;
pub use uuid::Uuid;
