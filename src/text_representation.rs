#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct TextRepresentation {
    pub script: String,
    pub language: String
}

impl TextRepresentation {
    pub fn new(script: String, language: String) -> TextRepresentation {
        TextRepresentation{
            script: script,
            language: language
        }
    }

    pub fn empty() -> TextRepresentation {
        TextRepresentation::new(
            String::new(),
            String::new()
        )
    }
}

impl Default for TextRepresentation {
    fn default() -> TextRepresentation { TextRepresentation::empty() }
}
