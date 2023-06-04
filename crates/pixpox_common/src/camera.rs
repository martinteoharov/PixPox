pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Camera {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    max_width: u32,
    max_height: u32,
    min_width: u32,
    min_height: u32,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(x: u32, y: u32, height: u32, width: u32, max_height: u32, max_width: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;

        Self {
            x,
            y,
            width,
            height,
            max_height,
            max_width,
            min_width: (10 as f32 * aspect_ratio) as u32,
            min_height: 10,
            aspect_ratio,
        }
    }

    fn move_origin(&mut self, x: i32, y: i32) {
        self.x = (self.x as i32 + x).clamp(0, (self.max_width - self.width) as i32) as u32;
        self.y = (self.y as i32 + y).clamp(0, (self.max_height - self.height) as i32) as u32;
    }

    pub fn zoom(&mut self, scale: f32) {
        let old_center_x = self.x + self.width / 2;
        let old_center_y = self.y + self.height / 2;

        let mut new_width = (self.width as f32 * scale) as u32;
        let mut new_height = (self.height as f32 * scale) as u32;

        // Correct aspect ratio
        if new_width as f32 / new_height as f32 > self.aspect_ratio {
            new_width = (new_height as f32 * self.aspect_ratio) as u32;
        } else {
            new_height = (new_width as f32 / self.aspect_ratio) as u32;
        }

        // Keep width and height in bounds (also keeping original aspect ratio)
        self.width = new_width.clamp(self.min_width, self.max_width);
        self.height = new_height.clamp(self.min_height, self.max_height);

        let new_origin_x = old_center_x as i32 - self.width as i32 / 2;
        let new_origin_y = old_center_y as i32 - self.height as i32 / 2;

        self.move_origin(new_origin_x - self.x as i32, new_origin_y - self.y as i32);
    }

    // move function with direction
    pub fn move_direction(&mut self, direction: Direction) {
        // calculate movement speed based on camera scale
        let speed = (1.0 * self.get_scale()).ceil() as i32;

        match direction {
            Direction::Up => self.move_origin(0, -speed),
            Direction::Down => self.move_origin(0, speed),
            Direction::Left => self.move_origin(-speed, 0),
            Direction::Right => self.move_origin(speed, 0),
        }
    }

    // move function with delta
    pub fn move_delta(&mut self, delta: (i32, i32)) {
        self.move_origin(delta.0, delta.1);
    }

    // getters
    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_dim(&mut self) -> (u32, u32) {
        (0, 0)
    }

    pub fn get_scale(&self) -> f32 {
        self.width as f32 / self.max_width as f32
    }
}
