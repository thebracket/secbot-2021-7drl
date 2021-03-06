use super::{all_space, spawn_random_colonist};
use crate::{
    components::{Description, Door, Glyph, Position, TileTrigger},
    map::{tile::TileType, Layer, Tile, HEIGHT, WIDTH},
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

    // Floor
    for y in TOP..=BOTTOM {
        for x in LEFT..=RIGHT {
            let idx = map.point2d_to_index(Point::new(x, y));
            map.tiles[idx] = Tile::capsule_floor();
        }
    }

    // Encasing Walls
    for x in LEFT - 1..=RIGHT + 1 {
        let idx = map.point2d_to_index(Point::new(x, TOP - 1));
        map.tiles[idx] = Tile::capsule_wall();
        let idx = map.point2d_to_index(Point::new(x, BOTTOM + 1));
        map.tiles[idx] = Tile::capsule_wall();
    }
    for y in TOP - 1..=BOTTOM + 1 {
        let idx = map.point2d_to_index(Point::new(LEFT - 1, y));
        map.tiles[idx] = Tile::capsule_wall();
        let idx = map.point2d_to_index(Point::new(RIGHT + 1, y));
        map.tiles[idx] = Tile::capsule_wall();
    }

    // Add some windows
    let x_middle = (LEFT + RIGHT) / 2;
    let idx = map.point2d_to_index(Point::new(x_middle - 2, TOP - 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle - 2, BOTTOM + 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle + 2, TOP - 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle + 2, BOTTOM + 1));
    map.tiles[idx] = Tile::capsule_window();
    ecs.push((
        Position::with_pt(Point::new(x_middle - 2, TOP - 1), 0),
        Description("A window. It doesn't look fun outside.".to_string())
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle - 2, BOTTOM + 1), 0),
        Description("A window. It doesn't look fun outside.".to_string())
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle + 2, TOP - 1), 0),
        Description("A window. It doesn't look fun outside.".to_string())
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle + 2, BOTTOM + 1), 0),
        Description("A window. It doesn't look fun outside.".to_string())
    ));

    // Spawn the game exit
    add_game_exit(map, ecs, Point::new(LEFT - 1, MIDDLE));

    // Start adding in building complex features
    add_door(map, ecs, Point::new(RIGHT + 1, MIDDLE));
    let start_room = add_entryway(map, ecs, Point::new(RIGHT + 1, MIDDLE));
    let mut rooms = vec![start_room];
    while rooms.len() < 24 {
        try_random_room(map, ecs, &mut rooms);
    }

    // Fill in the edges
    edge_filler(map);

    // Add some exterior windows
    add_windows(map, ecs);

    // Add an exit
    add_exit(&mut rooms, map, ecs);

    // Populate rooms
    populate_rooms(&mut rooms, map, ecs);

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

fn add_landscape(map: &mut Layer, _ecs: &mut World) {
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

fn add_door(map: &mut Layer, ecs: &mut World, pt: Point) {
    let idx = map.point2d_to_index(pt);
    ecs.push((
        Position::with_pt(pt, 0),
        Description("A heavy, steel door.".to_string()),
        Glyph {
            glyph: to_cp437('+'),
            color: ColorPair::new(CYAN, BLACK),
        },
        Door {},
    ));
    map.tiles[idx] = Tile::wall();
    map.is_door[idx] = true;
}

fn add_entryway(map: &mut Layer, _ecs: &mut World, entrance: Point) -> Rect {
    let room = Rect::with_size(entrance.x + 1, entrance.y - 5, 20, 10);
    fill_room(map, &room);

    room
}

fn try_wall(map: &mut Layer, pt: Point) {
    if map.in_bounds(pt) {
        let idx = map.point2d_to_index(pt);
        if !map.is_door[idx] {
            map.tiles[idx] = Tile::wall();
        }
    }
}

fn fill_room(map: &mut Layer, room: &Rect) {
    room.for_each(|pt| {
        if map.in_bounds(pt) {
            let idx = map.point2d_to_index(pt);
            map.tiles[idx] = Tile::floor();
        }
    });
    for x in i32::max(0, room.x1 - 1)..=i32::min(WIDTH as i32 - 1, room.x2 + 1) {
        try_wall(map, Point::new(x, room.y1 - 1));
        try_wall(map, Point::new(x, room.y2 + 1));
    }
    for y in i32::max(room.y1, 0)..=i32::min(room.y2, HEIGHT as i32 - 1) {
        try_wall(map, Point::new(room.x1 - 1, y));
        try_wall(map, Point::new(room.x2 + 1, y));
    }
}

fn try_random_room(map: &mut Layer, ecs: &mut World, rooms: &mut Vec<Rect>) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();
    if let Some(parent_room) = rng.random_slice_entry(&rooms) {
        let x;
        let y;
        let next_x;
        let next_y;

        // Decide where to consider an exit
        if rng.range(0, 2) == 0 {
            // Take from the horizontal walls
            x = parent_room.x1 + rng.range(0, parent_room.width() + 1);
            next_x = x;
            if rng.range(0, 2) == 0 {
                // Take from the north side
                y = parent_room.y1 - 1;
                next_y = y - 1;
            } else {
                // Take from the south side
                y = parent_room.y2 + 1;
                next_y = y + 1;
            }
        } else {
            // Take from the vertical walls
            y = parent_room.y1 + rng.range(0, parent_room.height() + 1);
            next_y = y;
            if rng.range(0, 2) == 0 {
                x = parent_room.x1 - 1;
                next_x = x - 1;
            } else {
                x = parent_room.x2 + 1;
                next_x = x + 1;
            }
        }
        let dx = next_x - x;
        let dy = next_y - y;

        // Try to place it
        let next_pt = Point::new(next_x, next_y);
        if !map.in_bounds(next_pt) {
            return;
        }
        let next_idx = map.point2d_to_index(next_pt);
        if map.tiles[next_idx].tile_type == TileType::Outside {
            let new_room = if dx == 1 {
                Rect::with_size(x + 1, y, rng.range(4, 10), rng.range(3, 6))
            } else if dy == 1 {
                Rect::with_size(x, next_y, rng.range(3, 6), rng.range(4, 10))
            } else if dx == -1 {
                let w = 5;
                Rect::with_size(x - w, y, rng.range(4, 10), rng.range(3, 6))
            } else {
                let h = 5;
                Rect::with_size(x, y - h, rng.range(3, 6), rng.range(4, 10))
            };

            let mut can_add = true;
            new_room.for_each(|p| {
                if map.in_bounds(p) {
                    let idx = map.point2d_to_index(p);
                    if map.tiles[idx].tile_type != TileType::Outside {
                        can_add = false;
                    }
                } else {
                    can_add = false;
                }
            });

            if can_add {
                add_door(map, ecs, Point::new(x, y));
                fill_room(map, &new_room);
                rooms.push(new_room);
            }
        }
    }
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

fn add_windows(map: &mut Layer, ecs: &mut World) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let pt = Point::new(x, y);
            let idx = map.point2d_to_index(pt);
            if map.tiles[idx].tile_type == TileType::Wall {
                if map.tiles[idx - 1].tile_type == TileType::Outside
                    || map.tiles[idx + 1].tile_type == TileType::Outside
                    || map.tiles[idx - WIDTH].tile_type == TileType::Outside
                    || map.tiles[idx - WIDTH].tile_type == TileType::Outside
                {
                    if rng.range(0, 10) == 0 {
                        map.tiles[idx] = Tile::window();
                        ecs.push((
                            Position::with_pt(Point::new(x, y), 0),
                            Description("A window. Not sure who thought that was a good idea.".to_string())
                        ));
                    }
                }
            }
        }
    }
}

fn add_exit(rooms: &mut Vec<Rect>, map: &mut Layer, ecs: &mut World) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();
    let room = rng.random_slice_entry(&rooms).unwrap();
    let exit_location = room.center();
    let idx = map.point2d_to_index(exit_location);
    map.tiles[idx] = Tile::stairs_down();

    ecs.push((
        Position::with_pt(exit_location, 0),
        Description("Stairs further into the complex".to_string()),
    ));
}

fn populate_rooms(rooms: &Vec<Rect>, map: &mut Layer, ecs: &mut World) {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    // The first room always contains a single colonist
    spawn_random_colonist(ecs, rooms[0].center(), 0);

    // Each room after that can be random. This is an initial, very boring spawn to get
    // the colonist functionality going.
    rooms
        .iter()
        .skip(1)
        .for_each(|r| {
            if rng.range(0, 5) == 0 {
                spawn_random_colonist(ecs, r.center(), 0);
            }
        }
    );
}