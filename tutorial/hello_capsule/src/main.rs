use bracket_lib::prelude::*;
pub use legion::*;
pub mod map;
pub use map::*;
pub mod components;

struct State {
    ecs: World,
    map: map::Map,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let map = map::Map::new(&mut ecs);
        let mut state = Self { ecs, map };
        state.new_game();
        state
    }

    fn new_game(&mut self) {
        use components::*;
        self.ecs.clear();

        // Spawn the player
        self.ecs.push((
            Player {},
            Position::with_pt(self.map.get_current().starting_point, 0),
            Glyph {
                glyph: to_cp437('@'),
                color: ColorPair::new(YELLOW, BLACK),
            },
            Description("Everybody's favorite Bracket Corp SecBot".to_string()),
        ));
    }

    fn render_glyphs(&self, ctx: &mut BTerm) {
        use components::{Glyph, Position};
        let mut query = <(&Position, &Glyph)>::query();
        query.for_each(&self.ecs, |(pos, glyph)| {
            if pos.layer == self.map.current_layer as u32 {
                ctx.set(
                    pos.pt.x + 1,
                    pos.pt.y + 1,
                    glyph.color.fg,
                    glyph.color.bg,
                    glyph.glyph,
                );
            }
        });
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
        self.render_glyphs(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}