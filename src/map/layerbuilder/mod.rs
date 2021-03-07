mod entrance;
mod mine_top;
mod mine_middle;
use super::{Layer, Tile};
mod colonists;
use colonists::*;
pub use entrance::build_entrance;
pub use mine_top::build_mine_top;
pub use mine_middle::build_mine_middle;
mod monsters;
use monsters::*;

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