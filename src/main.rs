#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;
pub use legion::*;

mod rng;
pub use rng::RNG; // Make the RNG globally available
mod components;
pub mod game;
mod map;
mod render;
mod text;
mod turn;
pub use turn::NewState;
use turn::State;
pub mod stats;

pub const LAYER_MAP: usize = 0;
pub const LAYER_DECOR: usize = 1;
pub const LAYER_ITEMS: usize = 2;
pub const LAYER_CHR: usize = 3;
pub const LAYER_TEXT: usize = 4;

embedded_resource!(TILE_FONT, "../resources/font.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");
embedded_resource!(DEAD_SKULL, "../resources/skull.xp");
embedded_resource!(ESCAPED, "../resources/takeoff.xp");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/font.png");
    link_resource!(VGA_FONT, "resources/vga.png");
    link_resource!(DEAD_SKULL, "resources/skull.xp");
    link_resource!(ESCAPED, "resources/takeoff.xp");

    let context = BTermBuilder::new()
        .with_title("Secbot - 2021 7DRL") // Set Window Title
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        .with_font("font.png", 16, 16) // Load big font
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_simple_console(56, 31, "font.png") // Console 0: Base map
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 1: Decorations
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 2: Items
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 3: Characters
        .with_sparse_console(112, 31, "vga.png") // Console 4: User Interface
        .build()?;

    main_loop(context, State::new())
}
