mod entrance;
use super::{Layer, Tile};
pub use entrance::build_entrance;
mod colonists;
use colonists::*;

fn all_space(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::empty();
    });
}