#[derive(Debug, Clone)]
pub struct Disc {
    pub sectors: i32,
    pub offset_count: i32,
    pub id: String,
    pub offsets: Vec<i32>
}

impl Disc {
    pub fn new(sectors: i32,
               offset_count: i32,
               id: String,
               offsets: Vec<i32>) -> Disc {

        Disc {
            sectors: sectors,
            offset_count: offset_count,
            id: id,
            offsets: offsets
        }
    }

    pub fn empty() -> Disc {
        Disc::new(
            0,
            0,
            String::new(),
            Vec::new()
        )
    }
}
