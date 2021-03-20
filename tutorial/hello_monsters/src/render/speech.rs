use crate::{components::*, map::Map, map::WIDTH};
use bracket_lib::prelude::*;
use legion::*;

pub fn render_speech(ctx: &mut BTerm, ecs: &mut World, map: &Map) {
    let mut commands = legion::systems::CommandBuffer::new(ecs);
    let mut query = <(Entity, &mut Speech, &Position, &Description)>::query();
    query.for_each_mut(ecs, |(entity, speech, pos, desc)| {
        if pos.layer == map.current_layer as u32 {
            let x = if pos.pt.x < WIDTH as i32 / 2 {
                pos.pt.x - 1
            } else {
                pos.pt.x + 1
            };

            ctx.print_color(x, pos.pt.y - 2, GREEN, BLACK, &desc.0);

            speech.lifetime -= 1;
            if speech.lifetime == 0 {
                commands.remove(*entity);
            }
        }
    });
    commands.flush(ecs);
}
