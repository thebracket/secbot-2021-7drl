mod entrance;
use super::{Layer, Tile};
mod colonists;
use colonists::*;
pub use entrance::build_entrance;
mod monsters;
use monsters::*;

fn all_space(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::empty();
    });
}
