pub mod player;
pub use player::player_turn;
pub mod colonists;
pub use colonists::colonists_turn;
pub mod monsters;
pub use monsters::monsters_turn;
pub mod combat;

use crate::components::*;
use legion::*;
pub fn human_resources(ecs: &World) -> i32 {
    let mut human_resources = 50;

    // Property Damage
    let damage: i32 = <(&PropertyValue, &Position)>::query()
        .filter(!component::<Health>())
        .iter(ecs)
        .map(|(v, _)| v.0)
        .sum();
    human_resources -= damage / 100;

    // Colonists
    let mut query = <(Entity, &Colonist, &ColonistStatus)>::query();
    let mut total_colonists = 0;
    let mut located_alive = 0;
    let mut located_dead = 0;
    let mut died_in_rescue = 0;
    let mut rescued = 0;

    query.for_each(ecs, |(entity, _, status)| {
        total_colonists += 1;
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

    human_resources += rescued * 3;
    human_resources -= located_dead;
    human_resources -= died_in_rescue * 10;
    human_resources += located_alive * 2;

    human_resources
}
