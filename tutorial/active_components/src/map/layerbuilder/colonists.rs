use bracket_lib::prelude::*;
use legion::*;
use legion::systems::CommandBuffer;
use crate::components::*;

fn build_base_colonist(ecs: &mut World, location: Point, layer: u32) -> Entity {
    ecs.push((
        Colonist { path: None },
        Position::with_pt(location, layer),
        ColorPair::new(LIME_GREEN, BLACK),
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Alive,
        Name("Colonist".to_string()),
        Targetable {},
        CanBeActivated {},
    ))
}

pub fn spawn_first_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "Bracket Corp is going to save us?".to_string(),
                "No idea where the others are.".to_string(),
            ],
        },
    );
    commands.flush(ecs);
}

pub fn spawn_random_colonist(ecs: &mut World, location: Point, layer: u32) {
    // Using this pattern because Legion has a limit to how many components it takes in a push
    let entity = build_base_colonist(ecs, location, layer);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec!["Thanks, SecBot!".to_string()],
        },
    );
    commands.flush(ecs);
}
