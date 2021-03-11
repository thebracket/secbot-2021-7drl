use bracket_lib::prelude::*;
use crate::NewState;
use legion::World;

pub fn game_over_left(ctx: &mut BTerm, ecs: &World) -> NewState {
    let mut batch = DrawBatch::new();
    // Clear the screen
    for i in 0..5 {
        batch.target(i);
        batch.cls();
    }

    batch.target(crate::LAYER_MAP);
    let backdrop = XpFile::from_resource("resources/takeoff.xp").unwrap();
    let sprite = MultiTileSprite::from_xp(&backdrop);
    sprite.add_to_batch(&mut batch, Point::new(0,0));

    batch.target(crate::LAYER_TEXT);
    batch.print_color(Point::new(70, 5), "SecBot has Escaped!", ColorPair::new(GOLD, BLACK));
    batch.print_color(Point::new(50, 25), "Press ENTER or ESCAPE to try again.", ColorPair::new(CYAN, BLACK));

    let stats = crate::stats::get_stats();
    let bw = ColorPair::new(WHITE, BLACK);
    let status = crate::render::gui::PlayerStatus::query(ecs, 0);

    batch.print_color(Point::new(50, 7), format!("SecBot survived for {} turns.", stats.turns_elapsed), bw);
    batch.print_color(Point::new(50, 8), format!("A total of {} things were killed/destroyed.", stats.total_dead), bw);
    batch.print_color(Point::new(50, 9), format!("{} props were smashed.", stats.total_props_smashed), bw);
    batch.print_color(Point::new(50, 10), format!("{} monsters died.", stats.total_dead), bw);

    batch.print_color(Point::new(50, 11), format!("Caused ${} in property damage.", status.property_damage), bw);
    let (color, phrase) = if status.human_resources < 10 {
        (RED, "About to kill you")
    } else if status.human_resources < 20 {
        (ORANGE, "Cranky")
    } else if status.human_resources < 30 {
        (ORANGE, "Quite Concerned")
    } else if status.human_resources < 40 {
        (YELLOW, "Nervous")
    } else if status.human_resources < 60 {
        (GRAY, "Normal")
    } else if status.human_resources < 70 {
        (GREEN, "Content")
    } else if status.human_resources < 90 {
        (GREEN, "You're doing great!")
    } else {
        (GREEN, "Ecstatic")
    };
    batch.print_color(Point::new(50, 12), format!("Human Resources were: {}", phrase), ColorPair::new(color, BLACK));

    batch.print_color(Point::new(50, 13), format!("There were {} colonists in the game:", status.colony.total_colonists), bw);
    batch.print_color(Point::new(50, 14), format!("  ... of whom {} were rescued,", status.colony.rescued), bw);
    batch.print_color(Point::new(50, 15), format!("  ... {} died during the rescue attempt,", status.colony.died_in_rescue), bw);
    batch.print_color(Point::new(50, 16), format!("  ... {} were already dead when you got there,", status.colony.located_dead), bw);

    batch.submit(1_000_000).expect("Unable to submit batch");

    if let Some(key) = ctx.key {
        if key == VirtualKeyCode::Return || key == VirtualKeyCode::Escape {
            return NewState::Restart;
        }
    }
    NewState::NoChange
}