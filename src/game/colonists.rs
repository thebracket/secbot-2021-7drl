use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::{a_star_search, Algorithm2D};
use legion::{systems::CommandBuffer, *};

pub fn colonists_turn(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);

    let mut colonists = <(
        Entity,
        &mut Colonist,
        &mut ColonistStatus,
        &mut Position,
        &mut Dialog,
        &Active,
    )>::query();
    colonists
        .iter_mut(ecs)
        .filter(|(_, _, status, _, _, _)| **status == ColonistStatus::Alive)
        .for_each(|(entity, colonist, status, pos, dialog, _)| {
            let mut should_move = true;

            // Check basics like "am I dead?"

            // Am I at the exit? If so, I can change my status to "rescued"
            // Am I at a level boundary? If so, go up it!
            if pos.pt == map.get_layer(pos.layer as usize).colonist_exit {
                should_move = false;
                if pos.layer == 0 {
                    *status = ColonistStatus::Rescued;
                    commands.remove_component::<Glyph>(*entity);
                    commands.remove_component::<Description>(*entity);
                } else {
                    pos.layer = pos.layer - 1;
                    pos.pt = map.get_layer(pos.layer as usize).find_down_stairs();
                    colonist.path = None;
                }
            }

            // Since I'm activated, I should move towards the exit
            if should_move {
                let current_map = map.get_layer(pos.layer as usize);
                if let Some(path) = &mut colonist.path {
                    if !path.is_empty() {
                        let next_step = path[0];
                        path.remove(0);
                        pos.pt = current_map.index_to_point2d(next_step);
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

                // If the actor has dialogue, emit it
                if !dialog.lines.is_empty() {
                    let line = dialog.lines[0].clone();
                    dialog.lines.remove(0);
                    commands.push((Speech { lifetime: 20 }, pos.clone(), Description(line)));
                }
            }
        });

    // Execute the command buffer
    commands.flush(ecs);
}
