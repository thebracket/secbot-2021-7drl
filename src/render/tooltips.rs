use bracket_lib::prelude::*;
use legion::*;
use crate::{components::{Description, Position}, map::{HEIGHT, Map, WIDTH}};

pub fn render_tooltips(ctx: &mut BTerm, ecs: &World, map: &Map) {
    let (mx, my) = ctx.mouse_pos();
    let map_x = mx -1;
    let map_y = my - 1;
    if map_x >= 0 && map_x < WIDTH as i32 && map_y >= 0 && map_y < HEIGHT as i32 {
        let mut lines = Vec::new();
        let mut query = <(&Position, &Description)>::query();
        query.for_each(ecs, |(pos, desc)| {
            if pos.layer == map.current_layer as u32 && pos.pt.x == map_x && pos.pt.y == map_y {
                lines.push(desc.0.clone());
            }
        });

        if !lines.is_empty() {
            let height = lines.len() + 1;
            let width = lines.iter().map(|s| s.len()).max().unwrap() + 2;
            let tip_x = if map_x < WIDTH as i32/2 {
                mx+1
            } else {
                mx - (width as i32 +1)
            };
            let tip_y = if map_y > HEIGHT as i32/2 {
                my - height as i32
            } else {
                my
            };
            ctx.draw_box(tip_x, tip_y, width, height, WHITE, BLACK);
            let mut y = tip_y + 1;
            lines.iter().for_each(|s| {
                ctx.print_color(tip_x+1, y, WHITE, BLACK, s);
                y += 1;
            });
        }
    }
}