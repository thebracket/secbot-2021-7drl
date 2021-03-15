use crate::{components::*, render::tooltips::render_tooltips};
use crate::{
    map::{Map, HEIGHT, WIDTH},
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

fn tile_triggers(new_state: &mut NewState, ecs: &mut World, map: &mut Map) {
    if *new_state != NewState::Wait {
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

fn update_fov(new_state: &NewState, ecs: &mut World, map: &mut Map) {
    if *new_state != NewState::Wait {
        return;
    }

    let mut query = <(&Player, &Position, &mut FieldOfView)>::query();
    query.for_each_mut(ecs, |(_, pos, fov)| {
        fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());
        let current_layer = map.get_current_mut();
        current_layer.clear_visible();
        fov.visible_tiles.iter().for_each(|pt| {
            let idx = current_layer.point2d_to_index(*pt);
            current_layer.revealed[idx] = true;
            current_layer.visible[idx] = true;
        });
    });

}