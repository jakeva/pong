use dynamo_lib::geometry::Geometry;
use dynamo_lib::keyboard::*;
use dynamo_lib::renderer::render_text::TextRenderer;
use dynamo_lib::sound::SoundSystem;
use dynamo_lib::Game;

use std::io::Cursor;

use crate::input::Input;
use crate::state::*;
use crate::system::*;

const BOUNCE_BYTES: &[u8] = include_bytes!("../res/sounds/4362__noisecollector__pongblipa-4.wav");

pub struct SoundPack {
    bounce: Cursor<&'static [u8]>,
}

impl SoundPack {
    pub fn new() -> Self {
        Self {
            bounce: Cursor::new(BOUNCE_BYTES),
        }
    }

    pub fn bounce(&self) -> rodio::Decoder<Cursor<&'static [u8]>> {
        rodio::Decoder::new(self.bounce.clone()).unwrap()
    }
}

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
    sound_pack: SoundPack,
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
            sound_pack: SoundPack::new(),
        }
    }
}

impl Game for PongGame {
    fn initialize(
        &mut self,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
        _window_size: (f32, f32),
    ) {
        self.menu_system.start(&mut self.state);
        self.state.initialize(geometry, text_renderer);
    }

    fn update(
        &mut self,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        sound_system: &SoundSystem,
    ) {
        for event in &self.events {
            match event {
                Event::FocusChanged | Event::ButtonPressed => {
                    sound_system.queue(self.sound_pack.bounce());
                }
                Event::BallBounce(_pos) => {
                    sound_system.queue(self.sound_pack.bounce());
                }
                Event::Score(_) => {
                    sound_system.queue(self.sound_pack.bounce());
                }
            }
        }
        self.events.clear();

        self.visibility_system
            .update_state(&mut self.input, &mut self.state, &mut self.events);
        match self.state.game_state {
            GameState::MainMenu => {
                self.menu_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Serving {
                    self.serving_system.start(&mut self.state);
                }
            }
            GameState::Serving => {
                self.serving_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
                self.play_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Playing {
                    self.play_system.start(&mut self.state);
                }
            }
            GameState::Playing => {
                self.ball_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
                self.play_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
                if self.state.game_state == GameState::Serving {
                    self.serving_system.start(&mut self.state);
                } else if self.state.game_state == GameState::GameOver {
                    self.game_over_system.start(&mut self.state);
                }
            }
            GameState::GameOver => {
                self.game_over_system
                    .update_state(&mut self.input, &mut self.state, &mut self.events);
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
