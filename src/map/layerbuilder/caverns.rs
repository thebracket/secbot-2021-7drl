use super::{
    edge_filler_lava,
};
use crate::{
    map::{tile::TileType, Layer, Tile, HEIGHT, WIDTH},
};
use bracket_lib::prelude::*;
use legion::*;

pub fn build_caverns(ecs: &mut World) -> Layer {
    let mut layer = Layer::new(std::usize::MAX, ecs); // Gets a default layer
                                                      // We're using Cellular Automata here, straight out of Hands-On Rust.
    random_noise_map(&mut layer);
    for _ in 0..15 {
        iteration(&mut layer);
    }
    edge_filler_lava(&mut layer);

    let desired_start = Point::new(2, HEIGHT / 2);
    let mut possible_starts: Vec<(usize, f32)> = layer
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| t.tile_type == TileType::Floor)
        .map(|(idx, _)| {
            (
                idx,
                DistanceAlg::Pythagoras.distance2d(desired_start, layer.index_to_point2d(idx)),
            )
        })
        .collect();
    possible_starts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    layer.starting_point = layer.index_to_point2d(possible_starts[0].0);
    layer.colonist_exit = layer.starting_point;
    layer.tiles[possible_starts[0].0] = Tile::stairs_up();

    layer
}

fn random_noise_map(map: &mut Layer) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    map.tiles.iter_mut().for_each(|t| {
        let roll = rng.range(0, 100);
        if roll > 55 {
            *t = Tile::floor();
        } else {
            *t = Tile::lava();
        }
    });
}

fn count_neighbours(map: &Layer, x: i32, y: i32) -> usize {
    let mut neighbors = 0;
    for iy in -1..=1 {
        for ix in -1..=1 {
            let idx = map.point2d_to_index(Point::new(x + ix, y + iy));
            if !(ix == 0 && iy == 0) && map.tiles[idx].tile_type == TileType::Wall {
                neighbors += 1;
            }
        }
    }
    neighbors
}

fn iteration(map: &mut Layer) {
    let mut new_tiles = map.tiles.clone();
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let neighbors = count_neighbours(map, x as i32, y as i32);
            let idx = map.point2d_to_index(Point::new(x, y));
            if neighbors > 4 || neighbors == 0 {
                new_tiles[idx] = Tile::lava();
            } else {
                new_tiles[idx] = Tile::floor();
            }
        }
    }
    map.tiles = new_tiles;
}
