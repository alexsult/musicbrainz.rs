use utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LifeSpan {
    begin: Option<String>,
    end: Option<String>,
    #[serde(deserialize_with="utils::deserialize_bool")]
    ended: bool
}

impl LifeSpan {
    pub fn new(begin: Option<String>, end: Option<String>, ended: bool) -> LifeSpan {
        LifeSpan {
            begin: begin,
            end: end,
            ended: ended
        }
    }

    pub fn empty() -> LifeSpan {
        LifeSpan::new(
            None,
            None,
            false
        )
    }
}

impl Default for LifeSpan {
    fn default() -> LifeSpan { LifeSpan::empty() } 
}
