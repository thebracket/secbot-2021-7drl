use crate::{map::Map, NewState};
use bracket_lib::prelude::*;
use legion::*;
mod gui;
pub mod modal;
pub mod tooltips;
pub use modal::*;
mod camera;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..5 {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn render_gui(ecs: &mut World, map: &Map, mouse_x: i32, mouse_y: i32) {
    let status = gui::PlayerStatus::query(ecs, map.current_layer);

    let camera = camera::Camera::new(ecs);

    let mut gui_batch = DrawBatch::new();
    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, &status);
    gui::render_colony_info(&mut gui_batch, &status.colony);
    gui::render_targeting(&mut gui_batch, &status.target);
    gui_batch.submit(50_000).expect("Batch error"); // On top of everything

    camera.render_map(map);
    camera.render_glyphs(map, ecs);
    camera.render_speech(ecs, map);
    camera.render_projectiles(ecs, map);
    camera.render_targeting(&status.target);
    camera.render_tooltips(ecs, map, mouse_x, mouse_y);
}

pub fn game_over_left(ctx: &mut BTerm) -> NewState {
    ctx.cls();
    ctx.print(
        1,
        1,
        "Game over. You left the map. Haven't written the stuff to show here.",
    );
    ctx.print(
        1,
        2,
        "You need to refresh or reload. Haven't done restarting yet.",
    );
    NewState::NoChange
}

pub fn game_over_decompression(ctx: &mut BTerm) -> NewState {
    ctx.cls();
    ctx.print(
        1,
        1,
        "Game over. Shooting a window in a pressurized atmosphere turned out to be a bad idea.",
    );
    ctx.print(
        1,
        2,
        "You need to refresh or reload. Haven't done restarting yet.",
    );
    NewState::NoChange
}

pub fn game_over_dead(ctx: &mut BTerm) -> NewState {
    ctx.cls();
    ctx.print(1, 1, "Game over. You ran out of hit points.");
    ctx.print(
        1,
        2,
        "You need to refresh or reload. Haven't done restarting yet.",
    );
    NewState::NoChange
}
