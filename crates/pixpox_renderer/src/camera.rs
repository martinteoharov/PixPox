use crate::types::Position;

pub struct Camera2D {
    aspect_ratio: f32,
    position: Position,
}

impl Camera2D {
	pub fn new(aspect_ratio: f32, position: Position) -> Self {
		let mut camera = Camera2D {
			aspect_ratio: aspect_ratio,
			position: position
		};

		camera
	}

	pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
		self.aspect_ratio = aspect_ratio;
		self
	}
}