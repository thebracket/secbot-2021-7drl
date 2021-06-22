use crate::components::*;
use bracket_lib::prelude::*;
use lazy_static::*;
use legion::systems::CommandBuffer;
use legion::*;
use std::sync::Mutex;

fn build_base_colonist(ecs: &mut World, location: Point, layer: u32) -> Entity {
    let name_lock = NAMES.lock();
    let name = name_lock.unwrap().random_human_name();
    let entity = ecs.push((
        Colonist { path: None },
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
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
    commands.flush(ecs);

    entity
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
    commands.add_component(entity, Description("Colonist senior manager.".to_string()));
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

const FIRST_NAMES_1: &str = include_str!("first_names_female.txt");
const FIRST_NAMES_2: &str = include_str!("first_names_male.txt");
const LAST_NAMES: &str = include_str!("last_names.txt");

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
