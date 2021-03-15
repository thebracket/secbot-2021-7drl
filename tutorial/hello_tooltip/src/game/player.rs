use crate::{components::*, render::tooltips::render_tooltips};
use crate::{
    map::{Map, HEIGHT, WIDTH},
    NewState,
};
use bracket_lib::prelude::*;
use legion::*;

pub fn player_turn(ctx: &mut BTerm, ecs: &mut World, map: &mut Map) -> NewState {
    render_tooltips(ctx, ecs, map);
    NewState::Wait
}
