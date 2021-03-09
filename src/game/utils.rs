use crate::components::*;
use legion::*;

pub fn has_component<T>(entity: Entity, ecs: &World) -> bool
where
    T: legion::storage::Component,
{
    if let Ok(er) = ecs.entry_ref(entity) {
        if let Ok(_c) = er.get_component::<T>() {
            return true;
        }
    }
    false
}

pub fn targeting_weight(entity: Entity, ecs: &World) -> f32 {
    let mut weight = 0.0;
    if has_component::<Hostile>(entity, ecs) {
        weight -= 50.0;
    } else if has_component::<Colonist>(entity, ecs) {
        weight += 50.0;
    } else if has_component::<Explosive>(entity, ecs) {
        weight -= 25.0;
    }
    weight
}
