use super::{
    all_wall, colonists::spawn_first_colonist, edge_filler, spawn_face_eater, spawn_random_colonist,
};
use crate::{
    components::{Description, Door, Glyph, Position, TileTrigger},
    map::{tile::TileType, Layer, Tile, HEIGHT, WIDTH},
};
use bracket_lib::prelude::*;
use legion::*;

pub fn build_mine_top(ecs: &mut World) -> Layer {
    let mut layer = Layer::new(std::usize::MAX, ecs); // Gets a default layer
    all_wall(&mut layer);
    let center_pt: Point = Point::new(WIDTH / 2, HEIGHT / 2);

    // Start by building a platform with a mining hole around it
    for y in center_pt.y - 10..=center_pt.y + 10 {
        for x in center_pt.x - 10..=center_pt.x + 10 {
            let pt = Point::new(x, y);
            let idx = layer.point2d_to_index(pt);
            layer.tiles[idx] = Tile::empty();
            let d = DistanceAlg::Pythagoras.distance2d(center_pt, pt);
            if d >= 9.0 {
                layer.tiles[idx] = Tile::floor();
            }

            if y == center_pt.y || y == center_pt.y + 1 || y == center_pt.y - 1 {
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
    layer.colonist_exit = up_pt;

    // Start building rooms and corridors
    // Using the Hands-On Rust rooms/corridors builder slightly modified to go towards the middle
    let mut rooms = vec![Rect::with_size((WIDTH / 2) - 10, (HEIGHT / 2) - 10, 20, 20)];
    while rooms.len() < 14 {
        try_room(&mut rooms, &layer);
    }

    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();
    rooms.iter().skip(1).for_each(|r| {
        r.for_each(|pt| {
            let idx = layer.point2d_to_index(pt);
            layer.tiles[idx] = Tile::floor();
        });
        let room_center = r.center();
        if rng.range(0, 2) == 1 {
            // <callout id="co.dungeonrooms.randcor" />
            apply_horizontal_tunnel(&mut layer, room_center.x, center_pt.x, room_center.y);
            apply_vertical_tunnel(&mut layer, room_center.y, center_pt.y, center_pt.x);
        } else {
            apply_vertical_tunnel(&mut layer, room_center.y, center_pt.y, room_center.x);
            apply_horizontal_tunnel(&mut layer, room_center.x, center_pt.x, center_pt.y);
        }
    });

    edge_filler(&mut layer);
    std::mem::drop(rng);

    super::smooth_walls(&mut layer);

    rooms.iter().for_each(|r| {
        spawn_random_colonist(ecs, r.center(), 1);
    });

    layer
}

fn try_room(rooms: &mut Vec<Rect>, map: &Layer) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    let w = rng.range(4, 10);
    let h = rng.range(4, 10);
    let x = rng.range(1, WIDTH - w);
    let y = rng.range(1, HEIGHT - h);

    let room_rect = Rect::with_size(x, y, w, h);
    let mut ok = true;
    room_rect.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        if map.tiles[idx].tile_type != TileType::Wall {
            ok = false;
        }
    });
    if ok {
        rooms.push(room_rect);
    }
}

fn apply_horizontal_tunnel(map: &mut Layer, x1: i32, x2: i32, y: i32) {
    use std::cmp::{max, min};
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map.point2d_to_index(Point::new(x, y));
        if map.tiles[idx as usize].tile_type == TileType::Wall {
            map.tiles[idx as usize] = Tile::floor();
        }
    }
}

fn apply_vertical_tunnel(map: &mut Layer, y1: i32, y2: i32, x: i32) {
    use std::cmp::{max, min};
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map.point2d_to_index(Point::new(x, y));
        if map.tiles[idx as usize].tile_type == TileType::Wall {
            map.tiles[idx as usize] = Tile::floor();
        }
    }
}
