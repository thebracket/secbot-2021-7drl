use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::{a_star_search, Algorithm2D};
use legion::{systems::CommandBuffer, *};
use bracket_lib::prelude::*;

pub fn monsters_turn(ecs: &mut World, map: &mut Map) {
    let mut targets : Vec<(Position, Entity)> = <(Entity, &Colonist, &Health, &Position)>::query()
        .iter(ecs)
        .map(|(e, _, _, pos)| (*pos, *e))
        .collect();
    let ppos = <(Entity, &Player, &Position)>::query()
        .iter(ecs)
        .map(|(e, _, pos)| (*pos, *e))
        .nth(0)
        .unwrap();
    targets.push(ppos.clone());

    let mut commands = CommandBuffer::new(ecs);
    let mut melee_buffer = Vec::<(Entity, Entity, i32)>::new();
    <(Entity, &Active, &Hostile, &Position, &mut FieldOfView)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, _, hostile, pos, fov)| {
            let mut attacked = false;
            // What can I see?
            fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_layer(pos.layer as usize));
            let mut target_subset : Vec<(Point, f32, Entity)> = targets
                .iter()
                .filter(|p| p.0.layer == pos.layer)
                .map(|p| (
                    p.0.pt, 
                    DistanceAlg::Pythagoras.distance2d(pos.pt, p.0.pt),
                    p.1,
                )
                )
                .collect();
            target_subset.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            if !target_subset.is_empty() {
                // Can I melee?
                // If so, is there anything within reach?
                if target_subset[0].1 < 1.4 && !hostile.melee.is_empty() {
                    attacked = true;
                    hostile.melee.iter().for_each(|m| {
                        melee_buffer.push((*entity, target_subset[0].2, m.damage));
                    });
                }

                // Can I shoot?
                // If so, is there anything in range?
                hostile.ranged.iter().for_each(|ranged| {
                    // Fire!
                });
            }

            // What's my aggro target?
            // Movement
            if !attacked {
                // What's my aggro target?
                match hostile.aggro {
                    AggroMode::Player => {
                        if ppos.0.layer != pos.layer {
                            // TODO: Path a staircase! For now, sit there.
                        } else {
                            // Path at the player
                            let start = map.get_current().point2d_to_index(pos.pt);
                            let end = map.get_current().point2d_to_index(ppos.0.pt);
                            let path = a_star_search(start, end, map.get_layer(pos.layer as usize));
                            if path.success && path.steps.len() > 2 {
                                let next = map.get_current().index_to_point2d(path.steps[1]);
                                commands.add_component(*entity, Position::with_pt(next, pos.layer));
                            }
                        }
                    }
                    AggroMode::Nearest => {

                    }
                }
            }

            // If its the player, follow them
            // If its nearest, look for something to kill
        }
    );
    commands.flush(ecs);

    melee_buffer.iter().for_each(|(a,d,dmg)| {
        super::combat::melee(ecs, map, *a, *d, *dmg);
    });
}