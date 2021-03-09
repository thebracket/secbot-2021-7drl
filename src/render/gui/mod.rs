pub mod skeleton;
use bracket_lib::prelude::*;
pub use skeleton::*;
pub mod status;
pub use status::*;
pub mod queries;
pub use queries::*;
pub mod colony_info;
pub use colony_info::*;
pub mod targeting;
use crate::map::WIDTH;
pub use targeting::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    if pos.x > 0 && pos.y > 0 {
        batch.print_color(pos, text, color);
    }
}
