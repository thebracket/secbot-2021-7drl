use bracket_lib::prelude::*;
use lazy_static::*;
use std::sync::Mutex;
pub use legion::*;
pub mod map;
pub use map::*;
pub mod components;
mod render;
mod text;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    Modal{title: String, body: String},
}

struct State {
    ecs: World,
    map: map::Map,
    turn: TurnState,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let map = map::Map::new(&mut ecs);
        let mut state = Self { ecs, map, turn: TurnState::Modal{title: "SecBot Has Landed".to_string(), body: text::INTRO.to_string()} };
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
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        render::render_ui_skeleton(ctx);
        self.map.render(ctx);
        render::render_glyphs(ctx, &self.ecs, &self.map);

        match &self.turn {
            TurnState::Modal { title, body } => render::modal(ctx, title, body),
            _ => {} // Do nothing
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}