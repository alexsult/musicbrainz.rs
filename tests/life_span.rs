extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;

#[test]
fn life_span_parsing() {
    let json_data = r#"{
        "ended": null,
        "begin": "1981-01-05",
        "end": null
        }"#;
    
    let res: life_span::LifeSpan = serde_json::from_str(json_data).unwrap();
}

