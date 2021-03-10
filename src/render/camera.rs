use super::{gui::safe_print_color, gui::TargetInfo};
use crate::{
    components::*,
    map::{Map, HEIGHT, WIDTH},
    LAYER_CHR, LAYER_DECOR, LAYER_MAP, LAYER_TEXT,
};
use bracket_lib::prelude::*;
use legion::*;

pub struct Camera {
    player_pos: Point,
    viewport: Rect,
}

impl Camera {
    pub fn new(ecs: &World) -> Self {
        let player_pos = <(&Player, &Position)>::query()
            .iter(ecs)
            .map(|(_, pos)| pos.pt)
            .nth(0)
            .unwrap();

        let viewport = Rect::with_size(player_pos.x - 20, player_pos.y - 15, 40, 31);

        Self {
            player_pos,
            viewport,
        }
    }

    fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(20, 15)
    }

    fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    fn screen_to_world(&self, mouse_x: i32, mouse_y: i32) -> Point {
        Point::new(mouse_x + self.viewport.x1, mouse_y + self.viewport.y1)
    }

    pub fn render_map(&self, map: &Map) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_MAP);

        let layer = map.get_current();
        self.viewport.for_each(|pt| {
            if layer.in_bounds(pt) {
                let idx = layer.point2d_to_index(pt);

                if layer.visible[idx] {
                    let t = &layer.tiles[idx];
                    batch.set(self.world_to_screen(pt), t.color, t.glyph);
                } else if layer.revealed[idx] {
                    let t = &layer.tiles[idx];
                    batch.set(
                        self.world_to_screen(pt),
                        ColorPair::new(t.color.fg.to_greyscale(), BLACK),
                        t.glyph,
                    );
                }
            }
        });

        batch.submit(0).expect("Error batching map");
    }

    pub fn render_glyphs(&self, map: &Map, ecs: &World) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_CHR);

        let mut query = <(&Position, &Glyph)>::query();
        query.for_each(ecs, |(pos, glyph)| {
            if pos.layer == map.current_layer as u32 {
                let idx = map.get_current().point2d_to_index(pos.pt);
                if map.get_current().visible[idx] {
                    let screen_pos = self.world_to_screen(pos.pt);
                    batch.set(screen_pos, glyph.color, glyph.glyph);
                }
            }
        });

        batch.submit(4000).expect("Error batching map");
    }

    pub fn render_speech(&self, ecs: &mut World, map: &Map) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_TEXT);

        let mut commands = legion::systems::CommandBuffer::new(ecs);
        <(Entity, &mut Speech, &Position, &Description)>::query().for_each_mut(
            ecs,
            |(entity, speech, pos, desc)| {
                if pos.layer == map.current_layer as u32 {
                    let mut speech_pos = self.world_to_screen_text(pos.pt);
                    speech_pos.x -= (desc.0.len() / 2) as i32;
                    speech_pos.y -= 1;

                    safe_print_color(
                        &mut batch,
                        speech_pos,
                        &desc.0,
                        ColorPair::new(GREEN, BLACK),
                    );

                    speech.lifetime -= 1;
                    if speech.lifetime == 0 {
                        commands.remove(*entity);
                    }
                }
            },
        );
        commands.flush(ecs);

        batch.submit(20_000).expect("Error batching map");
    }

    pub fn render_targeting(&self, target: &TargetInfo) {
        if let Some(pt) = target.point {
            if pt.x < 1 || pt.x > WIDTH as i32 - 2 {
                return;
            }
            let mut batch = DrawBatch::new();
            batch.target(LAYER_MAP);

            let screen = self.world_to_screen(pt);
            if (screen + Point::new(-1, 0)).x > 0 && (screen + Point::new(-1, 0)).x < 50 {
                safe_print_color(
                    &mut batch,
                    screen + Point::new(-1, 0),
                    "[",
                    ColorPair::new(RED, BLACK),
                );
                safe_print_color(
                    &mut batch,
                    screen + Point::new(1, 0),
                    "]",
                    ColorPair::new(RED, BLACK),
                );
            }

            batch.submit(40_000).expect("Error batching map");
        }
    }

    pub fn render_projectiles(&self, ecs: &mut World, map: &Map) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_DECOR);

        let mut commands = legion::systems::CommandBuffer::new(ecs);
        let mut query = <(Entity, &Glyph, &mut Projectile)>::query();
        query.for_each_mut(ecs, |(entity, glyph, projectile)| {
            if projectile.layer == map.current_layer {
                if projectile.path.is_empty() {
                    commands.remove(*entity);
                } else {
                    let pt = projectile.path[0];
                    projectile.path.remove(0);
                    let screen_pt = self.world_to_screen(pt);
                    batch.set(screen_pt, glyph.color, glyph.glyph);
                }
            }
        });
        commands.flush(ecs);

        batch.submit(30_000).expect("Error batching map");
    }

    pub fn render_tooltips(&self, ecs: &World, map: &Map, mouse_x: i32, mouse_y: i32) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_TEXT);
        let map_pos = self.screen_to_world(mouse_x, mouse_y);

        let mut lines = Vec::new();
        <(Entity, &Position, &Description, &Name)>::query().for_each(
            ecs,
            |(entity, pos, desc, name)| {
                if pos.layer == map.current_layer as u32 && pos.pt == map_pos {
                    let idx = map.get_current().point2d_to_index(pos.pt);
                    if map.get_current().visible[idx] {
                        lines.push((CYAN, name.0.clone()));
                        lines.push((GRAY, desc.0.clone()));
                        if let Ok(er) = ecs.entry_ref(*entity) {
                            if let Ok(hp) = er.get_component::<Health>() {
                                lines.push((GRAY, format!("{}/{} hp", hp.current, hp.max)));
                            }
                        }
                    }
                }
            },
        );

        if !lines.is_empty() {
            let height = lines.len() + 1;
            let width = lines.iter().map(|s| s.1.len()).max().unwrap() + 2;
            let tip_x = if map_pos.x < WIDTH as i32 / 2 {
                (mouse_x * 2) + 1
            } else {
                (mouse_x * 2) - (width as i32 + 1)
            };
            let tip_y = if map_pos.y > HEIGHT as i32 / 2 {
                mouse_y - height as i32
            } else {
                mouse_y
            };
            batch.draw_box(
                Rect::with_size(
                    tip_x,
                    tip_y - (lines.len() / 2) as i32,
                    width as i32,
                    height as i32,
                ),
                ColorPair::new(WHITE, BLACK),
            );
            let mut y = tip_y + 1 - (lines.len() / 2) as i32;
            lines.iter().for_each(|s| {
                safe_print_color(
                    &mut batch,
                    Point::new(tip_x + 1, y),
                    &s.1,
                    ColorPair::new(s.0, BLACK),
                );
                y += 1;
            });
        }

        batch.submit(100_000).expect("Error batching map");
    }
}
