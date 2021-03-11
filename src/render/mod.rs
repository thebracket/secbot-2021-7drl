use crate::map::Map;
use bracket_lib::prelude::*;
use legion::*;
mod gui;
pub mod modal;
pub use modal::*;
mod camera;
pub mod gameover;
pub use gameover::*;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..5 {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn render_gui(ecs: &mut World, map: &Map, mouse_x: i32, mouse_y: i32, clicked: bool) {
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
    camera.render_tooltips(ecs, map, mouse_x, mouse_y, clicked);
}
