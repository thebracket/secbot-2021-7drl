use crate::components::*;
use bracket_lib::prelude::*;
use legion::*;

use super::WIDTH;

pub fn render_targeting_panel(
    mut y: i32,
    ctx: &mut BTerm,
    ecs: &World,
    _current_layer: usize,
) -> (i32, Option<Point>) {
    let mut target_point = None;
    let x = WIDTH + 3;
    let mut tq = <(&Player, &Targeting)>::query();
    let current_target = tq.iter(ecs).map(|(_, t)| t.current_target).nth(0).unwrap();

    ctx.print_color(x, y, GREY, BLACK, "----------------------------");
    y += 1;

    if let Some(target_entity) = current_target {
        // TODO: Retrieve target details here
        if let Ok(entry) = ecs.entry_ref(target_entity) {
            let color = if let Ok(g) = entry.get_component::<Glyph>() {
                g.color.fg
            } else {
                RGBA::named(RED)
            };
            if let Ok(name) = entry.get_component::<Name>() {
                ctx.print_color(
                    x,
                    y,
                    color,
                    BLACK,
                    format!("Target: {}", name.0.to_uppercase()),
                );
                y += 1;
            }
            if let Ok(pos) = entry.get_component::<Position>() {
                target_point = Some(pos.pt);
            }
        }
        ctx.print_color(x, y, GOLD, BLACK, "T to cycle targets");
        y += 1;
        ctx.print_color(x, y, GOLD, BLACK, "F to fire");
        y += 1;
    } else {
        ctx.print_color(x, y, GREEN, BLACK, "No current target");
        y += 1;
    }

    (y, target_point)
}
