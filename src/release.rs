use area::Area;
use artist::ArtistCredit;
use cover_art_archive::CoverArtArchive;
use label::{LabelInfo, Label};
use media::Media;
use release_group::ReleaseGroup;
use text_representation::TextRepresentation;
use std::fmt;
use std::collections::HashMap;
use traits::Entity;
use error::Error;
use uuid::Uuid;
use enums::*;


#[derive(Debug, Clone)]
pub struct ReleaseEvent {
    pub area: Area,
    pub date: String
}

impl ReleaseEvent {
    pub fn new(area: Area, date: String) -> ReleaseEvent{ 
        ReleaseEvent {
            area: area,
            date: date
        }
    }

    pub fn empty() -> ReleaseEvent {
        ReleaseEvent::new(
            Area::empty(),
            String::new()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Release {
    pub id: Uuid,
    pub title: String,
    pub realease_events: Vec<ReleaseEvent>,
    pub asin: String, 
    pub cover_art_archive: CoverArtArchive,
    pub text_representation: TextRepresentation,
    pub packaging: Packaging,
    pub status: ReleaseStatus, 
    pub disambiguation: String,
    pub release_group: ReleaseGroup,
    pub quality: String,
    pub barcode: String,
    pub label_info: Vec<LabelInfo>,
    pub date: String,
    pub artist_credit: Vec<ArtistCredit>,
    pub country: String,
    pub status_id: Uuid,
    pub packaging_id: Uuid,
    pub media: Vec<Media>,
    pub label: Label,
    pub catalog_number: String,
    pub language: String,
    pub script: String,
    pub mbid: Uuid,
    pub annotation: String,
}

impl Release {
    pub fn new(id: Uuid, 
               title: String, 
               realease_events: Vec<ReleaseEvent>,
               asin: String,
               cover_art_archive: CoverArtArchive,
               text_representation: TextRepresentation,
               packaging: Packaging,
               status: ReleaseStatus,
               disambiguation: String,
               release_group: ReleaseGroup,
               quality: String,
               barcode: String,
               label_info: Vec<LabelInfo>,
               date: String,
               artist_credit: Vec<ArtistCredit>,
               country: String,
               status_id: Uuid,
               packaging_id: Uuid,
               media: Vec<Media>,
               label: Label,
               catalog_number: String,
               language: String,
               script: String,
               mbid: Uuid,
               annotation: String) -> Release{ 

        Release {
            id: id,
            title: title,
            realease_events: realease_events,
            asin: asin,
            cover_art_archive: cover_art_archive,
            text_representation: text_representation,
            packaging: packaging,
            status: status,
            disambiguation: disambiguation,
            release_group: release_group,
            quality: quality,
            barcode: barcode,
            label_info: label_info,
            date: date,
            artist_credit: artist_credit,
            country: country,
            status_id: status_id,
            packaging_id: packaging_id,
            media: media,
            label: label,
            catalog_number: catalog_number,
            language: language,
            script: script,
            mbid: mbid,
            annotation: annotation
        }
    }

    pub fn empty() -> Release {
        Release::new(
            Uuid::nil(),
            String::new(),
            Vec::new(),
            String::new(),
            CoverArtArchive::empty(),
            TextRepresentation::empty(),
            Packaging::NoPack,
            ReleaseStatus::Official,
            String::new(),
            ReleaseGroup::empty(),
            String::new(),
            String::new(),
            Vec::new(),
            String::new(),
            Vec::new(),
            String::new(),
            Uuid::nil(),
            Uuid::nil(),
            Vec::new(),
            Label::empty(),
            String::new(),
            String::new(),
            String::new(),
            Uuid::nil(),
            String::new()
        )
    }
}

impl Entity for Release {
    fn search(&self, client: &super::MusicBrainz, params: &mut HashMap<&str, &str>) -> Result<Vec<Self>, Error> {
        let mut results : Vec<Release> = Vec::new();
        Ok(results)
    }

    fn lookup(&self, client: &super::MusicBrainz, entity_id: &Uuid, params: &mut HashMap<&str, &str>) -> Result<Self, Error> {
        let release_data = match client.get(&format!("release/{id}", id=entity_id), params) {
            Ok(x) => x,
            Err(e) => return Err(Error::ParseJson(e))
        };


        if !release_data["error"].is_null() {
            let error_msg = release_data["error"].to_string();
            println!("ERR {}", error_msg);
            return Err(Error::Http(error_msg));
        }

        let release_id = match release_data["id"].as_str() {
            Some(x) => {
                match Uuid::parse_str(x) {
                    Ok(y) => y,
                    Err(e) => return Err(Error::ParseUuid(e))
                }
            },
            None => return Err(Error::AsSlice)
        };
        
        let mut release_events: Vec<ReleaseEvent> = Vec::new();
        if !release_data["release-events"].is_null() {
            for release_event in release_data["release-events"].members() {
                let mut iso_3166_1_codes: Vec<String> = Vec::new();
                for iso_3166_1_code in release_event["area"]["iso-3166-1-codes"].members() {
                    iso_3166_1_codes.push(iso_3166_1_code.to_string());
                }

                let area_id =  match release_event["area"]["id"].as_str() {
                    Some(x) => {
                        match Uuid::parse_str(x) {
                            Ok(y) => y,
                            Err(e) => return Err(Error::ParseUuid(e))
                        }
                    },
                    None => return Err(Error::AsSlice)
                };

                let mut area = Area::new(
                    area_id,
                    release_event["area"]["sort-name"].to_string(),
                    release_event["area"]["name"].to_string(),
                    release_event["area"]["disambiguation"].to_string(),
                    iso_3166_1_codes
                );

                release_events.push(
                    ReleaseEvent::new(
                        area,
                        release_event["area"]["date"].to_string()
                    )
                );
            }
        }

        let mut cover_art_archive: CoverArtArchive = CoverArtArchive::empty();
        if !release_data["cover-art-archive"].is_null() {
            cover_art_archive.back = release_data["cover-art-archive"]["back"].as_bool().unwrap();
            cover_art_archive.front = release_data["cover-art-archive"]["front"].as_bool().unwrap();
            cover_art_archive.darkened = release_data["cover-art-archive"]["darkened"].as_bool().unwrap();
            cover_art_archive.count = release_data["cover-art-archive"]["count"].as_i32().unwrap();
            cover_art_archive.artwork = release_data["cover-art-archive"]["artwork"].as_bool().unwrap();
        }

        let mut text_representation: TextRepresentation = TextRepresentation::empty();
        if !release_data["text-representation"].is_null() {
            text_representation.script = release_data["text-representation"]["script"].to_string();
            text_representation.language = release_data["text-representation"]["language"].to_string();
        }

        let mut packaging: Packaging = Packaging::NoPack;
        if !release_data["packaging"].is_null() {
            packaging = match release_data["packaging"].as_str() {
                Some(x) => x.parse::<Packaging>().unwrap(),
                None => return Err(Error::AsSlice)
            };
        }

        let status = match release_data["status"].as_str() {
                Some(x) => x.parse::<ReleaseStatus>().unwrap(),
                None => return Err(Error::AsSlice)
            };

        let release_group = match ReleaseGroup::extract_release_group(&release_data["release-group"]) {
            Ok(x) => x,
            Err(e) => return Err(Error::AsSlice)
        };

        println!("{}",release_data["title"].to_string());
        //println!("{}",release_data);
        println!("{}",release_id);
        //println!("{}",release_events);
        println!("{}",release_data["asin"].to_string());
        println!("{:?}",cover_art_archive);
        println!("{:?}",text_representation);
        println!("{:?}",packaging);
        println!("{:?}",status);
        println!("{}",release_data["disambiguation"].to_string());
        println!("{:?}", release_group);

        Ok(Release::empty())
    }
}
