use crate::components::*;
use bracket_lib::prelude::*;
use legion::*;

pub struct PlayerStatus {
    pub current_hp: i32,
    pub max_hp: i32,
    pub property_damage: i32,
    pub human_resources: i32,
    pub colony: ColonyInfo,
    pub target: TargetInfo,
}

pub struct ColonyInfo {
    pub total_colonists: i32,
    pub colonists_on_layer: i32,
    pub located_alive: i32,
    pub located_dead: i32,
    pub died_in_rescue: i32,
    pub rescued: i32,
}

pub struct TargetInfo {
    pub target: Option<Entity>,
    pub color: Option<RGBA>,
    pub name: Option<String>,
    pub point: Option<Point>,
    pub probability: Option<u32>,
    pub range: Option<u32>,
}

impl PlayerStatus {
    pub fn query(ecs: &World, map_layer: usize) -> Self {
        let colony = PlayerStatus::colony_calculator(ecs, map_layer as u32);
        let (current_hp, max_hp) = PlayerStatus::health(ecs);
        let property_damage = PlayerStatus::property_damage(ecs);
        let human_resources = PlayerStatus::human_resources(&colony, property_damage);
        let target = PlayerStatus::targeting_info(ecs);
        Self {
            current_hp,
            max_hp,
            property_damage,
            human_resources,
            colony,
            target,
        }
    }

    fn health(ecs: &World) -> (i32, i32) {
        <(&Player, &Health)>::query()
            .iter(ecs)
            .map(|(_, hp)| (hp.current, hp.max))
            .nth(0)
            .unwrap()
    }

    fn property_damage(ecs: &World) -> i32 {
        <(&PropertyValue, &Position)>::query()
            .filter(!component::<Health>())
            .iter(ecs)
            .map(|(v, _)| v.0)
            .sum()
    }

    fn colony_calculator(ecs: &World, current_layer: u32) -> ColonyInfo {
        let mut query = <(Entity, &Colonist, &Position, &ColonistStatus)>::query();
        let mut total_colonists = 0;
        let mut colonists_on_layer = 0;
        let mut located_alive = 0;
        let mut located_dead = 0;
        let mut died_in_rescue = 0;
        let mut rescued = 0;

        query.for_each(ecs, |(entity, _, pos, status)| {
            total_colonists += 1;
            if pos.layer == current_layer as u32 && *status != ColonistStatus::Rescued {
                colonists_on_layer += 1;
            }
            if let Ok(entry) = ecs.entry_ref(*entity) {
                if let Ok(_) = entry.get_component::<Found>() {
                    match *status {
                        ColonistStatus::Alive => located_alive += 1,
                        ColonistStatus::StartedDead => located_dead += 1,
                        ColonistStatus::DiedAfterStart => died_in_rescue += 1,
                        ColonistStatus::Rescued => rescued += 1,
                    }
                }
            }
        });

        ColonyInfo {
            total_colonists,
            colonists_on_layer,
            located_alive,
            located_dead,
            died_in_rescue,
            rescued,
        }
    }

    fn human_resources(colony: &ColonyInfo, property_damage: i32) -> i32 {
        let mut human_resources = 50;

        // Pay for what you break
        human_resources -= property_damage / 1000;

        // Colonist status
        human_resources += colony.rescued * 10;
        human_resources -= colony.located_dead;
        human_resources -= colony.died_in_rescue * 2;
        human_resources += colony.located_alive * 2;

        human_resources
    }

    fn targeting_info(ecs: &World) -> TargetInfo {
        let target = <(&Player, &Targeting)>::query()
            .iter(ecs)
            .map(|(_, t)| t.current_target)
            .nth(0)
            .unwrap();

        let mut color = None;
        let mut name = None;
        let mut point = None;
        let mut probability = None;
        let mut range = None;

        if let Some(target_entity) = target {
            if let Ok(entry) = ecs.entry_ref(target_entity) {
                color = if let Ok(g) = entry.get_component::<Glyph>() {
                    Some(g.color.fg)
                } else {
                    Some(RGBA::named(RED))
                };

                name = if let Ok(name) = entry.get_component::<Name>() {
                    Some(name.0.to_uppercase())
                } else {
                    None
                };

                point = if let Ok(pos) = entry.get_component::<Position>() {
                    Some(pos.pt)
                } else {
                    None
                };
            }

            let (tprobability, trange) = crate::game::player::hit_probability(ecs, target_entity);
            probability = Some(tprobability);
            range = Some(trange);
        }

        TargetInfo {
            target,
            color,
            name,
            point,
            probability,
            range,
        }
    }
}
