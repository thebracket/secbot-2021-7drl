use crate::{components::*, game::{self, timed_events}, map::Map, render, text};
use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::*;
use std::collections::HashSet;

pub enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    WrapUpTurn,
    Modal { title: String, body: String },
    GameOverLeft,
    GameOverDecompression,
    GameOverDead,
}

#[derive(PartialEq)]
pub enum NewState {
    NoChange,
    Wait,
    Player,
    Enemy,
    WrapUp,
    LeftMap,
    ShotWindow,
    Dead,
}

pub struct State {
    pub ecs: World,
    pub map: Map,
    pub turn: TurnState,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let map = Map::new(&mut ecs);
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
        // Spawn the player
        let e = self.ecs.push((
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
            Health {
                max: 10,
                current: 10,
            },
        ));
        let mut commands = CommandBuffer::new(&self.ecs);
        commands.add_component(e, Blood(BROWN2.into()));
        commands.flush(&mut self.ecs);
        // TODO: Add blood

        // Trigger FOV for the first round
        game::player::update_fov(&NewState::Enemy, &mut self.ecs, &mut self.map);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        render::clear_all_consoles(ctx);
        ctx.set_active_console(0);
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        render::render_gui(&mut self.ecs, &self.map, mouse_x, mouse_y);
        render_draw_buffer(ctx).expect("Render error");

        let new_state = match &self.turn {
            TurnState::Modal { title, body } => render::modal(ctx, title, body),
            TurnState::WaitingForInput => game::player_turn(ctx, &mut self.ecs, &mut self.map),
            TurnState::PlayerTurn => {
                let mut is_dead = false;
                <(&Player, &Health)>::query().for_each(&self.ecs, |(_, hp)| {
                    if hp.current == 0 {
                        is_dead = true;
                    }
                });
                if is_dead {
                    NewState::Dead
                } else {
                    NewState::Enemy
                }
                // TODO: Extra turns for speed boosts could go here
            }
            TurnState::EnemyTurn => {
                game::colonists_turn(&mut self.ecs, &mut self.map);
                game::monsters_turn(&mut self.ecs, &mut self.map);
                NewState::WrapUp
            },
            TurnState::WrapUpTurn => {
                game::timed_events::manage_event_timers(&mut self.ecs, &self.map);
                game::explosions::process_explosions(&mut self.ecs, &mut self.map);
                game::dialog::spawn_dialog(&mut self.ecs);
                NewState::Wait
            }
            TurnState::GameOverLeft => render::game_over_left(ctx),
            TurnState::GameOverDecompression => render::game_over_decompression(ctx),
            TurnState::GameOverDead => render::game_over_dead(ctx),
        };
        match new_state {
            NewState::NoChange => {}
            NewState::Wait => self.turn = TurnState::WaitingForInput,
            NewState::Enemy => self.turn = TurnState::EnemyTurn,
            NewState::WrapUp => self.turn = TurnState::WrapUpTurn,
            NewState::LeftMap => self.turn = TurnState::GameOverLeft,
            NewState::Player => self.turn = TurnState::PlayerTurn,
            NewState::ShotWindow => self.turn = TurnState::GameOverDecompression,
            NewState::Dead => self.turn = TurnState::GameOverDead,
        }
    }
}
