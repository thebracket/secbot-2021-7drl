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
            format! {"Hit Points     : {} / {}", hp.current, hp.max},
        );
        y += 1;
    });

    let damage: i32 = <(&PropertyValue, &Position)>::query()
        .filter(!component::<Health>())
        .iter(ecs)
        .map(|(v, _)| v.0)
        .sum();
    ctx.print_color(x, y, YELLOW, BLACK, format! {"Property Damage: {}", damage});
    y += 1;

    let human_resources = crate::game::human_resources(ecs);

    let (color, phrase) = if human_resources < 10 {
        (RED, "About to kill you")
    } else if human_resources < 20 {
        (ORANGE, "Cranky")
    } else if human_resources < 30 {
        (ORANGE, "Quite Concerned")
    } else if human_resources < 40 {
        (YELLOW, "Nervous")
    } else if human_resources < 60 {
        (GRAY, "Normal")
    } else if human_resources < 70 {
        (GREEN, "Content")
    } else if human_resources < 90 {
        (GREEN, "You're doing great!")
    } else {
        (GREEN, "Ecstatic")
    };

    ctx.print_color(x, y, GREEN, BLACK, "Human Resources Status:".to_string());
    y += 1;
    ctx.print_color(
        x,
        y,
        color,
        BLACK,
        format!("{} ({})", phrase.to_string(), human_resources),
    );
    y += 1;

    ctx.print_color(x, y, GREY, BLACK, "----------------------------");
    y += 1;
    y
}
