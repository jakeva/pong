use dynamo_lib::geometry::quad::Quad;
use dynamo_lib::keyboard::*;
use dynamo_lib::start;
use dynamo_lib::GameState;

struct Input {}

impl Input {
    fn new() -> Self {
        Self {}
    }
}

impl dynamo_lib::InputHandler for Input {
    fn process_keyboard(&self, input: KeyboardInput) {
        println!("{:?}", input.key);
        println!("{:?}", input.state);
    }
}

struct Player {
    quad: Quad,
}

impl Player {
    fn new(position: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>) -> Player {
        Player {
            quad: Quad::new(position, size),
        }
    }
}

struct State {
    player: Player,
}

impl State {
    fn new() -> Self {
        let player = Player::new((0.0, 0.0).into(), (0.05, 0.4).into());
        State { player: player }
    }
}

impl GameState for State {
    fn initialize(&self) {
        println!("Game state initialized");
    }

    fn update(&self) {
        println!("Game state updated");
    }

    fn quads(&self) -> Vec<&Quad> {
        let mut vec = Vec::new();
        vec.push(&self.player.quad);
        return vec;
    }
}

fn main() {
    let mut v: Vec<Box<dyn dynamo_lib::InputHandler>> = Vec::new();
    let input = Input::new();
    v.push(Box::new(input));

    let state = State::new();

    start("Pong", Box::new(state), v);
}
