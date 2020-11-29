use dynamo_lib::geometry::Geometry;
use dynamo_lib::keyboard::*;
use dynamo_lib::Game;

use crate::ball::Ball;
use crate::input::Input;
use crate::player::Player;

#[derive(PartialEq)]
pub enum GameState {
    // MainMenu,
    // Serving,
    Playing,
    // GameOver,
    Quiting,
}

pub struct PongGame {
    player1: Player,
    player2: Player,
    ball: Ball,
    input: Input,
    game_state: GameState,
}

impl PongGame {
    pub fn new() -> Self {
        let player1 = Player::new((-0.8, 0.0).into(), (0.05, 0.4).into());
        let player2 = Player::new((0.8, 0.0).into(), (0.05, 0.4).into());
        let ball = Ball::new((0.0, 0.0).into(), 0.05);
        let input = Input::new();

        PongGame {
            player1: player1,
            player2: player2,
            ball: ball,
            input: input,
            game_state: GameState::Playing,
        }
    }
}

impl Game for PongGame {
    fn initialize(&self, geometry: &mut Geometry) {
        for quad in [self.player1.quad, self.player2.quad, self.ball.quad].iter() {
            geometry.push_quad(quad);
        }
    }

    fn update(&mut self, geometry: &mut Geometry) {
        if self.input.p1_up_pressed {
            let position = (self.player1.position().x, self.player1.position().y + 0.1);
            self.player1.update_position(position.into());
        }
        if self.input.p1_down_pressed {
            let position = (self.player1.position().x, self.player1.position().y - 0.1);
            self.player1.update_position(position.into());
        }
        if self.input.p2_up_pressed {
            let position = (self.player2.position().x, self.player2.position().y + 0.1);
            self.player2.update_position(position.into());
        }
        if self.input.p2_down_pressed {
            let position = (self.player2.position().x, self.player2.position().y - 0.1);
            self.player2.update_position(position.into());
        }

        // normalize players
        if self.player1.position().y > 1.0 - self.player1.size().y * 0.5 {
            let position = (self.player1.position().x, 1.0 - self.player1.size().y * 0.5);
            self.player1.update_position(position.into());
        } else if self.player1.position().y < self.player1.size().y * 0.5 - 1.0 {
            let position = (self.player1.position().x, self.player1.size().y * 0.5 - 1.0);
            self.player1.update_position(position.into());
        }
        if self.player2.position().y > 1.0 - self.player2.size().y * 0.5 {
            let position = (self.player2.position().x, 1.0 - self.player2.size().y * 0.5);
            self.player2.update_position(position.into());
        } else if self.player2.position().y < self.player2.size().y * 0.5 - 1.0 {
            let position = (self.player2.position().x, self.player2.size().y * 0.5 - 1.0);
            self.player2.update_position(position.into());
        }

        geometry.reset();
        for quad in [self.player1.quad, self.player2.quad, self.ball.quad].iter() {
            geometry.push_quad(quad);
        }
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.update(input);
    }

    fn is_quitting(&self) -> bool {
        self.game_state == GameState::Quiting
    }
}
