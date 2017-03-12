extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;

#[test]
fn tag_parsing(){
    let json_data = r#"{
        "count": 1, 
        "name": "dance and electronica"
    }"#;
    
    let res: tag::Tag = serde_json::from_str(json_data).unwrap();
}
