use uuid::Uuid;
use enums::*;

#[derive(Debug, Clone)]
pub struct Album {
    pub title: String,
    pub release_date: String,
    pub id: Uuid,
    pub artist: Uuid,
    pub primary_type: AlbumMainType,
    pub secondary_types: Vec<AlbumSecondaryType>
}

impl Album {
    pub fn new(title: String, release_date: String, id: Uuid, artist: Uuid, primary_type: AlbumMainType, secondary_types: Vec<AlbumSecondaryType>) -> Album {
        Album {
            title: title,
            release_date: release_date,
            id: id,
            artist: artist,
            primary_type: primary_type,
            secondary_types: secondary_types
        }
    }
}
