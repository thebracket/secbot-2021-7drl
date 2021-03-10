use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;

pub fn process_explosions(ecs: &mut World, map: &mut Map) {
    let mut commands = CommandBuffer::new(ecs);
    let mut damage_tiles = Vec::new();
    // Find explosions
    <(Entity, &Boom, &Position)>::query().for_each(ecs, |(entity, explosion, pos)| {
        commands.remove(*entity);

        // Map their FoV
        let target_tiles = field_of_view(pos.pt, explosion.range, map.get_current());
        // Add projectile effects for each boom tile
        target_tiles.iter().for_each(|pt| {
            let idx = map.get_current().point2d_to_index(*pt);
            map.get_current_mut().tiles[idx].color.fg = (50, 50, 50).into();
            damage_tiles.push(*pt);
            commands.push((
                Projectile {
                    path: line2d_bresenham(pos.pt, *pt),
                    layer: map.current_layer,
                },
                Glyph {
                    glyph: to_cp437('░'),
                    color: ColorPair::new(ORANGE, BLACK),
                },
            ));
        });
    });

    let mut ignore_me = None;
    damage_tiles.iter().for_each(|pt| {
        crate::game::combat::hit_tile_contents(
            ecs,
            *pt,
            map.current_layer as u32,
            &mut commands,
            &mut ignore_me,
            6,
        );
    });

    commands.flush(ecs);
}
