use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LifeSpan {
    begin: String,
    end: String,
    #[serde(deserialize_with="utils::deserialize_bool")]
    ended: bool
}

impl LifeSpan {
    pub fn new(begin: String, end: String, ended: bool) -> LifeSpan {
        LifeSpan {
            begin: begin,
            end: end,
            ended: ended
        }
    }

    pub fn empty() -> LifeSpan {
        LifeSpan::new(
            String::new(),
            String::new(),
            false
        )
    }
}

impl Default for LifeSpan {
    fn default() -> LifeSpan { LifeSpan::empty() } 
}
