use bracket_lib::prelude::Point;

pub struct Position {
    pub pt: Point,
    pub layer: u32,
}

impl Position {
    pub fn with_pt(pt: Point, layer: u32) -> Self {
        Self { pt, layer }
    }
}
