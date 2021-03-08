use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::{a_star_search, Algorithm2D};
use legion::{systems::CommandBuffer, *};

pub fn monsters_turn(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);
    let current_layer = map.current_layer;
    <(Entity, &Active, &Hostile, &Position)>::query()
        .iter(ecs)
        .for_each(|(entity, _, hostile, pos)| {
            // Can I melee?
            // If so, is there anything within reach?

            // Can I shoot?
            // If so, is there anything in range?

            // What's my aggro target?
            // If its the player, follow them
            // If its nearest, look for something to kill
        }
    );
    commands.flush(ecs);
}