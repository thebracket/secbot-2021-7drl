use crate::components::*;
use crate::NewState;
use legion::*;

pub fn end_of_turn(ecs: &mut World) -> NewState {
    crate::stats::record_turn();

    /*let n = <(&Colonist, &Position)>::query()
        .iter(ecs)
        .count();
    println!("Count: {}", n);*/

    if <(&Player, &Health)>::query()
        .iter(ecs)
        .map(|(_, h)| h.current)
        .sum::<i32>()
        < 1
    {
        return NewState::Dead;
    }

    NewState::Wait
}
