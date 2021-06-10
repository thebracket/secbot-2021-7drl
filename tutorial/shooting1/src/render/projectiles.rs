use crate::{components::*, map::Map, map::WIDTH};
use bracket_lib::prelude::*;
use legion::*;

pub fn render_projectiles(ctx: &mut BTerm, ecs: &mut World, map: &Map) {
    let mut commands = legion::systems::CommandBuffer::new(ecs);
    let mut query = <(Entity, &Glyph, &mut Projectile)>::query();
    query.for_each_mut(ecs, |(entity, glyph, projectile)| {
        if projectile.layer == map.current_layer {
            if projectile.path.is_empty() {
                commands.remove(*entity);
            } else {
                let pt = projectile.path[0];
                projectile.path.remove(0);
                ctx.set(
                    pt.x + 1,
                    pt.y + 1,
                    glyph.color.fg,
                    glyph.color.bg,
                    glyph.glyph,
                );
            }
        }
    });
    commands.flush(ecs);
}