use crate::components::*;
use bracket_lib::prelude::*;
use legion::*;

pub fn spawn_random_colonist(ecs: &mut World, location: Point, layer: u32) {
    ecs.push((
        Colonist { path: None },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
        },
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Unknown,
        Dialog {
            lines: vec!["Thanks, SecBot!".to_string()],
        },
    ));
}

pub fn spawn_first_colonist(ecs: &mut World, location: Point, layer: u32) {
    ecs.push((
        Colonist { path: None },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
        },
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Unknown,
        Dialog {
            lines: vec![
                "Bracket Corp is going to save us?".to_string(),
                "I'll head to your ship.".to_string(),
                "Comms are down, power is iffy.".to_string(),
                "No idea where the others are.".to_string(),
            ],
        },
    ));
}
