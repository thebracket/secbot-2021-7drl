use crate::components::*;
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;

pub fn spawn_soda_machine(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('◘'),
            color: ColorPair::new(YELLOW, BLACK),
        },
        Name("Soda Machine".to_string()),
        Description("A powered-down soda machine".to_string()),
        Health { current: 3, max: 3 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(100),
        SetDecoration {},
    ));
}

pub fn spawn_snack_machine(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('◘'),
            color: ColorPair::new(MAGENTA, BLACK),
        },
        Name("Snack Machine".to_string()),
        Description("A powered-down snack machine".to_string()),
        Health { current: 3, max: 3 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(100),
        SetDecoration {},
    ));
}

pub fn spawn_chair(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('╓'),
            color: ColorPair::new(GRAY, BLACK),
        },
        Name("Plastic Chair".to_string()),
        Description("A plastic chair".to_string()),
        Health { current: 1, max: 1 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(5),
        SetDecoration {},
    ));
}

pub fn spawn_table(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('╥'),
            color: ColorPair::new(GRAY, BLACK),
        },
        Name("Plastic Table".to_string()),
        Description("A plastic table".to_string()),
        Health { current: 2, max: 2 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(10),
        SetDecoration {},
    ));
}

pub fn spawn_greeter(ecs: &mut World, pos: Point, layer: u32) {
    let e = ecs.push((
        Glyph {
            glyph: to_cp437('♥'),
            color: ColorPair::new(PINK, BLACK),
        },
        Name("GreeterBot".to_string()),
        Description("Bracket Corp welcoming robot. Your safety is important to us!".to_string()),
        Health { current: 2, max: 2 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(100),
        SetDecoration {},
    ));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        e,
        Dialog {
            lines: vec![
                "Welcome to Bracket 394!".to_string(),
                "Your safety is important to us.".to_string(),
                "Please wear a hard hat at all times.".to_string(),
                "We hope you enjoy your mining experience!".to_string(),
            ],
        },
    );
    commands.add_component(e, CanBeActivated {});
    commands.flush(ecs);
}

pub fn spawn_bed(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('ß'),
            color: ColorPair::new(YELLOW, BLACK),
        },
        Name("Comfy Bed".to_string()),
        Description("A really comfortable bed".to_string()),
        Health { current: 5, max: 5 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(100),
        SetDecoration {},
    ));
}

pub fn spawn_tree(ecs: &mut World, pos: Point, layer: u32) {
    ecs.push((
        Glyph {
            glyph: to_cp437('♣'),
            color: ColorPair::new(GREEN, BLACK),
        },
        Name("Bonsai Tree".to_string()),
        Description("A small tree, providing oxygenation.".to_string()),
        Health { current: 5, max: 5 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(500),
        SetDecoration {},
    ));
}

pub fn spawn_explosive_barrel(ecs: &mut World, pos: Point, layer: u32) {
    let e = ecs.push((
        Glyph {
            glyph: to_cp437('O'),
            color: ColorPair::new(ORANGE, BLACK),
        },
        Name("Explosive Barrel".to_string()),
        Description("Why do people ALWAYS leave these lying around?".to_string()),
        Health { current: 5, max: 5 },
        Targetable {},
        Position::with_pt(pos, layer),
        PropertyValue(50),
        SetDecoration {},
    ));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(e, Explosive { range: 3 });
    commands.flush(ecs);
}

pub fn spawn_live_grenade(ecs: &mut World, pos: Point, layer: u32) {
    let e = ecs.push((
        Glyph {
            glyph: to_cp437('g'),
            color: ColorPair::new(ORANGE, BLACK),
        },
        Name("Live Grenade".to_string()),
        Description("Ticking time-bomg, counting down to an explosion.".to_string()),
        Health { current: 1, max: 1 },
        Targetable {},
        Position::with_pt(pos, layer),
        SetDecoration {},
        CanBeActivated {},
    ));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(e, TimedEvent { timer: 3 });
    commands.flush(ecs);
}
