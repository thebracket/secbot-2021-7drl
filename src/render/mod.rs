use crate::components::{Glyph, Position};
use crate::{
    map::{Map, HEIGHT, WIDTH},
    NewState,
};
use bracket_lib::prelude::*;
use legion::*;

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
    ctx.draw_hollow_box(0, 0, WIDTH + 1, HEIGHT + 1, GRAY, BLACK);
    ctx.print_color(2, 0, WHITE, BLACK, "┤ SecBot 2021 7DRL ├");
    ctx.draw_hollow_box(WIDTH + 1, 0, 30, HEIGHT + 1, GRAY, BLACK);
    ctx.set(WIDTH + 1, 0, GRAY, BLACK, to_cp437('┬'));
    ctx.set(WIDTH + 1, HEIGHT + 1, GRAY, BLACK, to_cp437('┴'));
}

pub fn modal(ctx: &mut BTerm, title: &String, body: &String) -> NewState {
    let mut draw_batch = DrawBatch::new();
    draw_batch.draw_double_box(Rect::with_size(19, 14, 71, 12), ColorPair::new(CYAN, BLACK));
    let mut buf = TextBuilder::empty();
    buf.ln()
        .fg(YELLOW)
        .bg(BLACK)
        .centered(title)
        .fg(CYAN)
        .bg(BLACK)
        .ln()
        .ln()
        .line_wrap(body)
        .ln()
        .ln()
        .fg(YELLOW)
        .bg(BLACK)
        .centered("PRESS ENTER TO CONTINUE")
        .reset();

    let mut block = TextBlock::new(20, 15, 70, 11);
    block.print(&buf).expect("Overflow occurred");
    block.render_to_draw_batch(&mut draw_batch);
    draw_batch.submit(0).expect("Batch error");
    render_draw_buffer(ctx).expect("Render error");

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Return => NewState::Wait,
            VirtualKeyCode::Space => NewState::Wait,
            _ => NewState::NoChange,
        }
    } else {
        NewState::NoChange
    }
}
