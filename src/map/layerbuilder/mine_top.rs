use super::{all_wall, colonists::*, edge_filler, monsters::*};
use crate::{
    components::*,
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

            if x == center_pt.x || x == center_pt.x + 1 || x == center_pt.x - 1 {
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

    super::smooth_walls(&mut layer);

    // Start by building the melee that greets your arrival
    for x in center_pt.x - 6..=center_pt.x + 6 {
        if rng.range(0, 4) == 0 {
            spawn_quill_worm(ecs, Point::new(x, center_pt.y - 10), 1);
            spawn_quill_worm(ecs, Point::new(x, center_pt.y + 10), 1);
        }
        if rng.range(0, 3) == 0 {
            spawn_face_eater(ecs, Point::new(x, center_pt.y - 9), 1);
            spawn_face_eater(ecs, Point::new(x, center_pt.y + 9), 1);
        }
    }

    // Spawn the defense squads
    for x in center_pt.x - 1..=center_pt.x + 1 {
        spawn_marine_colonist(ecs, Point::new(x, center_pt.y - 5), 1, rng);
        spawn_marine_colonist(ecs, Point::new(x, center_pt.y + 5), 1, rng);
    }
    spawn_marine_leader(ecs, Point::new(center_pt.x, center_pt.y - 2), 1);

    // Room-based population
    populate_rooms(&rooms, &mut layer, ecs, rng);

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

fn populate_rooms(
    rooms: &Vec<Rect>,
    map: &mut Layer,
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
) {
    // Each room after that can be random. This is an initial, very boring spawn to get
    // the colonist functionality going.
    let mut room_types = Vec::new();
    for i in 0..MAX_ROOM_TYPES {
        room_types.push(i);
    }
    let stairs = map.find_down_stairs();
    rooms.iter().for_each(|r| {
        if !r.point_set().contains(&stairs) {
            if !room_types.is_empty() {
                let room_index = rng.random_slice_index(&room_types).unwrap();
                let ri = room_types[room_index];
                room_types.remove(room_index);
                spawn_room(ri, r, map, ecs, rng);
            } else {
                if rng.range(0, 5) == 0 {
                    spawn_random_colonist(ecs, r.center(), 0);
                } else {
                    spawn_face_eater(ecs, r.center(), 0);
                }
            }
        }
    });
}

fn get_random_point(points: &mut Vec<Point>, rng: &mut RandomNumberGenerator) -> Point {
    let index = rng.random_slice_index(&points).unwrap();
    let result = points[index];
    points.remove(index);
    result
}

const MAX_ROOM_TYPES: usize = 1;

fn spawn_room(
    rt: usize,
    room: &Rect,
    map: &mut Layer,
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
) {
    match rt {
        0 => charnel_house(room, map, ecs, rng),
        _ => {}
    }
}

fn charnel_house(room: &Rect, map: &mut Layer, ecs: &mut World, rng: &mut RandomNumberGenerator) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = DARK_RED.into();
        if rng.range(0, 10) == 0 {
            spawn_dead_colonist(ecs, pt, 1);
        }
    });
}
