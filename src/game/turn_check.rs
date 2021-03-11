use crate::NewState;
use legion::*;
use crate::components::*;

pub fn end_of_turn(ecs: &mut World) -> NewState {
    crate::stats::record_turn();

    if <(&Player, &Health)>::query()
        .iter(ecs)
        .map(|(_, h)| h.current)
        .sum::<i32>() < 1 {
            return NewState::Dead;
        }

    NewState::Wait
}