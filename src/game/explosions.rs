use crate::components::*;
use crate::map::Map;
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;

pub fn process_explosions(ecs: &mut World, map: &Map) {
    let mut commands = CommandBuffer::new(ecs);
    let mut damage_tiles = Vec::new();
    // Find explosions
    <(Entity, &Boom, &Position)>::query().for_each(ecs, |(entity, explosion, pos)| {
        commands.remove(*entity);

        // Map their FoV
        let target_tiles = field_of_view(pos.pt, explosion.range, map.get_current());
        // Add projectile effects for each boom tile
        target_tiles.iter().for_each(|pt| {
            damage_tiles.push(*pt);
            let line = line2d_bresenham(pos.pt, *pt);
            line.iter().for_each(|_lpt| {
                commands.push((
                    Projectile {
                        path: line.clone(),
                        layer: map.current_layer,
                    },
                    Glyph {
                        glyph: to_cp437('░'),
                        color: ColorPair::new(ORANGE, BLACK),
                    },
                ));
            });
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
        );
    });

    commands.flush(ecs);
}
