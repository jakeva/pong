use dynamo_lib::*;
use dynamo_lib::keyboard::*;

struct Input {

}

impl Input {
  fn new() -> Self {
    Self {}
  }
}

impl InputHandler for Input {
  fn process_keyboard(&self, input: KeyboardInput) {
    println!("{:?}", input.key);
    println!("{:?}", input.state);
  }
}

fn main() {
  let mut v: Vec<Box<InputHandler>> = Vec::new();
  let input = Input::new();
  v.push(Box::new(input));
  start("Pong", v);
}
