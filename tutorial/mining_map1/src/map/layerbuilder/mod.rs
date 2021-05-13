mod entrance;
use super::{Layer, Tile};
pub use entrance::build_entrance;
mod colonists;
use colonists::*;
mod monsters;
pub use monsters::*;
mod mine_top;
mod mine_middle;
pub use mine_top::build_mine_top;
pub use mine_middle::build_mine_middle;

fn all_space(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::empty();
    });
}

fn all_wall(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::wall();
    });
}