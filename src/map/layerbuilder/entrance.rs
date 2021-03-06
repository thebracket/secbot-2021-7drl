use super::{all_space, colonists::*, edge_filler, props::*, spawn_face_eater, spawn_quill_worm};
use crate::{
    components::*,
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
        Description("A window. It doesn't look fun outside.".to_string()),
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle - 2, BOTTOM + 1), 0),
        Description("A window. It doesn't look fun outside.".to_string()),
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle + 2, TOP - 1), 0),
        Description("A window. It doesn't look fun outside.".to_string()),
    ));
    ecs.push((
        Position::with_pt(Point::new(x_middle + 2, BOTTOM + 1), 0),
        Description("A window. It doesn't look fun outside.".to_string()),
    ));

    // Spawn the game exit
    add_game_exit(map, ecs, Point::new(LEFT - 1, MIDDLE));

    // Start adding in building complex features
    add_door(map, ecs, Point::new(RIGHT + 1, MIDDLE));
    let start_room = add_entryway(map, ecs, Point::new(RIGHT + 1, MIDDLE));
    let mut rooms = vec![start_room];
    while rooms.len() < 12 {
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

    // Smooth the walls
    super::smooth_walls(map);

    // Outside is revealed - you saw it while landing
    for (i, t) in map.tiles.iter_mut().enumerate() {
        if t.tile_type == TileType::Outside {
            map.revealed[i] = true;
        }
    }

    map.starting_point = Point::new(LEFT + 1, MIDDLE);
}

fn add_game_exit(map: &mut Layer, ecs: &mut World, pt: Point) {
    let exit_idx = map.point2d_to_index(pt);
    map.tiles[exit_idx] = Tile::game_over();
    map.colonist_exit = pt;

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
    map.tiles[idx].glyph = to_cp437('+');
    map.tiles[idx].color.fg = CYAN.into();
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
                            Description(
                                "A window. Not sure who thought that was a good idea.".to_string(),
                            ),
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

    // The first room always contains a single colonist, who must be alive.
    entryway(&rooms[0], map, ecs, rng);

    // Each room after that can be random. This is an initial, very boring spawn to get
    // the colonist functionality going.
    let mut room_types = Vec::new();
    for i in 0..MAX_ROOM_TYPES {
        room_types.push(i);
    }
    let stairs = map.find_down_stairs();
    rooms.iter().skip(1).for_each(|r| {
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

////////// Room Definitions

fn get_random_point(points: &mut Vec<Point>, rng: &mut RandomNumberGenerator) -> Point {
    let index = rng.random_slice_index(&points).unwrap();
    let result = points[index];
    points.remove(index);
    result
}

fn entryway(room: &Rect, map: &mut Layer, ecs: &mut World, rng: &mut RandomNumberGenerator) {
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });

    // Spawn the colonist who greets you
    spawn_first_colonist(ecs, get_random_point(&mut open_space, rng), 0);
    spawn_explosive_barrel(ecs, get_random_point(&mut open_space, rng), 0);
    spawn_explosive_barrel(ecs, get_random_point(&mut open_space, rng), 0);
    spawn_soda_machine(ecs, get_random_point(&mut open_space, rng), 0);
    spawn_snack_machine(ecs, get_random_point(&mut open_space, rng), 0);
    spawn_greeter(ecs, get_random_point(&mut open_space, rng), 0);
    for _ in 0..10 {
        let point = get_random_point(&mut open_space, rng);
        if open_space.contains(&(point + Point::new(1, 0))) {
            spawn_chair(ecs, point, 0);
            spawn_table(ecs, point + Point::new(1, 0), 0);
        }
    }
}

const MAX_ROOM_TYPES: usize = 12;

fn spawn_room(
    rt: usize,
    room: &Rect,
    map: &mut Layer,
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
) {
    match rt {
        0 => charnel_house(room, map, ecs),
        1 => bedroom(room, map, ecs, rng),
        2 => bedroom_not_so_nice(room, map, ecs, rng),
        3 => charnel_house_with_fe(room, map, ecs),
        4 => hidey_boom(room, ecs),
        5 => med_bay(room, ecs, map),
        7 => hydroponics(room, ecs, map, rng),
        8 => hydroponics(room, ecs, map, rng),
        9 => hydroponic_monstrous(room, ecs, map, rng),
        10 => suicidal_colonist_room(room, ecs),
        11 => hydroponic_ranged_monstrous(room, ecs, map, rng),
        _ => {}
    }
}

fn charnel_house(room: &Rect, map: &mut Layer, ecs: &mut World) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = DARK_RED.into();
    });
    spawn_dead_colonist(ecs, room.center() + Point::new(-1, 0), 0);
    spawn_dead_colonist(ecs, room.center() + Point::new(1, 0), 0);
}

fn charnel_house_with_fe(room: &Rect, map: &mut Layer, ecs: &mut World) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = DARK_RED.into();
    });
    spawn_dead_colonist(ecs, room.center() + Point::new(-1, 0), 0);
    spawn_dead_colonist(ecs, room.center() + Point::new(1, 0), 0);
    spawn_face_eater(ecs, room.center(), 0);
}

fn bedroom(room: &Rect, map: &mut Layer, ecs: &mut World, rng: &mut RandomNumberGenerator) {
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });
    let pt = get_random_point(&mut open_space, rng);
    spawn_napping_colonist(ecs, pt, 0);
    spawn_bed(ecs, pt, 0);
}

fn bedroom_not_so_nice(
    room: &Rect,
    map: &mut Layer,
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
) {
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });
    let pt = get_random_point(&mut open_space, rng);
    spawn_napping_colonist(ecs, pt, 0);
    spawn_bed(ecs, pt, 0);
    spawn_face_eater(ecs, Point::new(room.x1, room.y1), 0);
    spawn_face_eater(ecs, Point::new(room.x2, room.y1), 0);
    spawn_face_eater(ecs, Point::new(room.x1, room.y2), 0);
}

fn hidey_boom(room: &Rect, ecs: &mut World) {
    room.for_each(|pt| {
        if pt != room.center() {
            spawn_explosive_barrel(ecs, pt, 0);
        }
    });
    spawn_hiding_colonist(ecs, room.center(), 0);
}

fn med_bay(room: &Rect, ecs: &mut World, map: &mut Layer) {
    let c = room.center();
    let idx = map.point2d_to_index(c);
    map.tiles[idx] = Tile::healing();
    spawn_random_colonist(ecs, c + Point::new(1, 0), 0);
    ecs.push((
        Position::with_pt(c, 0),
        Description("This auto-doc loves healing SecBots!".to_string()),
        TileTrigger(crate::components::TriggerType::Healing),
    ));
}

fn hydroponics(room: &Rect, ecs: &mut World, map: &mut Layer, rng: &mut RandomNumberGenerator) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = GREEN.into();
    });
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });
    for _ in 0..10 {
        if !open_space.is_empty() {
            let pt = get_random_point(&mut open_space, rng);
            spawn_tree(ecs, pt, 0);
        }
    }
}

fn hydroponic_monstrous(
    room: &Rect,
    ecs: &mut World,
    map: &mut Layer,
    rng: &mut RandomNumberGenerator,
) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = RED.into();
    });
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });
    for _ in 0..10 {
        if !open_space.is_empty() {
            let pt = get_random_point(&mut open_space, rng);
            if rng.range(1, 10) < 5 {
                spawn_tree(ecs, pt, 0)
            } else {
                spawn_face_eater(ecs, pt, 0);
            }
        }
    }
}

fn hydroponic_ranged_monstrous(
    room: &Rect,
    ecs: &mut World,
    map: &mut Layer,
    rng: &mut RandomNumberGenerator,
) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx].color.fg = RED.into();
    });
    let mut open_space = Vec::new();
    room.for_each(|p| {
        if p != map.starting_point {
            open_space.push(p)
        }
    });
    let mut n = 0;
    for _ in 0..10 {
        if !open_space.is_empty() {
            let pt = get_random_point(&mut open_space, rng);
            if rng.range(1, 10) < 5 {
                spawn_tree(ecs, pt, 0)
            } else {
                if n < 2 {
                    spawn_quill_worm(ecs, pt, 0);
                    n += 1;
                }
            }
        }
    }
}

fn suicidal_colonist_room(room: &Rect, ecs: &mut World) {
    let c = room.center();
    spawn_suicidal_colonist(ecs, c, 0);
    spawn_face_eater(ecs, Point::new(room.x1, room.y1), 0);
    spawn_face_eater(ecs, Point::new(room.x2, room.y1), 0);
    spawn_face_eater(ecs, Point::new(room.x1, room.y2), 0);
    spawn_face_eater(ecs, Point::new(room.x2, room.y2), 0);
}
