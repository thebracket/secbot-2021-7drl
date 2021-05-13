use bracket_lib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Capsule,
    Wall,
    Floor,
    Outside,
    StairsDown,
}

#[derive(Clone)]
pub struct Tile {
    pub glyph: FontCharType,
    pub color: ColorPair,
    pub blocked: bool,
    pub opaque: bool,
    pub tile_type: TileType,
}

impl Tile {
    pub fn default() -> Self {
        Self {
            glyph: to_cp437('.'),
            color: ColorPair::new(GREY, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::Floor,
        }
    }

    pub fn empty() -> Self {
        Self {
            glyph: to_cp437(' '),
            color: ColorPair::new(DARK_GRAY, BLACK),
            blocked: true,
            opaque: false,
            tile_type: TileType::Empty,
        }
    }

    pub fn floor() -> Self {
        Self {
            glyph: to_cp437('∙'),
            color: ColorPair::new(DARK_GRAY, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::Floor,
        }
    }

    pub fn wall() -> Self {
        Self {
            glyph: to_cp437('#'),
            color: ColorPair::new(DARK_GRAY, BLACK),
            blocked: true,
            opaque: true,
            tile_type: TileType::Wall,
        }
    }

    pub fn window() -> Self {
        Self {
            glyph: to_cp437('#'),
            color: ColorPair::new(DARK_CYAN, BLACK),
            blocked: true,
            opaque: false,
            tile_type: TileType::Wall,
        }
    }

    pub fn capsule_floor() -> Self {
        Self {
            glyph: to_cp437('∙'),
            color: ColorPair::new(DARK_CYAN, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::Capsule,
        }
    }

    pub fn capsule_wall() -> Self {
        Self {
            glyph: to_cp437('#'),
            color: ColorPair::new(DARK_CYAN, BLACK),
            blocked: true,
            opaque: true,
            tile_type: TileType::Capsule,
        }
    }

    pub fn capsule_window() -> Self {
        Self {
            glyph: to_cp437('%'),
            color: ColorPair::new(DARK_CYAN, BLACK),
            blocked: true,
            opaque: false,
            tile_type: TileType::Capsule,
        }
    }

    pub fn game_over() -> Self {
        Self {
            glyph: to_cp437('+'),
            color: ColorPair::new(YELLOW, RED),
            blocked: false,
            opaque: false,
            tile_type: TileType::Capsule,
        }
    }

    pub fn alien_landscape(height: f32) -> Self {
        let fg = if height < 0.0 {
            if height < -0.25 {
                (40, 20, 0)
            } else {
                GRAY
            }
        } else {
            (
                (height * 128.0) as u8 + 128,
                ((height * 128.0) as u8 + 128) / 2,
                0,
            )
        };

        Self {
            glyph: to_cp437('~'),
            color: ColorPair::new(fg, BLACK),
            blocked: height <= -0.255,
            opaque: false,
            tile_type: TileType::Outside,
        }
    }

    pub fn stairs_down() -> Self {
        Self {
            glyph: to_cp437('>'),
            color: ColorPair::new(YELLOW, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::StairsDown,
        }
    }

    pub fn stairs_up() -> Self {
        Self {
            glyph: to_cp437('<'),
            color: ColorPair::new(YELLOW, BLACK),
            blocked: false,
            opaque: false,
            tile_type: TileType::StairsDown,
        }
    }
}