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
pub use targeting::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        //println!("Batched text[{}] at {:?}", text.to_string(), pos);
        batch.print_color(pos, text, color);
    }
}
