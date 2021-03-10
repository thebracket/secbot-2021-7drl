use crate::components::*;
use crate::map::*;
use crate::NewState;
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;
use std::collections::HashSet;

pub fn player_open_fire_at_target(ecs: &mut World, map: &mut Map) -> NewState {
    let mut player_entity = None;
    let mut target = None;
    <(Entity, &Player, &Targeting)>::query()
        .iter(ecs)
        .for_each(|(entity, _, targeting)| {
            target = targeting.current_target;
            player_entity = Some(*entity);
        });

    // If there's nothing to fire at, return to waiting
    if target.is_none() {
        return NewState::Wait;
    }

    ranged_attack(ecs, map, player_entity.unwrap(), target.unwrap(), 20);

    NewState::Player
}

pub fn ranged_attack(
    ecs: &mut World,
    map: &mut Map,
    attacker: Entity,
    victim: Entity,
    ranged_power: i32,
) {
    let mut attacker_pos = None;
    let mut victim_pos = None;

    // Find positions for the start and end
    if let Ok(ae) = ecs.entry_ref(attacker) {
        if let Ok(pos) = ae.get_component::<Position>() {
            attacker_pos = Some(pos.clone());
        }
    }
    if let Ok(ae) = ecs.entry_ref(victim) {
        if let Ok(pos) = ae.get_component::<Position>() {
            victim_pos = Some(pos.clone());
        }
    }
    if attacker_pos.is_none() || victim_pos.is_none() {
        return;
    }
    let attacker_pos = attacker_pos.unwrap();
    let victim_pos = victim_pos.unwrap();

    // Set state for the projectile path
    let mut power = ranged_power;
    let mut range = 0;
    let mut projectile_path = Vec::new();
    let mut splatter = None;
    let mut commands = CommandBuffer::new(ecs);
    let current_layer = attacker_pos.layer;

    // Map of entity locations. Rebuilt every time because it might change.
    let pos_map = <(&Position, &Health)>::query()
        .iter(ecs)
        .map(|(pos, _)| pos.pt)
        .collect::<HashSet<Point>>();

    // Plot the initial trajectory
    line2d_bresenham(attacker_pos.pt, victim_pos.pt)
        .iter()
        .skip(1)
        .for_each(|pt| {
            projectile_path.push(*pt);
            if pos_map.contains(&pt) {
                power -= hit_tile_contents(ecs, *pt, current_layer, &mut commands, &mut splatter, power);
            }
            if let Some(bsplatter) = &mut splatter {
                let idx = map.get_current().point2d_to_index(*pt);
                map.get_current_mut().tiles[idx].color.fg = bsplatter.to_rgba(1.0);
                bsplatter.r = f32::max(0.0, bsplatter.r - 0.1);
                bsplatter.g = f32::max(0.0, bsplatter.g - 0.1);
                bsplatter.b = f32::max(0.0, bsplatter.b - 0.1);
                if bsplatter.r + bsplatter.g + bsplatter.b < 0.1 {
                    splatter = None;
                }
            }
            range += 1;
            if range > 5 {
                power -= 1;
            }
        });

    // The trajectory can continue if we have power left
    use ultraviolet::Vec2;
    let mut projectile_pos: Vec2 = Vec2::new(victim_pos.pt.x as f32, victim_pos.pt.y as f32);
    let slope = (projectile_pos - Vec2::new(attacker_pos.pt.x as f32, attacker_pos.pt.y as f32))
        .normalized();
    while range < 25 && power > 0 {
        projectile_pos += slope;
        let pt = Point::new(projectile_pos.x as i32, projectile_pos.y as i32);
        projectile_path.push(pt);
        if pos_map.contains(&pt) {
            power -= hit_tile_contents(ecs, pt, current_layer, &mut commands, &mut splatter, power);
        }
        if let Some(bsplatter) = &mut splatter {
            let idx = map.get_current().point2d_to_index(pt);
            map.get_current_mut().tiles[idx].color.fg = bsplatter.to_rgba(1.0);
            bsplatter.r = f32::max(0.0, bsplatter.r - 0.1);
            bsplatter.g = f32::max(0.0, bsplatter.g - 0.1);
            bsplatter.b = f32::max(0.0, bsplatter.b - 0.1);
            if bsplatter.r + bsplatter.g + bsplatter.b < 0.1 {
                splatter = None;
            }
        }
        let idx = map.get_current().point2d_to_index(pt);
        if map.get_current().tiles[idx].tile_type == TileType::Wall {
            range += 100;
            power = 0;
        }
        range += 1;
        if range > 5 {
            power -= 1;
        }
    }

    commands.push((
        Projectile {
            path: projectile_path,
            layer: current_layer as usize,
        },
        Glyph {
            glyph: to_cp437('*'),
            color: ColorPair::new(RED, BLACK),
        },
    ));

    commands.flush(ecs);
}

pub fn hit_tile_contents(
    ecs: &mut World,
    pt: Point,
    layer: u32,
    commands: &mut CommandBuffer,
    splatter: &mut Option<RGB>,
    power: i32
) -> i32 {
    let mut rng_lock = crate::RNG.lock();
    let rng = rng_lock.as_mut().unwrap();

    let mut power_loss = 0;
    let mut dead_entities = Vec::new();
    <(Entity, &Position, &mut Health)>::query()
        .iter_mut(ecs)
        .filter(|(_, pos, _)| pos.layer == layer && pos.pt == pt)
        .for_each(|(entity, _, hp)| {
            let damage = power + rng.roll_dice(1, 4) - 2;
            hp.current -= damage;
            if hp.current < 0 {
                hp.current = 0;
                dead_entities.push(*entity);
            }
            power_loss += hp.current;
        });

    dead_entities.iter().for_each(|e| {
        if let Ok(er) = ecs.entry_ref(*e) {
            if let Ok(boom) = er.get_component::<Explosive>() {
                if let Ok(pos) = er.get_component::<Position>() {
                    commands.push((
                        Position::with_pt(pos.pt, pos.layer),
                        Boom { range: boom.range },
                    ));
                }
            }
        }
    });

    kill_things(ecs, commands, dead_entities, splatter);

    power_loss
}

pub fn melee(ecs: &mut World, map: &mut Map, attacker: Entity, victim: Entity, melee_power: i32) {
    // Check range and validity
    let mut attacker_pos = None;
    let mut defender_pos = None;

    if let Ok(e) = ecs.entry_ref(attacker) {
        if let Ok(pos) = e.get_component::<Position>() {
            attacker_pos = Some(*pos);
        }
    }

    if let Ok(e) = ecs.entry_ref(victim) {
        if let Ok(pos) = e.get_component::<Position>() {
            defender_pos = Some(*pos);
        }
    }
    if attacker_pos.is_none() || defender_pos.is_none() {
        return; // Bail out - invalid data arrived
    }
    let apos = attacker_pos.unwrap();
    let dpos = defender_pos.unwrap();
    if apos.layer != dpos.layer {
        return; // Bail out - can't attack across layers
    }
    let d = DistanceAlg::Pythagoras.distance2d(apos.pt, dpos.pt);
    if d > 1.5 {
        return; // Too far away, bail
    }

    // Inflict damage upon the hapless victim
    let mut dead_entities = Vec::new();
    if let Ok(mut v) = ecs.entry_mut(victim) {
        if let Ok(hp) = v.get_component_mut::<Health>() {
            hp.current = i32::max(0, hp.current - melee_power);
            if hp.current == 0 {
                dead_entities.push(victim);
            }
        }
        if let Ok(blood) = v.get_component::<Blood>() {
            let idx = map.get_layer(dpos.layer as usize).point2d_to_index(dpos.pt);
            map.get_layer_mut(dpos.layer as usize).tiles[idx].color.fg = blood.0.into();
        }
    }

    // If necessary, kill them.
    let mut commands = CommandBuffer::new(ecs);
    let mut splatter = None;
    kill_things(ecs, &mut commands, dead_entities, &mut splatter);

    // Splatter blood. It's good for you.
}

fn kill_things(
    ecs: &mut World,
    commands: &mut CommandBuffer,
    dead_entities: Vec<Entity>,
    splatter: &mut Option<RGB>,
) {
    dead_entities.iter().for_each(|entity| {
        let mut was_decor = false;
        let mut was_player = false;
        if let Ok(mut er) = ecs.entry_mut(*entity) {
            let mut was_colonist = false;
            if let Ok(_colonist) = er.get_component_mut::<ColonistStatus>() {
                commands.add_component(*entity, ColonistStatus::DiedAfterStart);
                was_colonist = true;
            }
            if let Ok(g) = er.get_component_mut::<Glyph>() {
                g.color.bg = DARK_RED.into();
                g.color.fg = DARK_GRAY.into();
            }
            if let Ok(n) = er.get_component_mut::<Name>() {
                n.0 = format!("Corpse: {}", n.0);
            }
            if was_colonist {
                if let Ok(d) = er.get_component_mut::<Description>() {
                    let mut rng = RandomNumberGenerator::new();
                    if rng.range(0, 10) < 5 {
                        d.0 = format!(
                            "{} They left behind a spouse and {} children.",
                            d.0,
                            rng.range(1, 8)
                        );
                    }
                }
            }
            if let Ok(b) = er.get_component::<Blood>() {
                *splatter = Some(b.0);
            }
            if let Ok(_) = er.get_component::<SetDecoration>() {
                was_decor = true;
            }
            if let Ok(_) = er.get_component::<Player>() {
                was_player = true;
            }
        }
        if !was_player {
            commands.remove_component::<Health>(*entity);
            commands.remove_component::<Active>(*entity);
            commands.remove_component::<CanBeActivated>(*entity);
            commands.remove_component::<Blood>(*entity);
            commands.remove_component::<Targetable>(*entity);
            commands.remove_component::<Explosive>(*entity);
            commands.remove_component::<TimedEvent>(*entity);
        }
        if was_decor {
            commands.remove_component::<Glyph>(*entity);
            commands.remove_component::<Description>(*entity);
        }
    });
}
