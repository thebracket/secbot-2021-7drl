use bracket_lib::prelude::*;
pub use legion::*;
pub mod map;
pub use map::*;

struct State {
    ecs: World,
    map: map::Map,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let map = map::Map::new(&mut ecs);
        Self { ecs, map }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        use map::{HEIGHT, WIDTH};
        ctx.draw_hollow_box(0, 0, WIDTH+1, HEIGHT+1, GRAY, BLACK);
        ctx.print_color(2, 0, WHITE, BLACK, "┤ SecBot 2021 7DRL ├");
        ctx.draw_hollow_box(WIDTH+1, 0, 30, HEIGHT+1, GRAY, BLACK);
        ctx.set(WIDTH+1, 0, GRAY, BLACK, to_cp437('┬'));
        ctx.set(WIDTH+1, HEIGHT+1, GRAY, BLACK, to_cp437('┴'));
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}