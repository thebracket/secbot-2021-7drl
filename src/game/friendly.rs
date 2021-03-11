use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::{a_star_search, field_of_view_set, Algorithm2D, DistanceAlg};
use legion::{systems::CommandBuffer, *};

pub fn friendly_turn(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);

    let mut ranged_buffer = Vec::<(Entity, Entity, i32)>::new();

    let mut colonists = <(Entity, &Friendly, &Position, &Active)>::query();
    colonists.iter(ecs).for_each(|(entity, _, pos, _)| {
        let mut should_move = true;

        // Open fire, all weapons!
        let damage = 10; // Yes, I'm ignoring the ECS. In a hurry.
        let visible_tiles = field_of_view_set(pos.pt, 8, map.get_current());
        if !visible_tiles.is_empty() {
            let mut targets = <(Entity, &Position, &Hostile, &Active, &Health)>::query()
                .iter(ecs)
                .filter(|(_, pos, _, _, _)| {
                    pos.layer == map.current_layer as u32 && visible_tiles.contains(&pos.pt)
                })
                .map(|(e, epos, _, _, _)| (*e, DistanceAlg::Pythagoras.distance2d(pos.pt, epos.pt)))
                .collect::<Vec<(Entity, f32)>>();
            if !targets.is_empty() {
                should_move = false;
                targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let target = targets[0].0;
                ranged_buffer.push((*entity, target, damage));

                commands.push((
                    Speech { lifetime: 300 },
                    Description("Fire!".to_string()),
                    pos.clone(),
                ));
            }
        }

        // Find the Queen
        if should_move {
            let qpos = <(&Hostile, &Name, &Position)>::query()
                .iter(ecs)
                .filter(|(_, name, _)| name.0 == "Alien Queen")
                .map(|(_, _, pos)| pos.pt)
                .nth(0);

            if let Some(qpos) = qpos {
                let start = map.get_current().point2d_to_index(pos.pt);
                let end = map.get_current().point2d_to_index(qpos);
                let path = a_star_search(start, end, map.get_current());
                if path.success && !path.steps.len() > 1 {
                    let mut new_pos = pos.clone();
                    new_pos.pt = map.get_current().index_to_point2d(path.steps[1]);
                    commands.add_component(*entity, new_pos);

                    if path.steps.len() == 15 {
                        commands.push((
                            Speech { lifetime: 1000 },
                            Description("Gear up, we're getting close.".to_string()),
                            pos.clone(),
                        ));
                    }
                }
            } else {
                commands.push((
                    Speech { lifetime: 1000 },
                    Description("The Queen is Dead. Save the colonists.".to_string()),
                    pos.clone(),
                ));
            }
        }
    });

    // Execute the command buffer
    commands.flush(ecs);

    // Fire missiles!
    ranged_buffer.iter().for_each(|(a, d, dmg)| {
        super::combat::ranged_attack(ecs, map, *a, *d, *dmg);
    });
}
