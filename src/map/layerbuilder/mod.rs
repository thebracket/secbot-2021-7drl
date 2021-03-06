mod entrance;
use super::{Layer, Tile};
mod colonists;
pub use entrance::build_entrance;
use colonists::*;

fn all_space(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::empty();
    });
}
