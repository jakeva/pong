use dynamo_lib::keyboard::*;

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
            _ => false,
        }
    }
}
