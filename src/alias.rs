use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Alias {
    pub sort_name: String,
    pub name: String,
    pub locale: String,
    #[serde(rename = "type")]
    pub alias_type: String,
    #[serde(deserialize_with="utils::deserialize_bool")]
    pub primary: bool,
    pub begin_date: String,
    pub end_date: String,
    pub score: String
}

impl Alias {
    pub fn new(sort_name: String,
               name: String,
               locale: String,
               alias_type: String,
               primary: bool,
               begin_date: String,
               end_date: String,
               score: String) -> Alias {
        Alias{
            sort_name: sort_name,
            name: name,
            locale: locale,
            alias_type: alias_type,
            primary: primary,
            begin_date: begin_date,
            end_date: end_date,
            score: score
        }
    }

    pub fn empty() -> Alias {
        Alias::new(
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            false,
            String::new(),
            String::new(),
            String::new()
        )
    }
}

impl Default for Alias {
    fn default() -> Alias { Alias::empty() } 
}
