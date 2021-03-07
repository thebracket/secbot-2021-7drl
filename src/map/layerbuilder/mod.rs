mod entrance;
mod mine_top;
mod mine_middle;
mod caverns;
use super::{HEIGHT, Layer, Tile, WIDTH, tile::TileType};
mod colonists;
use colonists::*;
pub use entrance::build_entrance;
pub use mine_top::build_mine_top;
pub use mine_middle::build_mine_middle;
pub use caverns::build_caverns;
mod monsters;
use monsters::*;
use bracket_lib::prelude::{Point, Algorithm2D};

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

fn edge_filler(map: &mut Layer) {
    for y in 0..HEIGHT {
        let idx = map.point2d_to_index(Point::new(0, y));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
        let idx = map.point2d_to_index(Point::new(WIDTH - 1, y));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
    }
    for x in 0..WIDTH {
        let idx = map.point2d_to_index(Point::new(x, 0));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
        let idx = map.point2d_to_index(Point::new(x, HEIGHT - 1));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
    }
}