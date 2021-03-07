use super::{all_wall, colonists::spawn_first_colonist, spawn_face_eater, spawn_random_colonist};
use crate::{
    components::{Description, Door, Glyph, Position, TileTrigger},
    map::{tile::TileType, Layer, Tile, HEIGHT, WIDTH},
};
use bracket_lib::prelude::*;
use legion::*;

pub fn build_mine_middle(ecs: &mut World) -> Layer {
    let mut layer = Layer::new(std::usize::MAX, ecs); // Gets a default layer
    all_wall(&mut layer);
    let center_pt : Point = Point::new(WIDTH/2, HEIGHT/2);

    // Start by building a platform with a mining hole around it
    for y in center_pt.y-10..=center_pt.y+10 {
        for x in center_pt.x-10..=center_pt.x+10 {
            let pt = Point::new(x,y);
            let idx = layer.point2d_to_index(pt);
            layer.tiles[idx] = Tile::empty();
            let d = DistanceAlg::Pythagoras.distance2d(center_pt, pt);
            if d >= 9.0 {
                layer.tiles[idx] = Tile::floor();
            }

            if y == center_pt.y || y == center_pt.y+1 || y == center_pt.y - 1 {
                layer.tiles[idx] = Tile::floor();
            }
        }
    }

    // Place the up and down stairs
    let up_pt = center_pt + Point::new(-1, 0);
    let down_pt = center_pt + Point::new(1, 0);
    let up_idx = layer.point2d_to_index(up_pt);
    let down_idx = layer.point2d_to_index(down_pt);
    layer.tiles[up_idx] = Tile::stairs_up();
    layer.tiles[down_idx] = Tile::stairs_down();
    layer.starting_point = up_pt;
    layer.colonist_exit = down_pt;

    // Start building rooms and corridors

    layer
}