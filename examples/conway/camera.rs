use log::debug;

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
        Self {
            x,
            y,
            width,
            height,
            max_height,
            max_width,
            min_width: 10,
            min_height: 10,
            aspect_ratio: width as f32 / height as f32,
        }
    }

    pub fn move_origin(&mut self, x: i32, y: i32) {
        self.x = (self.x as i32 + x) as u32;
        self.y = (self.y as i32 + y) as u32;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // don't allow resizing to be bigger than max
        self.width = if width > self.max_width {
            self.max_width
        } else {
            width
        };

        self.height = if height > self.max_height {
            self.max_height
        } else {
            height
        };
    }

    pub fn zoom(&mut self, scale: f32) {
        /*
        self.width = (self.width as f32 * scale) as u32;
        self.height = (self.height as f32 * scale) as u32;

        // keep aspect ratio and ensure max size
        if self.width as f32 / self.height as f32 > self.aspect_ratio {
            self.width = (self.height as f32 * self.aspect_ratio) as u32;
        } else {
            self.height = (self.width as f32 / self.aspect_ratio) as u32;
        }

        // keep width and height in bounds
        self.width = if self.width < self.min_width {
            self.min_width
        } else if self.width > self.max_width {
            self.max_width
        } else {
            self.width
        };

        self.height = if self.height < self.min_height {
            self.min_height
        } else if self.height > self.max_height {
            self.max_height
        } else {
            self.height
        };


        */
        debug!("ZOOM: self.width: {}, self.height: {}, scale: {}", self.width, self.height, scale);
        let new_width = (self.width as f32 * scale) as u32;
        let new_height = (self.height as f32 * scale) as u32;

        if self.width as f32 / self.height as f32 > self.aspect_ratio {
            self.width = (self.height as f32 * self.aspect_ratio) as u32;
        } else {
            self.height = (self.width as f32 / self.aspect_ratio) as u32;
        }

        self.width = new_width.clamp(self.min_width, self.max_width);
        self.height = new_height.clamp(self.min_height, self.max_height);
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
