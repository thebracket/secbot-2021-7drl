use crate::components::*;
use crate::map::Map;
use legion::systems::CommandBuffer;
use legion::*;

pub fn manage_event_timers(ecs: &mut World, _map: &Map) {
    let mut commands = CommandBuffer::new(ecs);

    <(Entity, &mut TimedEvent, &Position, &Active)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, timer, pos, _)| {
            timer.timer -= 1;
            if timer.timer == 0 {
                // Delete the entity if it's concluded its timer
                commands.remove(*entity);

                // Create an explosion (TODO: Conditional if we need more timers)
                commands.push((Position::with_pt(pos.pt, pos.layer), Boom { range: 3 }));
            } else {
                commands.push((
                    Speech { lifetime: 40 },
                    pos.clone(),
                    Description(format!("Timer: {}", timer.timer)),
                ));
            }
        });

    commands.flush(ecs);
}
