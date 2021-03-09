use super::{combat::player_open_fire_at_target, targeting_weight};
use crate::{components::*, render::tooltips::render_tooltips};
use crate::{map::Map, map::TileType, NewState};
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;
use std::collections::HashSet;

pub fn player_turn(ctx: &mut BTerm, ecs: &mut World, map: &mut Map) -> NewState {
    render_tooltips(ctx, ecs, map);

    // Check for input
    let mut new_state = if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move(ecs, map, 0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S => try_move(ecs, map, 0, 1),
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move(ecs, map, -1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move(ecs, map, 1, 0),
            VirtualKeyCode::T | VirtualKeyCode::Tab => cycle_target(ecs),
            VirtualKeyCode::Comma => go_up(ecs, map),
            VirtualKeyCode::Period => go_down(ecs, map),
            VirtualKeyCode::Space => NewState::Player, // Wait action
            VirtualKeyCode::F => player_open_fire_at_target(ecs, map),
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
            result = NewState::Player;
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

fn tile_triggers(new_state: &mut NewState, ecs: &mut World, _map: &mut Map) {
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
            .map(|(e, _, pos)| {
                (
                    *e,
                    DistanceAlg::Pythagoras.distance2d(player_pos, pos.pt)
                        + targeting_weight(*e, ecs),
                )
            })
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
