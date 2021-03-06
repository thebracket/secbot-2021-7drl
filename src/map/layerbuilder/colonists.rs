use bracket_lib::prelude::*;
use legion::*;
use crate::components::*;

pub fn spawn_random_colonist(ecs: &mut World, location: Point, layer: u32) {
    ecs.push((
        Colonist{},
        Position::with_pt(location, layer),
        Glyph{ glyph: to_cp437('☺'), color: ColorPair::new( LIME_GREEN, BLACK ) },
        Description("A squishy friend. You are here to rescue your squishies.".to_string())
    ));
}