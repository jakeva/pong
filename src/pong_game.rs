use dynamo_lib::geometry::Geometry;
use dynamo_lib::keyboard::*;
use dynamo_lib::renderer::render_text::TextRenderer;
use dynamo_lib::Game;

use crate::input::Input;
use crate::state::*;
use crate::system::*;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    ButtonPressed,
    FocusChanged,
    BallBounce(cgmath::Vector2<f32>),
    Score(u32),
}

pub struct PongGame {
    pub input: Input,
    events: Vec<Event>,
    state: State,
    menu_system: MenuSystem,
    serving_system: ServingSystem,
    play_system: PlaySystem,
    ball_system: BallSystem,
    game_over_system: GameOverSystem,
    visibility_system: VisibilitySystem,
}

impl PongGame {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            events: Vec::new(),
            state: State::new(),
            menu_system: MenuSystem,
            serving_system: ServingSystem::new(),
            play_system: PlaySystem,
            ball_system: BallSystem,
            game_over_system: GameOverSystem::new(),
            visibility_system: VisibilitySystem,
        }
    }
}

impl Game for PongGame {
    fn initialize(
        &mut self,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        window_size: (f32, f32),
    ) {
        self.menu_system.start(&mut self.state);
        self.state.initialize(geometry, text_renderer);
    }

    fn update(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
        self.visibility_system
            .update_state(&self.input, &mut self.state, &mut self.events);
        match self.state.game_state {
            GameState::MainMenu => {
                self.menu_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Serving {
                    self.serving_system.start(&mut self.state);
                }
            }
            GameState::Serving => {
                self.serving_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                self.play_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Playing {
                    self.play_system.start(&mut self.state);
                }
            }
            GameState::Playing => {
                self.ball_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                self.play_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Serving {
                    self.serving_system.start(&mut self.state);
                } else if self.state.game_state == GameState::GameOver {
                    self.game_over_system.start(&mut self.state);
                }
            }
            GameState::GameOver => {
                self.game_over_system
                    .update_state(&self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::MainMenu {
                    self.menu_system.start(&mut self.state);
                }
            }
            GameState::Quitting => {}
        }

        geometry.reset();
        text_renderer.reset();

        self.state.update(geometry, text_renderer);
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.update(input);
    }

    fn is_quitting(&self) -> bool {
        self.state.game_state == GameState::Quitting
    }
}
