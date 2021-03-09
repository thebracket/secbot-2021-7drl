use legion::*;
use crate::{ map::Map, text, components::*, game, render };
use bracket_lib::prelude::*;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

pub enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
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
        render::render_gui(&self.ecs, &self.map);
        render_draw_buffer(ctx).expect("Render error");
        /*render::render_ui_skeleton(ctx);
        let y = render::render_status(ctx, &self.ecs, 2);
        let y = render::render_colonist_panel(ctx, &self.ecs, self.map.current_layer, y);
        let (_y, target_pt) =
            render::render_targeting_panel(y, ctx, &self.ecs, self.map.current_layer);
        self.map.render(ctx);
        render::render_glyphs(ctx, &self.ecs, &self.map, target_pt);
        render::speech::render_speech(ctx, &mut self.ecs, &self.map);
        render::projectiles::render_projectiles(ctx, &mut self.ecs, &self.map);*/

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
            NewState::LeftMap => self.turn = TurnState::GameOverLeft,
            NewState::Player => self.turn = TurnState::PlayerTurn,
            NewState::ShotWindow => self.turn = TurnState::GameOverDecompression,
            NewState::Dead => self.turn = TurnState::GameOverDead,
        }
    }
}
