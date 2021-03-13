use super::{Tile, HEIGHT, TILES, WIDTH};
use bracket_lib::prelude::*;
use legion::*;

pub struct Layer {
    pub tiles: Vec<Tile>,
    pub starting_point: Point,
}

impl Layer {
    pub fn new(depth: usize, ecs: &mut World) -> Self {
        let layer = match depth {
            _ => Self {
                tiles: vec![Tile::default(); TILES],
                starting_point: Point::new(WIDTH / 2, HEIGHT / 2),
            },
        };
        layer
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut idx = 0;
        while y < HEIGHT {
            for x in 0..WIDTH {
                let t = &self.tiles[idx];
                ctx.set(x+1, y+1, t.color.fg, t.color.bg, t.glyph);
                idx += 1;
            }
            y += 1;
        }
    }
}