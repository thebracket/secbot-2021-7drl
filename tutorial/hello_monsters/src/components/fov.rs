use std::collections::HashSet;
use bracket_lib::prelude::Point;

pub struct FieldOfView {
    pub radius: i32,
    pub visible_tiles: HashSet<Point>
}