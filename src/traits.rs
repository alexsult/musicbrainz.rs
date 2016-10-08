use artist::Artist;
use release_group::ReleaseGroup;
use std::collections::HashMap;

/// Provides methods for browsing, looking up or searching artists.
pub trait ArtistTrait {
    fn search_artist(&self, params: &mut HashMap<&str, &str>) -> Vec<Artist>;
    fn lookup_artist(&self, artist: Artist, params: &mut HashMap<&str, &str>) -> Result<Artist, String>;
}

/// Provides methods for browsing, looking up or searching release groups.
pub trait AlbumTrait {
    fn search_album(&self, params: &mut HashMap<&str, &str>) -> Vec<ReleaseGroup>;
}
