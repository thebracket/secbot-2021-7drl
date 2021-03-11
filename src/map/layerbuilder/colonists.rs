use crate::components::*;
use bracket_lib::prelude::*;
use lazy_static::*;
use legion::{systems::CommandBuffer, *};
use std::sync::Mutex;

fn build_base_colonist(
    ecs: &mut World,
    location: Point,
    layer: u32,
    weapon: Option<i32>,
) -> Entity {
    let name_lock = NAMES.lock();
    let name = name_lock.unwrap().random_human_name();
    let entity = ecs.push((
        Colonist { path: None, weapon },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(WHITE, BLACK),
        },
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Alive,
        Name(name),
        Targetable {},
        CanBeActivated {},
    ));

    let mut rng = RandomNumberGenerator::new();
    let hp = rng.roll_dice(1, 6) + 3;
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Health {
            max: hp,
            current: hp,
        },
    );
    commands.add_component(entity, Blood(DARK_RED.into()));
    //commands.add_component(entity, Active{});
    commands.flush(ecs);

    entity
}

pub fn spawn_random_colonist(ecs: &mut World, location: Point, layer: u32) {
    // Using this pattern because Legion has a limit to how many components it takes in a push
    let entity = build_base_colonist(ecs, location, layer, None);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec!["Thanks, SecBot!".to_string()],
        },
    );
    commands.flush(ecs);
}

pub fn spawn_first_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer, None);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "Bracket Corp is going to save us?".to_string(),
                "I'll head to your ship.".to_string(),
                "Comms are down, power is iffy.".to_string(),
                "No idea where the others are.".to_string(),
            ],
        },
    );
    commands.add_component(entity, Description("Colonist senior manager.".to_string()));
    commands.flush(ecs);
}

pub fn spawn_napping_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer, None);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "YAWN! Oh, hi SecBot!".to_string(),
                "I was having a terrible dream about monsters.".to_string(),
            ],
        },
    );
    commands.add_component(entity, Description("Colonist senior manager.".to_string()));
    commands.flush(ecs);
}

pub fn spawn_hiding_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer, None);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec!["Nobody will find me in here!".to_string()],
        },
    );
    commands.flush(ecs);
}

pub fn spawn_suicidal_colonist(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer, None);
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "Game Over, Man!".to_string(),
                "Game Over!".to_string(),
                "Wait, I'm not dead?".to_string(),
            ],
        },
    );
    commands.add_component(
        entity,
        Description("Colonist security manager.".to_string()),
    );
    commands.flush(ecs);

    super::props::spawn_live_grenade(ecs, location + Point::new(-1, -1), layer);
}

pub fn spawn_marine_colonist(
    ecs: &mut World,
    location: Point,
    layer: u32,
    rng: &mut RandomNumberGenerator,
) {
    let entity = build_base_colonist(ecs, location, layer, Some(5));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: match rng.range(0, 5) {
                0 => vec!["Better part of valor!".to_string()],
                1 => vec![String::new(), "There's too many of them!".to_string()],
                2 => vec![
                    String::new(),
                    String::new(),
                    "Run for your lives!".to_string(),
                ],
                3 => vec![String::new(), "Anyone bring some ammo?".to_string()],
                _ => vec!["And so it ends...".to_string()],
            },
        },
    );
    commands.add_component(entity, Description("Colonist defense squad.".to_string()));
    commands.flush(ecs);
}

pub fn spawn_marine_leader(ecs: &mut World, location: Point, layer: u32) {
    let entity = build_base_colonist(ecs, location, layer, Some(5));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        Dialog {
            lines: vec![
                "Fire when you see the whites of ... whatever that is".to_string(),
                "Hold the line!".to_string(),
                "Death or Glory!".to_string(),
                "Not looking so good for glory, now.".to_string(),
            ],
        },
    );
    commands.add_component(entity, Description("Chief of Security.".to_string()));
    commands.flush(ecs);
}

pub fn spawn_dead_colonist(ecs: &mut World, location: Point, layer: u32) {
    let name_lock = NAMES.lock();
    let name = name_lock.unwrap().random_human_name();
    ecs.push((
        Colonist {
            path: None,
            weapon: None,
        },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(GRAY, DARK_RED),
        },
        Description("This colonist was dead when you arrived.".to_string()),
        ColonistStatus::StartedDead,
        Name(format!("Corpse: {}", name)),
        CanBeActivated {},
    ));
}

pub fn spawn_dead_doctor(ecs: &mut World, location: Point, layer: u32) {
    let name_lock = NAMES.lock();
    let name = name_lock.unwrap().random_human_name();
    ecs.push((
        Colonist {
            path: None,
            weapon: None,
        },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(GRAY, DARK_RED),
        },
        Description("They appear to have been dissecting an alien.".to_string()),
        ColonistStatus::StartedDead,
        Name(format!("Corpse: Dr. {}", name)),
        CanBeActivated {},
    ));
}

pub fn spawn_dead_xeno(ecs: &mut World, location: Point, layer: u32) {
    ecs.push((
        Colonist {
            path: None,
            weapon: None,
        },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('x'),
            color: ColorPair::new(GRAY, DARK_RED),
        },
        Description("Dissecting a creature with acid blood is a bad idea".to_string()),
        ColonistStatus::StartedDead,
        Name("Corpse: Xenomorph".to_string()),
        CanBeActivated {},
    ));
}

pub fn spawn_colony_secbot(ecs: &mut World, location: Point, layer: u32) {
    let e = ecs.push((
        Friendly {},
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('@'),
            color: ColorPair::new(GREEN, BLACK),
        },
        Description("Security bot model 2, charged with defending the outpost.".to_string()),
        Name("Colony SecBot".to_string()),
        Targetable {},
        CanBeActivated {},
        Dialog {
            lines: vec![
                "Follow me - To The Queen!".to_string(),
                "I know the way.".to_string(),
                "Cover me.".to_string(),
                "LEEROY JENKINS!".to_string(),
            ],
        },
    ));
    let mut commands = CommandBuffer::new(ecs);
    commands.add_component(
        e,
        Health {
            current: 10,
            max: 10,
        },
    );
    commands.flush(ecs);
}

/* Name Generation */

const FIRST_NAMES_1: &str = include_str!("../../../resources/first_names_female.txt");
const FIRST_NAMES_2: &str = include_str!("../../../resources/first_names_male.txt");
const LAST_NAMES: &str = include_str!("../../../resources/last_names.txt");

#[derive(Clone, Debug)]
struct Names {
    male_first: Vec<String>,
    female_first: Vec<String>,
    last_names: Vec<String>,
}

impl Names {
    fn new() -> Self {
        Self {
            male_first: FIRST_NAMES_1.split("\n").map(|n| n.to_string()).collect(),
            female_first: FIRST_NAMES_2.split("\n").map(|n| n.to_string()).collect(),
            last_names: LAST_NAMES.split("\n").map(|n| n.to_string()).collect(),
        }
    }

    fn random_human_name(&self) -> String {
        use inflector::Inflector;
        let mut rng = RandomNumberGenerator::new(); // Avoiding locking issues
        let male = rng.range(0, 100) < 50;
        let first_source = match male {
            true => &self.male_first,
            false => &self.female_first,
        };
        let first_name = rng
            .random_slice_entry(first_source)
            .unwrap()
            .to_title_case();
        let last_name = rng
            .random_slice_entry(&self.last_names)
            .unwrap()
            .to_title_case();

        format!("{} {}", first_name, last_name).to_string()
    }
}

lazy_static! {
    static ref NAMES: Mutex<Names> = Mutex::new(Names::new());
}
