#[derive(Debug, Clone)]
pub struct LifeSpan {
    begin: String,
    end: String,
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
