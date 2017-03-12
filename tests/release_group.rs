extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;

#[test]
fn release_group_parsing(){
    let json_data = r#"{
            "first-release-date": "2008-01-16",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "id": "0d69911b-28f0-38fb-b5f5-29cf26839e3e",
            "primary-type": "Album",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "title": "Mixmag Presents: Tech-Trance-Electro Madness",
            "disambiguation": ""
    }"#;

    let res: release_group::ReleaseGroup = serde_json::from_str(json_data).unwrap();
}
