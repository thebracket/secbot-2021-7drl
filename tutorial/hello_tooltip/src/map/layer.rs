use super::{layerbuilder::*, Tile, HEIGHT, TILES, WIDTH};
use bracket_lib::prelude::*;
use legion::*;

pub struct Layer {
    pub tiles: Vec<Tile>,
    pub starting_point: Point,
}

impl Layer {
    pub fn new(depth: usize, ecs: &mut World) -> Self {
        let layer = match depth {
            0 => build_entrance(ecs),
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

    fn test_exit(&self, pt: Point, delta: Point, exits: &mut SmallVec<[(usize, f32); 10]>) {
        let dest_pt = pt + delta;
        if self.in_bounds(dest_pt) {
            let dest_idx = self.point2d_to_index(pt + delta);
            if !self.tiles[dest_idx].blocked {
                exits.push((dest_idx, 1.0));
            }
        }
    }
}

impl Algorithm2D for Layer {
    fn dimensions(&self) -> Point {
        Point::new(WIDTH, HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < WIDTH as i32 && pos.y > 0 && pos.y <= HEIGHT as i32
    }
}

impl BaseMap for Layer {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].opaque
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let pt = self.index_to_point2d(idx);
        self.test_exit(pt, Point::new(-1, 0), &mut exits);
        self.test_exit(pt, Point::new(1, 0), &mut exits);
        self.test_exit(pt, Point::new(0, -1), &mut exits);
        self.test_exit(pt, Point::new(0, 1), &mut exits);
        exits
    }
}