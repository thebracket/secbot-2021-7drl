use bracket_lib::prelude::*;
use lazy_static::*;
pub use legion::*;
use std::{collections::HashSet, sync::Mutex};
mod components;
mod game;
mod map;
mod render;
mod text;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    Modal { title: String, body: String },
    GameOverLeft,
}

#[derive(PartialEq)]
pub enum NewState {
    NoChange,
    Wait,
    Player,
    Enemy,
    LeftMap,
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
        let mut state = Self {
            ecs,
            map,
            turn: TurnState::Modal {
                title: "SecBot Has Landed".to_string(),
                body: text::INTRO.to_string(),
            },
        };
        state.new_game();
        state
    }

    fn new_game(&mut self) {
        use components::*;

        // Spawn the player
        self.ecs.push((
            Player {},
            Name("SecBot".to_string()),
            Position::with_pt(
                self.map.get_current().starting_point,
                self.map.current_layer as u32,
            ),
            Glyph {
                glyph: to_cp437('@'),
                color: ColorPair::new(YELLOW, BLACK),
            },
            Description("Everybody's favorite Bracket Corp SecBot".to_string()),
            FieldOfView {
                radius: 20,
                visible_tiles: HashSet::new(),
            },
            Targeting {
                targets: Vec::new(),
                current_target: None,
                index: 0,
            },
        ));

        // Trigger FOV for the first round
        game::player::update_fov(&NewState::Enemy, &mut self.ecs, &mut self.map);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        render::render_ui_skeleton(ctx);
        let y = render::render_colonist_panel(ctx, &self.ecs, self.map.current_layer);
        let (_y, target_pt) =
            render::render_targeting_panel(y, ctx, &self.ecs, self.map.current_layer);
        self.map.render(ctx);
        render::render_glyphs(ctx, &self.ecs, &self.map, target_pt);
        render::speech::render_speech(ctx, &mut self.ecs, &self.map);

        let new_state = match &self.turn {
            TurnState::Modal { title, body } => render::modal(ctx, title, body),
            TurnState::WaitingForInput => game::player_turn(ctx, &mut self.ecs, &mut self.map),
            TurnState::EnemyTurn => {
                game::colonists_turn(&mut self.ecs, &mut self.map);
                NewState::Wait
            }
            TurnState::GameOverLeft => render::game_over_left(ctx),
            TurnState::PlayerTurn => NewState::Enemy, // Placeholder
        };
        match new_state {
            NewState::NoChange => {}
            NewState::Wait => self.turn = TurnState::WaitingForInput,
            NewState::Enemy => self.turn = TurnState::EnemyTurn,
            NewState::LeftMap => self.turn = TurnState::GameOverLeft,
            NewState::Player => self.turn = TurnState::PlayerTurn,
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
