use dynamo_lib::geometry::quad::Quad;

pub struct Player {
    pub quad: Quad,
}

impl Player {
    pub fn new(position: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>) -> Player {
        Player {
            quad: Quad::new(position, size),
        }
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.quad.position
    }

    pub fn size(&self) -> cgmath::Vector2<f32> {
        self.quad.size
    }

    pub fn update_position(&mut self, position: cgmath::Vector2<f32>) {
        self.quad = Quad::new(position, self.quad.size);
    }
}
