extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;

#[test]
fn test_area_parsing(){
    let json_data = r#"{
        "id": "71bbafaa-e825-3e15-8ca9-017dcad1748b",
        "disambiguation": "",
        "iso-3166-1-codes": [
            "CA"
        ],
        "sort-name": "Canada",
        "name": "Canada"
    }"#;
    
    let res: area::Area = serde_json::from_str(json_data).unwrap();
}
