use dynamo_lib::geometry::quad::Quad;

pub struct Ball {
    pub quad: Quad,
    pub velocity: cgmath::Vector2<f32>,
    pub visible: bool,
}

impl Ball {
    pub fn new(position: cgmath::Vector2<f32>, radius: f32) -> Ball {
        Ball {
            quad: Quad::new(position, (radius, radius).into()),
            velocity: (0.0, 0.0).into(),
            visible: true,
        }
    }

    pub fn position(&self) -> cgmath::Vector2<f32> {
        self.quad.position
    }

    pub fn radius(&self) -> f32 {
        self.quad.size.x
    }

    pub fn update_position(&mut self, position: cgmath::Vector2<f32>) {
        self.quad = Quad::new(position, self.quad.size);
    }
}
