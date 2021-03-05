use bracket_lib::prelude::*;
use legion::*;
use crate::map::{ Map, WIDTH, HEIGHT };
use crate::components::{Position, Glyph};

pub fn render_glyphs(ctx: &mut BTerm, ecs: &World, map: &Map) {
    let mut query = <(&Position, &Glyph)>::query();
    query.for_each(ecs, |(pos, glyph)| {
        if pos.layer == map.current_layer as u32 {
            ctx.set(
                pos.pt.x + 1,
                pos.pt.y + 1,
                glyph.color.fg,
                glyph.color.bg,
                glyph.glyph,
            );
        }
    });
}

pub fn render_ui_skeleton(ctx: &mut BTerm) {
    ctx.draw_hollow_box(0, 0, WIDTH+1, HEIGHT+1, GRAY, BLACK);
    ctx.print_color(2, 0, WHITE, BLACK, "┤ SecBot 2021 7DRL ├");
    ctx.draw_hollow_box(WIDTH+1, 0, 30, HEIGHT+1, GRAY, BLACK);
    ctx.set(WIDTH+1, 0, GRAY, BLACK, to_cp437('┬'));
    ctx.set(WIDTH+1, HEIGHT+1, GRAY, BLACK, to_cp437('┴'));
}