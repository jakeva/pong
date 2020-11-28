use dynamo_lib::geometry::quad::Quad;
use dynamo_lib::keyboard::*;
use dynamo_lib::start;
use dynamo_lib::*;

struct Player {
    quad: Quad,
}

impl Player {
    fn new(position: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>) -> Player {
        Player {
            quad: Quad::new(position, size),
        }
    }

    fn position(&self) -> cgmath::Vector2<f32> {
        self.quad.position
    }

    fn size(&self) -> cgmath::Vector2<f32> {
        self.quad.size
    }

    fn update_position(&mut self, position: cgmath::Vector2<f32>) {
        self.quad = Quad::new(position, self.quad.size);
        // println!("{:?}", self.quad);
    }
}

#[derive(Debug, Default)]
pub struct Input {
    pub p1_up_pressed: bool,
    pub p1_down_pressed: bool,
    pub p2_up_pressed: bool,
    pub p2_down_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, input: KeyboardInput) -> bool {
        let pressed = input.state == KeyboardKeyState::Pressed;
        match input.key {
            KeyboardKey::Up => {
                self.p2_up_pressed = pressed;
                true
            }
            KeyboardKey::Down => {
                self.p2_down_pressed = pressed;
                true
            }
            KeyboardKey::W => {
                self.p1_up_pressed = pressed;
                true
            }
            KeyboardKey::S => {
                self.p1_down_pressed = pressed;
                true
            }
            KeyboardKey::Return => {
                // self.enter_pressed = pressed;
                true
            }
            _ => false,
        }
    }

    pub fn ui_up_pressed(&self) -> bool {
        self.p1_up_pressed // || self.p2_up_pressed
    }

    pub fn ui_down_pressed(&self) -> bool {
        false
        // self.p1_down_pressed || self.p2_down_pressed
    }
}

struct State {
    player1: Player,
    player2: Player,
    input: Input,
}

impl State {
    fn new() -> Self {
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
        // println!("{:?}", &self.player.quad);
        let mut vec = Vec::new();
        vec.push(&self.player1.quad);
        vec.push(&self.player2.quad);
        return vec;
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        self.input.update(input);
        // println!("{:?}", input.key);
        // println!("{:?}", input.state);
    }
}

fn main() {
    let state = State::new();

    // let mut v: Vec<Box<dyn dynamo_lib::InputHandler>> = Vec::new();
    // v.push(Box::new(state));

    start("Pong", Box::new(state));
}
