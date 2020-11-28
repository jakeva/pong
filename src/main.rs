use dynamo_lib::start;

mod input;
mod player;
mod state;
use state::State;

fn main() {
    let state = State::new();
    start("Pong", Box::new(state));
}
