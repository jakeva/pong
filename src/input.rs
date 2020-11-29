use dynamo_lib::keyboard::*;

#[derive(Debug, Default)]
pub struct Input {
    pub p1_up_pressed: bool,
    pub p1_down_pressed: bool,
    pub p2_up_pressed: bool,
    pub p2_down_pressed: bool,
    pub enter_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, input: KeyboardInput) {
        let pressed = input.state == KeyboardKeyState::Pressed;
        match input.key {
            KeyboardKey::Up => {
                self.p2_up_pressed = pressed;
            }
            KeyboardKey::Down => {
                self.p2_down_pressed = pressed;
            }
            KeyboardKey::W => {
                self.p1_up_pressed = pressed;
            }
            KeyboardKey::S => {
                self.p1_down_pressed = pressed;
            }
            KeyboardKey::Return => {
                self.enter_pressed = pressed;
            }
            _ => (),
        }
    }

    pub fn ui_up_pressed(&self) -> bool {
        self.p1_up_pressed || self.p2_up_pressed
    }

    pub fn ui_down_pressed(&self) -> bool {
        self.p1_down_pressed || self.p2_down_pressed
    }
}
