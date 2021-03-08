use crate::components::*;
use crate::map::WIDTH;
use bracket_lib::prelude::*;
use legion::*;

pub fn render_status(ctx: &mut BTerm, ecs: &World, mut y: i32) -> i32 {
    let x = WIDTH + 3;
    let mut hp_query = <(&Player, &Health)>::query();
    hp_query.for_each(ecs, |(_, hp)| {
        ctx.print_color(
            x,
            y,
            WHITE,
            BLACK,
            format! {"Hit Points        : {} / {}", hp.current, hp.max},
        );
        y += 1;
    });

    let damage : i32 = <(&PropertyValue, &Position)>::query()
        .filter(!component::<Health>())
        .iter(ecs)
        .map(|(v, _)| v.0)
        .sum();
    ctx.print_color(
        x,
        y,
        WHITE,
        BLACK,
        format! {"Property Damage: {}", damage},
    );
    y += 1;

    ctx.print_color(x, y, GREY, BLACK, "----------------------------");
    y += 1;
    y
}
