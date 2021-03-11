use crate::components::*;
use crate::map::Map;
use bracket_lib::{
    prelude::{a_star_search, field_of_view_set, Algorithm2D},
    random::RandomNumberGenerator,
};
use legion::{systems::CommandBuffer, *};

pub fn colonists_turn(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);

    let mut ranged_buffer = Vec::<(Entity, Entity, i32)>::new();

    let mut colonists = <(Entity, &Colonist, &ColonistStatus, &Position, &Active)>::query();
    colonists
        .iter(ecs)
        .filter(|(_, _, status, _, _)| **status == ColonistStatus::Alive)
        .for_each(|(entity, colonist, _, pos, _)| {
            let mut should_move = true;

            // Check basics like "am I dead?"
            if let Ok(er) = ecs.entry_ref(*entity) {
                if let Ok(health) = er.get_component::<Health>() {
                    if health.current < 1 {
                        commands.add_component(*entity, ColonistStatus::DiedAfterStart);
                        commands.remove_component::<Active>(*entity);
                        return;
                    }
                } else {
                    commands.add_component(*entity, ColonistStatus::DiedAfterStart);
                    commands.remove_component::<Active>(*entity);
                    return;
                }
            }

            // Am I at the exit? If so, I can change my status to "rescued"
            // Am I at a level boundary? If so, go up it!
            if pos.pt == map.get_layer(pos.layer as usize).colonist_exit {
                should_move = false;
                if pos.layer == 0 {
                    commands.add_component(*entity, ColonistStatus::Rescued);
                    commands.remove_component::<Glyph>(*entity);
                    commands.remove_component::<Description>(*entity);
                } else {
                    commands.add_component(
                        *entity,
                        Position::with_pt(
                            map.get_layer(pos.layer as usize).find_down_stairs(),
                            pos.layer - 1,
                        ),
                    );
                    commands.add_component(
                        *entity,
                        Colonist {
                            path: None,
                            weapon: colonist.weapon,
                        },
                    );
                }
            }

            // Should I try and be a hero?
            let mut rng = RandomNumberGenerator::new();
            if let Some(damage) = colonist.weapon {
                if rng.range(0, 10) < 5 {
                    let visible_tiles = field_of_view_set(pos.pt, 8, map.get_current());
                    if !visible_tiles.is_empty() {
                        let targets = <(Entity, &Position, &Hostile, &Active, &Health)>::query()
                            .iter(ecs)
                            .filter(|(_, pos, _, _, _)| {
                                pos.layer == map.current_layer as u32
                                    && visible_tiles.contains(&pos.pt)
                            })
                            .map(|(e, _, _, _, _)| *e)
                            .collect::<Vec<Entity>>();
                        if !targets.is_empty() {
                            should_move = false;
                            let target = rng.random_slice_entry(&targets).unwrap();
                            ranged_buffer.push((*entity, *target, damage));
                        }
                    }
                }
            }

            // Since I'm activated, I should move towards the exit
            if should_move {
                let current_map = map.get_layer(pos.layer as usize);
                if let Some(path) = &colonist.path {
                    if !path.is_empty() {
                        let next_step = path[0];
                        let mut new_path = path.clone();
                        new_path.remove(0);
                        commands.add_component(
                            *entity,
                            Colonist {
                                path: Some(new_path),
                                weapon: colonist.weapon,
                            },
                        );
                        let mut new_pos = pos.clone();
                        new_pos.pt = current_map.index_to_point2d(next_step);
                        commands.add_component(*entity, new_pos);
                    }
                } else {
                    let start = current_map.point2d_to_index(pos.pt);
                    let end = current_map.point2d_to_index(current_map.colonist_exit);
                    let finder = a_star_search(start, end, current_map);
                    if finder.success {
                        commands.add_component(
                            *entity,
                            Colonist {
                                path: Some(finder.steps),
                                weapon: colonist.weapon,
                            },
                        );
                    } else {
                        //println!("Failed to find the path");
                    }
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
