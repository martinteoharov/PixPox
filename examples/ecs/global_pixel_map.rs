use log::{debug, error};
use pixpox_ecs::{InputHandler, Texture, Update};
use winit::event::VirtualKeyCode;

use pixpox_renderer::{Camera, Direction};

#[derive(Debug)]
pub struct GlobalPixelMap {
    pixelmap: Vec<[u8; 4]>,
    window_width: u32,
    window_height: u32,
    camera: Camera,
}

impl GlobalPixelMap {
    /// Creates a new empty pixelmap with the given dimensions.
    /// The pixelmap is initially filled with the given clear color.
    ///
    /// ### Arguments
    ///
    /// * `window_width` - The width of the pixelmap in pixels.
    /// * `window_height` - The height of the pixelmap in pixels.
    ///
    /// ### Returns
    ///
    /// A new PixelMap instance.
    ///
    /// ### Example
    ///
    /// ```
    /// # use pixelmap::PixelMap;
    /// let pixelmap = PixelMap::new_empty(640, 480);
    /// ```
    pub fn new_empty(window_height: u32, window_width: u32, camera: Camera) -> Self {
        debug!("camera width: {}, window_width: {}", camera.get_width(), camera.get_height());
        // assert camera fits pixelmap dimensions
        assert!(
            camera.get_width() <= window_width && camera.get_height() <= window_height,
            "Camera dimensions must be smaller than pixelmap dimensions"
        );

        let mut pixelmap: Vec<[u8; 4]> = Vec::new();

        for _y in 0..window_height {
            for _x in 0..window_width {
                let c: [u8; 4] = [0, 0, 0, 0];
                pixelmap.push(c);
            }
        }

        Self {
            pixelmap,
            window_width,
            window_height,
            camera,
        }
    }

    /// Returns index of the position in the grid
    /// * `pos`: (isize, isize) - (column, row) coordinates
    pub fn get_idx(&self, pos: (isize, isize)) -> usize {
        let idx = pos.1 * self.window_width as isize + pos.0;
        idx as usize
    }

    /// Draws a pixel at the given position with the given color.
    /// * `pos`: the position of the pixel to draw.
    /// * `color`: the color to use when drawing the pixel.
    pub fn draw_pos(&mut self, pos: (isize, isize), color: [u8; 4]) {
        let idx = self.get_idx(pos);
        self.pixelmap[idx as usize] = color;
    }

    /// Draws a flat vector of pixels to the screen.
    /// * `vec`: the vector of pixels to draw.
    pub fn draw_flat_vec(&mut self, vec: &mut Vec<[u8; 4]>) {
        std::mem::swap(&mut self.pixelmap, vec);
    }

    /// Extracts and scales the camera pixelmap to the window pixelmap.
    /// * `camera`: the camera to use for extracting the visible region.
    /// ### Example
    /// ```
    /// # use pixelmap::PixelMap;
    /// # use camera::Camera;
    /// let mut pixelmap = PixelMap::new_empty(640, 480);
    /// let camera = Camera::new(0, 0, 320, 240);
    /// pixelmap.extract_and_scale_visible_region(&camera);
    /// ```
    /// ### Returns
    /// A vector of pixels representing the visible region of the camera.
    /// The vector is scaled to the window dimensions.
    pub fn extract_and_scale_visible_region(&self, camera: &Camera) -> Vec<[u8; 4]> {
        debug!(
            "extract_and_scale_visible_region() called with camera: {:?}",
            camera
        );

        let sf_width = self.window_width as f32 / camera.get_width() as f32;
        let sf_height = self.window_height as f32 / camera.get_height() as f32;
        // let sf = sf_width.min(sf_height).ceil() as u32;
        let sfw = sf_width.ceil() as u32;
        let sfh = sf_height.ceil() as u32;

        debug!(
            "region: [window_width: {}, camera.width: {} | window.height: {}, camera.height: {}]",
            self.window_width,
            camera.get_width(),
            self.window_height,
            camera.get_height()
        );
 
        debug!("sfw: {}", sfw);
        debug!("sfh: {}", sfh);

        // scale camera_pixelmap to window_pixelmap
        let mut window_pixelmap = vec![[0; 4]; (self.window_width * self.window_height) as usize];
        for camera_y in camera.get_y()..(camera.get_y() + camera.get_height()) {
            for camera_x in camera.get_x()..(camera.get_x() + camera.get_width()) {
                // calculate real indexes
                let (real_x, real_y) = (camera_x - camera.get_x(), camera_y - camera.get_y());

                // calculate camera index
                let camera_idx = self.get_idx((camera_x as isize, camera_y as isize));

                // ensure camera index is in bounds
                if camera_idx >= self.pixelmap.len() {
                    continue;
                }

                // get camera pixel value to scale
                let pixel_value = self.pixelmap[camera_idx as usize];

                // scale camera pixel value to real window
                for dy in 0..sfh {
                    for dx in 0..sfw {
                        let s_x = (real_x * sfw + dx).clamp(0, self.window_width as u32 - 1);
                        let s_y = (real_y * sfh + dy).clamp(0, self.window_height as u32 - 1);
                        let s_idx = s_y as usize * self.window_width as usize + s_x as usize;

                        window_pixelmap[s_idx] = pixel_value;
                    }
                }
            }
        }

        window_pixelmap
    }
}

impl Texture for GlobalPixelMap {
    fn render(&self, pixels: &mut [u8]) {
        let pixelmap = self.extract_and_scale_visible_region(&self.camera);

        debug!("pixelmap len: {}", pixelmap.len());

        let pixel_chunks = pixels.chunks_exact_mut(4);

        for (c, pix) in pixelmap.iter().zip(pixel_chunks) {
            pix.copy_from_slice(c);
        }
    }

    fn update(&mut self, input: &InputHandler) {
        let scroll_delta = input.winit.scroll_diff();
        // log::error!("update() mouse scroll delta: [{}]", scroll_delta);

        // if scroll up, zoom in
        if scroll_delta >= 1.0 {
            self.camera.zoom(0.8);
        }

        // if scroll down, zoom out
        if scroll_delta <= -1.0 {
            self.camera.zoom(1.2);
        }

        // if key D is pressed, move camera right
        if input.winit.key_held(VirtualKeyCode::D) {
            self.camera.move_direction(Direction::Right);
        }

        // if key A is held, move camera left
        if input.winit.key_held(VirtualKeyCode::A) {
            self.camera.move_direction(Direction::Left);
        }

        // if key W is held, move camera up
        if input.winit.key_held(VirtualKeyCode::W) {
            self.camera.move_direction(Direction::Up);
        }

        // if key S is held, move camera down
        if input.winit.key_held(VirtualKeyCode::S) {
            self.camera.move_direction(Direction::Down);
        }
    }

    fn size(&self) -> (u32, u32) {
        return (self.window_width, self.window_height);
    }

    fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
}
