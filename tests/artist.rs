extern crate musicbrainz;
extern crate serde_json;
use musicbrainz::*;

#[test]
fn test_artist_parsing(){
    let json_data = r#"{
        "disambiguation": "",
        "type-id": "e431f5f6-b5d2-343d-8b36-72607fffb74b",
        "name": "Radiohead",
        "area": {
            "name": "United Kingdom",
            "sort-name": "United Kingdom",
            "disambiguation": "",
            "id": "8a754a16-0027-3a29-b6d7-2b40ea0481ed",
            "iso-3166-1-codes": [
                "GB"
            ]
        },
        "gender-id": null,
        "type": "Group",
        "gender": null,
        "sort-name": "Radiohead",
        "id": "a74b1b7f-71a5-4011-9441-d0b5e4122711",
        "ipis": [],
        "begin_area": {
            "name": "Abingdon-on-Thames",
            "sort-name": "Abingdon-on-Thames",
            "disambiguation": "",
            "id": "d840d4b3-8987-4626-928b-398de760cc24"
        },
        "isnis": [
            "0000000115475162"
        ],
        "country": "GB",
        "end_area": null,
        "life-span": {
            "ended": false,
            "begin": "1991",
            "end": null
        }}"#;
    
    //let json_parsed = json::parse(json_data).unwrap();
    //let res: artist::Artist = artist::Artist::extract_artist(&json_parsed).unwrap(); 
    let res: artist::Artist = serde_json::from_str(json_data).unwrap();
}

#[test]
fn test_full_artist_parsing(){
    let json_data = r#"{
    "release-groups": [
        {
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ],
            "id": "2c435990-69a2-43e0-b124-c96e8b02c455",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "title": "Hail and Backtrack",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2003"
        },
        {
            "disambiguation": "Pablo Honey / The Bends / OK Computer / Kid A / Amnesiac",
            "first-release-date": "2012-10-12",
            "primary-type": "Album",
            "title": "5 Album Set",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "30f765ef-d7f0-4cce-8a3b-d6e1069f1dda",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "disambiguation": "",
            "first-release-date": "2003",
            "title": "Sail to Montreux",
            "primary-type": "Album",
            "secondary-type-ids": [],
            "id": "407aa2b5-b48b-434e-881f-072ea6690860",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "primary-type": "Album",
            "title": "Oxford Angels: The Rarities",
            "disambiguation": "",
            "first-release-date": "2013",
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [],
            "id": "4e0ea78b-d058-461b-ac7c-3de43823a131",
            "secondary-type-ids": []
        },
        {
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [],
            "id": "5c14fd50-a2f1-3672-9537-b0dad91bea2f",
            "secondary-type-ids": [],
            "title": "Hail to the Thief",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2003-05-26"
        },
        {
            "disambiguation": "",
            "first-release-date": "",
            "primary-type": "Album",
            "title": "A Total Waste of Time",
            "id": "65d99b2c-05c1-378a-9218-5e0c19bc106f",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ]
        },
        {
            "title": "In Rainbows",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2007-10-10",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [],
            "id": "6e335887-60ba-38f0-95af-fae7774336bf"
        },
        {
            "title": "The Best Of",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2008-06-02",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "80975c59-4094-30d5-bb58-365e8b061274"
        },
        {
            "title": "Coup d\u2019\u00c9tat",
            "primary-type": "Album",
            "first-release-date": "2001-01-22",
            "disambiguation": "",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "85a1b388-7b4c-3145-8812-0de4d231c8c5"
        },
        {
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [],
            "id": "899b6d09-807e-4c18-a6d4-3642e00d6a3d",
            "title": "The King of Limbs",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2011-02-18"
        },
        {
            "title": "The Best of Radiohead",
            "primary-type": "Album",
            "first-release-date": "2009-11",
            "disambiguation": "",
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ],
            "id": "af44dc42-1bf4-4cdc-ad98-cdfdc2312b09",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ]
        },
        {
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [],
            "id": "b1392450-e666-3926-a536-22c65f834433",
            "title": "OK Computer",
            "primary-type": "Album",
            "first-release-date": "1997-05-21",
            "disambiguation": ""
        },
        {
            "title": "Album Box Set",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2007-12-10",
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ],
            "id": "b7db08f4-835a-4a69-970b-5321e6305032",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ]
        },
        {
            "secondary-type-ids": [],
            "id": "b8048f24-c026-3398-b23a-b5e50716cbc7",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "disambiguation": "",
            "first-release-date": "1995-03-08",
            "primary-type": "Album",
            "title": "The Bends"
        },
        {
            "disambiguation": "",
            "first-release-date": "2016-05-08",
            "primary-type": "Album",
            "title": "A Moon Shaped Pool",
            "secondary-type-ids": [],
            "id": "bbce0087-d386-4246-a51d-dbcdfdbe8fb2",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "title": "Amnesiac",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2001-06-04",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [],
            "id": "bca9280e-28b4-327f-8fe0-fd918579e486"
        },
        {
            "primary-type": "Album",
            "title": "Lost Treasures 1993\u20131997",
            "disambiguation": "",
            "first-release-date": "1997",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "c2ac2c8d-51c5-3abf-aafa-3f0e93795151"
        },
        {
            "disambiguation": "",
            "first-release-date": "2008",
            "primary-type": "Album",
            "title": "Greatest Hits",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "c4ae4ebb-2d68-4ba7-9c81-18ad7cb9022f",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "disambiguation": "",
            "first-release-date": "1993-02-22",
            "title": "Pablo Honey",
            "primary-type": "Album",
            "secondary-type-ids": [],
            "id": "cd76f76b-ff15-3784-a71d-4da3078a6851",
            "secondary-types": [],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "disambiguation": "",
            "first-release-date": "",
            "primary-type": "Album",
            "title": "B Sides",
            "id": "ceee3ae6-d614-350d-92ee-9c0b001a1440",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ]
        },
        {
            "disambiguation": "",
            "first-release-date": "2005",
            "title": "Breaks and Beats",
            "primary-type": "Album",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "dc626f33-2cf3-40b1-9612-c1d5bd2edf64",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "disambiguation": "",
            "first-release-date": "2004",
            "title": "Towering Above the Rest",
            "primary-type": "Album",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "e449d0a3-5661-338c-a51e-a9910f725fa5",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc"
        },
        {
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [],
            "id": "e75c0549-ad55-39e3-8025-c72c5d4a3c5d",
            "secondary-type-ids": [],
            "title": "Kid A",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2000-08-03"
        },
        {
            "title": "Looking Back at...",
            "primary-type": "Album",
            "disambiguation": "",
            "first-release-date": "2000",
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-types": [
                "Compilation"
            ],
            "id": "f4302994-a6fa-4a4a-9605-355a750468e6",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ]
        },
        {
            "primary-type": "Album",
            "title": "World Ballads Collection",
            "disambiguation": "",
            "first-release-date": "1999",
            "secondary-types": [
                "Compilation"
            ],
            "primary-type-id": "f529b476-6e62-324f-b0aa-1f3e33d313fc",
            "secondary-type-ids": [
                "dd2a21e1-0c00-3729-a7a0-de60b84eb5d1"
            ],
            "id": "f720c4d7-a80d-4466-b92d-9d1de99ddba7"
        }
    ],
    "disambiguation": "",
    "type-id": "e431f5f6-b5d2-343d-8b36-72607fffb74b",
    "name": "Radiohead",
    "area": {
        "name": "United Kingdom",
        "sort-name": "United Kingdom",
        "disambiguation": "",
        "id": "8a754a16-0027-3a29-b6d7-2b40ea0481ed",
        "iso-3166-1-codes": [
            "GB"
        ]
    },
    "gender-id": null,
    "type": "Group",
    "gender": null,
    "sort-name": "Radiohead",
    "id": "a74b1b7f-71a5-4011-9441-d0b5e4122711",
    "ipis": [],
    "begin_area": {
        "name": "Abingdon-on-Thames",
        "sort-name": "Abingdon-on-Thames",
        "disambiguation": "",
        "id": "d840d4b3-8987-4626-928b-398de760cc24"
    },
    "isnis": [
        "0000000115475162"
    ],
    "country": "GB",
    "end_area": null,
    "life-span": {
        "ended": false,
        "begin": "1991",
        "end": null
    }}"#;
    
    //let json_parsed = json::parse(json_data).unwrap();
    //let res: artist::Artist = artist::Artist::extract_artist(&json_parsed).unwrap(); 
    let res: artist::Artist = serde_json::from_str(json_data).unwrap();
}
