use dynamo_lib::geometry::quad::Quad;
use dynamo_lib::keyboard::*;
use dynamo_lib::GameState;

use crate::input::Input;
use crate::player::Player;

pub struct State {
    player1: Player,
    player2: Player,
    input: Input,
}

impl State {
    pub fn new() -> Self {
        let player1 = Player::new((-0.8, 0.0).into(), (0.05, 0.4).into());
        let player2 = Player::new((0.8, 0.0).into(), (0.05, 0.4).into());
        let input = Input::new();

        State {
            player1: player1,
            player2: player2,
            input: input,
        }
    }
}

impl GameState for State {
    fn initialize(&self) {
        println!("Game state initialized");
    }

    fn update(&mut self) {
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
    }

    fn quads(&self) -> Vec<&Quad> {
        let mut vec = Vec::new();
        vec.push(&self.player1.quad);
        vec.push(&self.player2.quad);
        return vec;
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.update(input);
    }
}
