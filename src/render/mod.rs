use crate::components::{Glyph, Position};
use crate::{
    map::{Map, HEIGHT, WIDTH},
    NewState,
};
use bracket_lib::prelude::*;
use legion::*;
pub mod colonist_panel;
pub mod tooltips;
pub use colonist_panel::*;
pub mod speech;
pub mod targeting_panel;
pub use targeting_panel::*;
pub mod status_panel;
pub use status_panel::*;

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

    let mut block = TextBlock::new(21, 15, 69, 11);
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
