use crate::components::*;
use bracket_lib::prelude::*;
use legion::*;
use std::collections::HashSet;

pub fn spawn_face_eater(ecs: &mut World, location: Point, layer: u32) {
    let entity = ecs.push((
        Name("Face Eater".to_string()),
        Hostile {
            aggro: AggroMode::Player,
            ranged: Vec::new(),
            melee: vec![Melee { damage: 1 }],
        },
        Targetable {},
        Position::with_pt(location, layer),
        Glyph {
            glyph: 157, // Yen symbol
            color: ColorPair::new(RED, BLACK),
        },
        Description("Nasty eight-legged beastie that likes to eat faces.".to_string()),
        Health { max: 3, current: 3 },
        Blood(DARK_GREEN.into()),
    ));
    let mut commands = legion::systems::CommandBuffer::new(ecs);
    commands.add_component(
        entity,
        FieldOfView {
            radius: 6,
            visible_tiles: HashSet::new(),
        },
    );
    commands.add_component(entity, CanBeActivated {});
    commands.flush(ecs);
}
