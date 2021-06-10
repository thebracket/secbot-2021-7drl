use crate::{components::*, render::tooltips::render_tooltips};
use crate::{
    map::{Map, HEIGHT, WIDTH, TileType},
    NewState,
};
use bracket_lib::prelude::*;
use legion::*;
use legion::systems::CommandBuffer;
use std::collections::HashSet;

pub fn player_turn(ctx: &mut BTerm, ecs: &mut World, map: &mut Map) -> NewState {
    render_tooltips(ctx, ecs, map);

    // Check for input
    let mut new_state = if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move(ecs, map, 0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::A => try_move(ecs, map, 0, 1),
            VirtualKeyCode::Left | VirtualKeyCode::S => try_move(ecs, map, -1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move(ecs, map, 1, 0),
            VirtualKeyCode::T | VirtualKeyCode::Tab => cycle_target(ecs),
            VirtualKeyCode::Comma => go_up(ecs, map),
            VirtualKeyCode::Period => go_down(ecs, map),
            VirtualKeyCode::Space => NewState::Player, // Wait action
            VirtualKeyCode::F => open_fire_at_target(ecs, map),
            _ => NewState::Wait,
        }
    } else {
        NewState::Wait
    };

    // Check for tile trigger effects
    tile_triggers(&mut new_state, ecs, map);
    update_fov(&new_state, ecs, map);

    new_state
}

fn try_move(ecs: &mut World, map: &mut Map, delta_x: i32, delta_y: i32) -> NewState {
    let mut find_player = <(&Player, &mut Position)>::query();
    let mut result = NewState::Wait;
    let mut doors_to_delete = HashSet::new();
    find_player.iter_mut(ecs).for_each(|(_, pos)| {
        let new_pos = pos.pt + Point::new(delta_x, delta_y);
        let new_idx = map.get_current().point2d_to_index(new_pos);
        if !map.get_current().tiles[new_idx].blocked {
            pos.pt = new_pos;
            result = NewState::Enemy;
        } else if map.get_current().is_door[new_idx] {
            map.get_current_mut().is_door[new_idx] = false;
            map.get_current_mut().tiles[new_idx].blocked = false;
            map.get_current_mut().tiles[new_idx].opaque = false;
            map.get_current_mut().tiles[new_idx].glyph = to_cp437('.');
            doors_to_delete.insert(map.get_current().index_to_point2d(new_idx));
        }
    });

    if !doors_to_delete.is_empty() {
        let mut commands = CommandBuffer::new(ecs);
        let mut q = <(Entity, &Position, &Door)>::query();
        q.for_each(ecs, |(entity, pos, _)| {
            if pos.layer == map.current_layer as u32 && doors_to_delete.contains(&pos.pt) {
                commands.remove(*entity);
            }
        });
        commands.flush(ecs);
    }

    result
}

fn go_up(ecs: &mut World, map: &mut Map) -> NewState {
    let mut find_player = <(&Player, &mut Position)>::query();
    find_player.for_each_mut(ecs, |(_, pos)| {
        let idx = map.get_current().point2d_to_index(pos.pt);
        if map.get_current().tiles[idx].tile_type == TileType::StairsUp {
            // It really is an up staircase
            let new_layer = pos.layer - 1;
            map.set_current_layer(new_layer as usize);
            pos.layer = new_layer;
            pos.pt = map.get_current().find_down_stairs();
        }
    });
    NewState::Player
}

fn go_down(ecs: &mut World, map: &mut Map) -> NewState {
    let mut find_player = <(&Player, &mut Position)>::query();
    find_player.for_each_mut(ecs, |(_, pos)| {
        let idx = map.get_current().point2d_to_index(pos.pt);
        if map.get_current().tiles[idx].tile_type == TileType::StairsDown {
            // It really is a down staircase
            let new_layer = pos.layer + 1;
            map.set_current_layer(new_layer as usize);
            pos.layer = new_layer;
            pos.pt = map.get_current().starting_point;
        }
    });
    NewState::Player
}

fn tile_triggers(new_state: &mut NewState, ecs: &mut World, map: &mut Map) {
    if *new_state == NewState::Wait {
        return;
    }
    let mut find_player = <(&Player, &Position)>::query();
    let player_pos = find_player.iter(ecs).map(|(_, pos)| *pos).nth(0).unwrap();

    let mut find_triggers = <(&TileTrigger, &Position)>::query();
    find_triggers
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(tt, _)| match tt.0 {
            TriggerType::EndGame => *new_state = NewState::LeftMap,
        });
}

pub fn update_fov(new_state: &NewState, ecs: &mut World, map: &mut Map) {
    if *new_state == NewState::Wait {
        return;
    }

    let mut visible = None;
    let mut player_pos = Point::zero();
    let mut player_entity = None;

    // Build the player FOV
    let mut query = <(Entity, &Player, &Position, &mut FieldOfView)>::query();
    query.for_each_mut(ecs, |(e, _, pos, fov)| {
        player_pos = pos.pt;
        player_entity = Some(*e);
        fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());
        let current_layer = map.get_current_mut();
        current_layer.clear_visible();
        fov.visible_tiles.iter().for_each(|pt| {
            if current_layer.in_bounds(*pt) {
                let idx = current_layer.point2d_to_index(*pt);
                current_layer.revealed[idx] = true;
                current_layer.visible[idx] = true;
            }
        });
        visible = Some(fov.visible_tiles.clone());
    });

    if let Some(vt) = visible {
        let mut commands = legion::systems::CommandBuffer::new(ecs);
        // Update colonist status
        let mut can_be_activated = <(Entity, &CanBeActivated, &Position)>::query();
        can_be_activated.for_each_mut(ecs, |(entity, _, pos)| {
            if pos.layer == map.current_layer as u32 && vt.contains(&pos.pt) {
                commands.add_component(*entity, Found {});
                if DistanceAlg::Pythagoras.distance2d(player_pos, pos.pt) < 6.0 {
                    commands.remove_component::<CanBeActivated>(*entity);
                    commands.add_component(*entity, Active {});
                }
            }
        });

        // Targeting system
        let mut possible_targets = <(Entity, &Targetable, &Position)>::query();
        let mut targets = possible_targets
            .iter(ecs)
            .filter(|(_, _, pos)| pos.layer == map.current_layer as u32 && vt.contains(&pos.pt))
            .map(|(e, _, pos)| (*e, DistanceAlg::Pythagoras.distance2d(player_pos, pos.pt)))
            .collect::<Vec<(Entity, f32)>>();

        targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let current_target = if targets.is_empty() {
            None
        } else {
            Some(targets[0].0)
        };
        commands.add_component(
            player_entity.unwrap(),
            Targeting {
                targets,
                current_target,
                index: 0,
            },
        );
        commands.flush(ecs);
    }
}

fn cycle_target(ecs: &mut World) -> NewState {
    let mut pq = <(&Player, &mut Targeting)>::query();
    pq.for_each_mut(ecs, |(_, targeting)| {
        if targeting.targets.is_empty() {
            targeting.current_target = None;
        } else {
            targeting.index += 1;
            if targeting.index > targeting.targets.len() - 1 {
                targeting.index = 0;
            }
            targeting.current_target = Some(targeting.targets[targeting.index].0);
        }
    });
    NewState::Wait
}

// Returns (probability, range)
pub fn hit_probability(ecs: &World, target: Entity) -> (u32, u32) {
    let mut target_pos = Point::zero();
    if let Ok(entry) = ecs.entry_ref(target) {
        if let Ok(pos) = entry.get_component::<Position>() {
            target_pos = pos.pt;
        }
    }

    let player_pos = <(&Player, &Position)>::query()
        .iter(ecs)
        .map(|(_, pos)| pos)
        .nth(0)
        .unwrap()
        .pt;

    let range = DistanceAlg::Pythagoras.distance2d(player_pos, target_pos) as u32;

    // TODO: More complexity here
    let mut hit_chance = 90;
    if range > 5 {
        hit_chance -= (range - 5) * 5;
    }

    (hit_chance, range)
}

fn open_fire_at_target(ecs: &mut World, map: &mut Map) -> NewState {
    let mut commands = CommandBuffer::new(ecs);
    let mut player_pos = Point::zero();
    let mut target = None;
    let mut current_layer = map.current_layer as u32;
    <(&Player, &Position, &Targeting)>::query()
        .iter(ecs)
        .for_each(|(_, pos, targeting)| {
            player_pos = pos.pt;
            target = targeting.current_target;
        });

    // If there's nothing to fire at, return to waiting
    if target.is_none() {
        return NewState::Wait;
    }
    let pos_map = <(&Position, &Health)>::query()
        .iter(ecs)
        .map(|(pos, _)| pos.pt)
        .collect::<HashSet<Point>>();
    if let Some(target) = target {
        if let Ok(target_ref) = ecs.entry_ref(target) {
            if let Ok(target_position) = target_ref.get_component::<Position>() {
                let target_pos = target_position.pt;
                let mut power = 20;
                let mut range = 0;
                let mut projectile_path = Vec::new();
                let mut splatter = None;
                line2d_bresenham(player_pos, target_pos)
                    .iter()
                    .skip(1)
                    .for_each(|pt| {
                        projectile_path.push(*pt);
                        if pos_map.contains(&pt) {
                            power -= hit_tile_contents(
                                ecs,
                                *pt,
                                current_layer,
                                &mut commands,
                                &mut splatter,
                            );
                        }
                        if let Some(bsplatter) = &mut splatter {
                            let idx = map.get_current().point2d_to_index(*pt);
                            map.get_current_mut().tiles[idx].color.bg = bsplatter.to_rgba(1.0);
                            bsplatter.r = f32::max(0.0, bsplatter.r - 0.1);
                            bsplatter.g = f32::max(0.0, bsplatter.g - 0.1);
                            bsplatter.b = f32::max(0.0, bsplatter.b - 0.1);
                            if bsplatter.r + bsplatter.g + bsplatter.b < 0.1 {
                                splatter = None;
                            }
                        }
                        range += 1;
                        if range > 5 {
                            power -= 1;
                        }
                    });

                use ultraviolet::Vec2;
                let mut projectile_pos: Vec2 = Vec2::new(target_pos.x as f32, target_pos.y as f32);
                let slope = (projectile_pos - Vec2::new(player_pos.x as f32, player_pos.y as f32))
                    .normalized();
                while range < 25 && power > 0 {
                    projectile_pos += slope;
                    let pt = Point::new(projectile_pos.x as i32, projectile_pos.y as i32);
                    projectile_path.push(pt);
                    if pos_map.contains(&pt) {
                        power -=
                            hit_tile_contents(ecs, pt, current_layer, &mut commands, &mut splatter);
                    }
                    if let Some(bsplatter) = &mut splatter {
                        let idx = map.get_current().point2d_to_index(pt);
                        map.get_current_mut().tiles[idx].color.bg = bsplatter.to_rgba(1.0);
                        bsplatter.r = f32::max(0.0, bsplatter.r - 0.1);
                        bsplatter.g = f32::max(0.0, bsplatter.g - 0.1);
                        bsplatter.b = f32::max(0.0, bsplatter.b - 0.1);
                        if bsplatter.r + bsplatter.g + bsplatter.b < 0.1 {
                            splatter = None;
                        }
                    }
                    let idx = map.get_current().point2d_to_index(pt);
                    if map.get_current().tiles[idx].tile_type == TileType::Wall {
                        range += 100;
                        power = 0;
                    }
                    if !map.get_current().tiles[idx].opaque && power > 5 {
                        // TODO: End the game because you broke a window
                    }
                    range += 1;
                    if range > 5 {
                        power -= 1;
                    }
                }
                commands.push((
                    Projectile {
                        path: projectile_path,
                        layer: current_layer as usize,
                    },
                    Glyph {
                        glyph: to_cp437('*'),
                        color: ColorPair::new(RED, BLACK),
                    },
                ));
            } else {
                // Unable to fire
                return NewState::Wait;
            }
        } else {
            // Unable to fire
            return NewState::Wait;
        }
    }

    commands.flush(ecs);
    NewState::Player
}

fn hit_tile_contents(
    ecs: &mut World,
    pt: Point,
    layer: u32,
    commands: &mut CommandBuffer,
    splatter: &mut Option<RGB>,
) -> i32 {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();
    let mut power_loss = 0;
    let mut dead_entities = Vec::new();
    <(Entity, &Position, &mut Health)>::query()
        .iter_mut(ecs)
        .filter(|(_, pos, _)| pos.layer == layer && pos.pt == pt)
        .for_each(|(entity, _, hp)| {
            let damage = rng.range(1, 5) + 10; // TODO: Complexity, please
            hp.current -= damage;
            if hp.current < 0 {
                hp.current = 0;
                dead_entities.push(*entity);
            }
            power_loss += hp.current;
        });
    dead_entities.iter().for_each(|entity| {
        if let Ok(mut er) = ecs.entry_mut(*entity) {
            if let Ok(_colonist) = er.get_component_mut::<ColonistStatus>() {
                commands.add_component(*entity, ColonistStatus::DiedAfterStart);
            }
            if let Ok(g) = er.get_component_mut::<Glyph>() {
                g.color.bg = DARK_RED.into();
                g.color.fg = DARK_GRAY.into();
            }
            if let Ok(n) = er.get_component_mut::<Name>() {
                n.0 = format!("Corpse: {}", n.0);
            }
            if let Ok(b) = er.get_component::<Blood>() {
                *splatter = Some(b.0);
            }
        }
        commands.remove_component::<Health>(*entity);
        commands.remove_component::<Active>(*entity);
        commands.remove_component::<CanBeActivated>(*entity);
        commands.remove_component::<Blood>(*entity);
        commands.remove_component::<Targetable>(*entity);
    });

    power_loss
}