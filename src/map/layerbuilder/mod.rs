mod caverns;
mod entrance;
mod mine_middle;
mod mine_top;
use super::{tile::TileType, Layer, Tile, HEIGHT, WIDTH};
mod colonists;
pub use caverns::build_caverns;
use colonists::*;
pub use entrance::build_entrance;
pub use mine_middle::build_mine_middle;
pub use mine_top::build_mine_top;
mod monsters;
use bracket_lib::prelude::{to_cp437, Algorithm2D, Point};
use monsters::*;
mod props;

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

fn is_wall_for_smoothing(idx: usize, map: &Layer) -> bool {
    map.tiles[idx].tile_type == TileType::Wall || map.tiles[idx].glyph == to_cp437('+')
}

fn smooth_walls(map: &mut Layer) {
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let idx = map.point2d_to_index(Point::new(x, y));
            if map.tiles[idx].tile_type == TileType::Wall
                && map.tiles[idx].glyph != to_cp437('+')
            {
                let mut mask: u8 = 0;
                if is_wall_for_smoothing(idx - WIDTH, map) {
                    mask += 1;
                }
                if is_wall_for_smoothing(idx + WIDTH, map) {
                    mask += 2;
                }
                if is_wall_for_smoothing(idx - 1, map) {
                    mask += 4;
                }
                if is_wall_for_smoothing(idx + 1, map) {
                    mask += 8;
                }

                let new_glyph = match mask {
                    0 => 9,    // Pillar because we can't see neighbors
                    1 => 186,  // Wall only to the north
                    2 => 186,  // Wall only to the south
                    3 => 186,  // Wall to the north and south
                    4 => 205,  // Wall only to the west
                    5 => 188,  // Wall to the north and west
                    6 => 187,  // Wall to the south and west
                    7 => 185,  // Wall to the north, south and west
                    8 => 205,  // Wall only to the east
                    9 => 200,  // Wall to the north and east
                    10 => 201, // Wall to the south and east
                    11 => 204, // Wall to the north, south and east
                    12 => 205, // Wall to the east and west
                    13 => 202, // Wall to the east, west, and south
                    14 => 203, // Wall to the east, west, and north
                    15 => 206, // ╬ Wall on all sides
                    _ => 35,   // We missed one?
                };
                map.tiles[idx].glyph = new_glyph;
            }
        }
    }
}
