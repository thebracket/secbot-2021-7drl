use super::all_space;
use crate::{
    components::{Description, Position, TileTrigger},
    map::{Layer, Tile, HEIGHT, TILES, WIDTH},
};
use bracket_lib::prelude::*;
use legion::*;

pub fn build_entrance(ecs: &mut World) -> Layer {
    let mut layer = Layer::new(std::usize::MAX, ecs); // Gets a default layer

    all_space(&mut layer);
    add_landscape(&mut layer, ecs);
    add_docking_capsule(&mut layer, ecs);

    layer
}

fn add_docking_capsule(map: &mut Layer, ecs: &mut World) {
    const MIDDLE: usize = HEIGHT / 2;
    const TOP: usize = MIDDLE - 3;
    const BOTTOM: usize = MIDDLE + 3;
    const LEFT: usize = 1;
    const RIGHT: usize = 8;

    for y in TOP..=BOTTOM {
        for x in LEFT..=RIGHT {
            let idx = map.point2d_to_index(Point::new(x, y));
            map.tiles[idx] = Tile::capsule_floor();
        }
    }

    // Spawn the game exit
    add_game_exit(map, ecs, Point::new(LEFT - 1, MIDDLE));

    map.starting_point = Point::new(LEFT + 1, MIDDLE);
}

fn add_game_exit(map: &mut Layer, ecs: &mut World, pt: Point) {
    let exit_idx = map.point2d_to_index(pt);
    map.tiles[exit_idx] = Tile::game_over();

    ecs.push((
        Position::with_pt(pt, 0),
        Description(
            "Exit to SecBot's Ship. Leave through here when you are ready to call it game over."
                .to_string(),
        ),
        TileTrigger(crate::components::TriggerType::EndGame),
    ));
}

fn add_landscape(map: &mut Layer, ecs: &mut World) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();
    let mut noise = FastNoise::seeded(rng.next_u64());
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(10);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(3.5);
    noise.set_frequency(0.02);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let h = noise.get_noise(x as f32, y as f32);
            let idx = map.point2d_to_index(Point::new(x, y));
            map.tiles[idx] = Tile::alien_landscape(h);
        }
    }
}