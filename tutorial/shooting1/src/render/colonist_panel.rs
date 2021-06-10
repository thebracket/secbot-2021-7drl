use bracket_lib::prelude::*;
use legion::*;
use crate::components::*;

use super::WIDTH;

pub fn render_colonist_panel(ctx: &mut BTerm, ecs: &World, current_layer: usize, mut y: i32) -> i32 {
    let mut query = <(Entity, &Colonist, &Position, &ColonistStatus)>::query();
    let mut total_colonists = 0;
    let mut colonists_on_layer = 0;
    let mut located_alive = 0;
    let mut located_dead = 0;
    let mut died_in_rescue = 0;
    let mut rescued = 0;

    query.for_each(ecs, |(entity, _, pos, status)| {
        total_colonists += 1;
        if pos.layer == current_layer as u32 && *status != ColonistStatus::Rescued {
            colonists_on_layer += 1;
        }
        if let Ok(entry) = ecs.entry_ref(*entity) {
            if let Ok(_) = entry.get_component::<Found>() {
                match *status {
                    ColonistStatus::Alive => located_alive += 1,
                    ColonistStatus::StartedDead => located_dead += 1,
                    ColonistStatus::DiedAfterStart => died_in_rescue += 1,
                    ColonistStatus::Rescued => rescued += 1,
                    _ => {}
                }
            }
        }
    });

    let x = WIDTH + 3;
    ctx.print_color(x, y, LIME_GREEN, BLACK, format!("Total Colonists   : {}", total_colonists));
    y += 1;
    ctx.print_color(x, y, LIME_GREEN, BLACK, format!("   (On this level): {}", colonists_on_layer));
    y += 1;
    ctx.print_color(x, y, LIME_GREEN, BLACK, format!(" (Located & Alive): {}", located_alive));
    y += 1;
    ctx.print_color(x, y, HOT_PINK, BLACK,   format!("  (Located & Dead): {}", located_dead));
    y += 1;
    ctx.print_color(x, y, RED, BLACK,        format!("  (Died in Rescue): {}", died_in_rescue));
    y += 1;
    ctx.print_color(x, y, GREEN, BLACK,      format!("         (Rescued): {}", rescued));

    y
}