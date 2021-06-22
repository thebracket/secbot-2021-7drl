use bracket_lib::prelude::Point;
use std::collections::HashSet;

pub struct FieldOfView {
    pub radius: i32,
    pub visible_tiles: HashSet<Point>,
}
