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

embedded_resource!(TILE_FONT, "../resources/font-16x16.png");

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Secbot - 2021 7DRL") // Set Window Title
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        .with_font("font-16x16.png", 16, 16) // Load big font
        .with_font("vga8x16.png", 8, 16) // Load easy-to-read font
        .with_simple_console(56, 31, "font-16x16.png") // Console 0: Base map
        .with_sparse_console(56, 31, "font-16x16.png") // Console 1: Decorations
        .with_sparse_console(56, 31, "font-16x16.png") // Console 2: Items
        .with_sparse_console(56, 31, "font-16x16.png") // Console 3: Characters
        .with_sparse_console(112, 31, "vga8x16.png") // Console 4: User Interface
        .build()?;

    main_loop(context, State::new())
}
