use crate::{
    components::{Glyph, Position},
    map::{Map, HEIGHT, WIDTH},
    NewState,
};
use bracket_lib::prelude::*;
use legion::*;
pub mod tooltips;
pub mod speech;
pub mod projectiles;
mod gui;
pub mod modal;
pub use modal::*;
mod camera;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..5 {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn render_gui(ecs: &World, map: &Map) {
    let status = gui::PlayerStatus::query(ecs, map.current_layer);

    let camera = camera::Camera::new(ecs);

    let mut gui_batch = DrawBatch::new();
    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, &status);
    gui::render_colony_info(&mut gui_batch, &status.colony);
    gui::render_targeting(&mut gui_batch, &status.target);
    gui_batch.submit(50_000).expect("Batch error"); // On top of everything

    camera.render_map(map);
}

pub fn render_glyphs(ctx: &mut BTerm, ecs: &World, map: &Map, target_pt: Option<Point>) {
    let mut player_point = Point::zero();
    let mut query = <(&Position, &Glyph)>::query();
    query.for_each(ecs, |(pos, glyph)| {
        if pos.layer == map.current_layer as u32 {
            let idx = map.get_current().point2d_to_index(pos.pt);
            if map.get_current().visible[idx] {
                ctx.set(
                    pos.pt.x + 1,
                    pos.pt.y + 1,
                    glyph.color.fg,
                    glyph.color.bg,
                    glyph.glyph,
                );
                if glyph.glyph == to_cp437('@') {
                    player_point = pos.pt;
                }
            }
        }
    });

    if let Some(pt) = target_pt {
        ctx.set(pt.x, pt.y + 1, RED, BLACK, to_cp437('['));
        ctx.set(pt.x + 2, pt.y + 1, RED, BLACK, to_cp437(']'));
        //ctx.set_bg(pt.x + 1, pt.y + 1, GOLD);
    }
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
