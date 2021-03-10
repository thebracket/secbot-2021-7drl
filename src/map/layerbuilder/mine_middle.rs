use super::{
    all_wall, colonists::*, edge_filler, monsters::*, props::*,
};
use crate::{
    map::{tile::TileType, Layer, Tile, HEIGHT, TILES, WIDTH},
};
use bracket_lib::prelude::*;
use legion::*;

pub fn build_mine_middle(ecs: &mut World) -> Layer {
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

    // Start using drunkard's walk to dig outwards
    while layer
        .tiles
        .iter()
        .filter(|t| t.tile_type == TileType::Floor)
        .count()
        < TILES / 3
    {
        drunkard(&mut layer);
    }

    edge_filler(&mut layer);
    super::smooth_walls(&mut layer);

    // Go with a simple approach for now
    let mut n = 0;
    while n < 30 {
        let mut rng_lock = crate::RNG.lock();
        let mut rng = rng_lock.as_mut().unwrap();
        let pt = Point::new(rng.range(0, WIDTH), rng.range(0, HEIGHT));
        let idx = layer.point2d_to_index(pt);
        let d = DistanceAlg::Pythagoras.distance2d(center_pt, pt);
        if layer.tiles[idx].tile_type == TileType::Floor && d > 12.0 {
            n += 1;
            match rng.range(0, 8) {
                0 => spawn_random_colonist(ecs, pt, 2),
                1 => spawn_marine_colonist(ecs, pt, 2, &mut rng),
                2 => spawn_explosive_barrel(ecs, pt, 2),
                3 => spawn_dead_colonist(ecs, pt, 2),
                4 => spawn_face_eater(ecs, pt, 2),
                5 => spawn_xeno_egg(ecs, pt, 2, rng.roll_dice(1, 6)),
                6 => spawn_quill_worm(ecs, pt, 2),
                7 => spawn_xenomorph(ecs,pt, 2),
                _ => {}
            }
        }
    }

    layer
}

fn drunkard(map: &mut Layer) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    let possible_starts: Vec<usize> = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| t.tile_type == TileType::Floor)
        .map(|(i, _)| i)
        .collect();

    let start = rng.random_slice_entry(&possible_starts).unwrap();
    let mut drunkard_pos = map.index_to_point2d(*start);
    let mut distance_staggered = 0;

    loop {
        let drunk_idx = map.point2d_to_index(drunkard_pos);
        if map.tiles[drunk_idx].tile_type == TileType::Wall {
            map.tiles[drunk_idx] = Tile::floor();
        }

        match rng.range(0, 4) {
            0 => drunkard_pos.x -= 1,
            1 => drunkard_pos.x += 1,
            2 => drunkard_pos.y -= 1,
            _ => drunkard_pos.y += 1,
        }
        if !map.in_bounds(drunkard_pos) {
            break;
        }

        distance_staggered += 1;
        if distance_staggered > 200 {
            break;
        }
    }
}
