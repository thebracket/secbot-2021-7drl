use crate::components::*;
use bracket_lib::prelude::{Algorithm2D, a_star_search};
use legion::{*, systems::CommandBuffer};
use crate::map::Map;

pub fn colonists_turn(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);

    let mut colonists = <(Entity, &mut Colonist, &mut ColonistStatus, &mut Position)>::query();
    colonists
        .iter_mut(ecs)
        .filter(|(_, _, status, _)| **status == ColonistStatus::Alive)
        .for_each(|(entity, colonist, status, pos)| {
            // Check basics like "am I dead?"

            // Am I at the exit? If so, I can change my status to "rescued"

            // Am I at a level boundary? If so, go up it!

            // Since I'm activated, I should move towards the exit
            let current_map = map.get_layer(pos.layer as usize);
            if let Some(path) = &mut colonist.path {
                if !path.is_empty() {
                    let next_step = path[0];
                    path.remove(0);
                    if !current_map.tiles[next_step].blocked {
                        pos.pt = current_map.index_to_point2d(next_step);
                    }
                } else {
                    // We've arrived - status update
                    if pos.layer == 0 {
                        *status = ColonistStatus::Rescued;
                        commands.remove_component::<Glyph>(*entity);
                        commands.remove_component::<Description>(*entity);
                    }
                }
            } else {
                let start = current_map.point2d_to_index(pos.pt);
                let end = current_map.point2d_to_index(current_map.colonist_exit);
                let finder = a_star_search(start, end, current_map);
                if finder.success {
                    colonist.path = Some(finder.steps);
                } else {
                    println!("Failed to find the path");
                }

            }
        }
    );

    // Execute the command buffer
    commands.flush(ecs);
}