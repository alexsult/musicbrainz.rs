use uuid::Uuid;
use utils;

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Area {
    #[serde(deserialize_with="utils::uuid_from_string")]
    #[serde(serialize_with="utils::string_from_uuid")]
    pub id: Uuid,
    pub sort_name: String,
    pub name: String,
    pub disambiguation: String,
    pub iso_3166_1_codes: Vec<String>
}

impl Area {
    pub fn new(id: Uuid, 
               sort_name: String, 
               name: String, 
               disambiguation: String,
               iso_3166_1_codes: Vec<String>) -> Area { 

        Area {
            id: id,
            sort_name: sort_name,
            name: name, 
            disambiguation: disambiguation,
            iso_3166_1_codes: iso_3166_1_codes
        }
    }

    pub fn empty() -> Area {
        Area::new(
            Uuid::nil(),
            String::new(),
            String::new(),
            String::new(),
            Vec::new()
        )
    }
}

impl Default for Area {
    fn default() -> Area { Area::empty() }
}
