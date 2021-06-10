use crate::components::*;
use bracket_lib::prelude::*;
use legion::*;

pub fn spawn_face_eater(ecs: &mut World, location: Point, layer: u32) {
    ecs.push((
        Name("Face Eater".to_string()),
        Hostile {},
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('f'),
            color: ColorPair::new(RED, BLACK),
        },
        Description("Nasty eight-legged beastie that likes to eat faces.".to_string()),
        Targetable{},
        Health{max: 3, current: 3},
        Blood(DARK_GREEN.into()),
    ));
}
