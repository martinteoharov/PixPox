use log::{debug, error};
use pixpox_ecs::{InputHandler, Texture, Update};

use crate::camera::Camera;

#[derive(Debug)]
pub struct GlobalPixelMap {
    pixelmap: Vec<[u8; 4]>,
    width: u32,
    height: u32,
    camera: Camera,
}

impl GlobalPixelMap {
    /// Creates a new empty pixelmap with the given dimensions.
    /// The pixelmap is initially filled with the given clear color.
    ///
    /// ### Arguments
    ///
    /// * `width` - The width of the pixelmap in pixels.
    /// * `height` - The height of the pixelmap in pixels.
    ///
    /// ### Returns
    ///
    /// A new PixelMap instance.
    ///
    /// ### Example
    ///
    /// ```
    /// # use pixelmap::PixelMap;
    /// let pixelmap = PixelMap::new_empty(640, 480, [255, 0, 0, 255]);
    /// ```
    pub fn new_empty(height: u32, width: u32, camera: Camera) -> Self {
        // assert camera fits pixelmap dimensions
        assert!(
            camera.get_width() <= width && camera.get_height() <= height,
            "Camera dimensions must be smaller than pixelmap dimensions"
        );

        let mut pixelmap: Vec<[u8; 4]> = Vec::new();

        for _y in 0..height {
            for _x in 0..width {
                let c: [u8; 4] = [0, 0, 0, 0];
                pixelmap.push(c);
            }
        }

        Self {
            pixelmap,
            width,
            height,
            camera,
        }
    }

    /// * `pos`: (isize, isize) - (row, column) coordinates
    /// Returns index of the position in the grid
    pub fn get_idx(&self, pos: (isize, isize)) -> usize {
        let idx = pos.1 * self.width as isize + pos.0;
        idx as usize
    }

    // Draws the given color at the given position in the canvas
    pub fn draw_pos(&mut self, pos: (isize, isize), color: [u8; 4]) {
        let idx = self.get_idx(pos);
        self.pixelmap[idx as usize] = color;
    }

    /// Draws a flat vector of pixels to the screen.
    /// This is the main drawing method for this application.
    pub fn draw_flat_vec(&mut self, vec: &mut Vec<[u8; 4]>) {
        std::mem::swap(&mut self.pixelmap, vec);
    }

    pub fn run(&self) {}

    pub fn extract_visible_region(&self, camera: Camera) -> Vec<[u8; 4]> {
        debug!("extract_visible_region() called with camera: {:?}", camera);
        debug!(
            "camera.width / camera.height: {}",
            camera.get_width() as f32 / camera.get_height() as f32
        );
        debug!(
            "self.width / self.height: {}",
            self.width as f32 / self.height as f32
        );

        // calculate scaling factor
        let sf_width = self.width as f32 / camera.get_width() as f32;
        let sf_height = self.height as f32 / camera.get_height() as f32;
        let sf = sf_width.min(sf_height) as u32;
        // sf = sf - sf % 2;

        debug!("sf: {}", sf);

        let mut visible_pixelmap: Vec<[u8; 4]> = Vec::new();

        for y in camera.get_y()..(camera.get_y() + camera.get_height()) {
            for x in camera.get_x()..(camera.get_x() + camera.get_width()) {
                let idx = self.get_idx((x as isize, y as isize));

                if idx >= self.pixelmap.len() {
                    break;
                }

                visible_pixelmap.push(self.pixelmap[idx as usize]);
            }
        }

        let new_width = self.width as usize;
        let new_height = self.height as usize;

        let mut scaled = vec![[0; 4]; new_width * new_height];

        for y in 0..camera.get_height() {
            for x in 0..camera.get_width() {
                let o_idx = (camera.get_y() + y) * self.width + (camera.get_x() + x);

                if o_idx as usize >= self.pixelmap.len() {
                    break;
                }

                let pixel_value = self.pixelmap[o_idx as usize];

                for dy in 0..sf {
                    for dx in 0..sf {
                        let s_x = (x * sf + dx) as usize;
                        let s_y = (y * sf + dy) as usize;
                        let s_idx = s_y * new_width + s_x;

                        // ensure we don't go out of bounds
                        if s_idx >= scaled.len() {
                            break;
                        }

                        scaled[s_idx] = pixel_value;
                    }
                }
            }
        }

        scaled
    }

    pub fn extract_and_scale_visible_region(&self, camera: &Camera) -> Vec<[u8; 4]> {
        debug!(
            "extract_and_scale_visible_region() called with camera: {:?}",
            camera
        );

        let sf_width = self.width as f32 / camera.get_width() as f32;
        let sf_height = self.height as f32 / camera.get_height() as f32;
        let sf = sf_width.min(sf_height).ceil() as u32;

        debug!(
            "self.width: {}, camera.width: {}",
            self.width,
            camera.get_width()
        );
        debug!("sf_width: {}", sf_width);
        debug!("sf_height: {}", sf_height);
        debug!("sf: {}", sf);

        let visible_width = camera.get_width() * sf;
        let visible_height = camera.get_height() * sf;

        let mut visible_pixelmap: Vec<[u8; 4]> =
            vec![[0; 4]; (visible_width * visible_height) as usize];

        for y in 0..camera.get_height() {
            for x in 0..camera.get_width() {
                let o_idx = self.get_idx((
                    camera.get_x() as isize + x as isize,
                    camera.get_y() as isize + y as isize,
                ));
                let pixel_value = self.pixelmap[o_idx];

                for dy in 0..sf {
                    for dx in 0..sf {
                        let s_x = (x * sf + dx) as usize;
                        let s_y = (y * sf + dy) as usize;
                        let s_idx = s_y * visible_width as usize + s_x;

                        visible_pixelmap[s_idx] = pixel_value;
                    }
                }
            }
        }

        visible_pixelmap
    }
}

impl Texture for GlobalPixelMap {
    fn render(&self, pixels: &mut [u8]) {
        debug!("Rendering GlobalPixelMap");
        // TODO: Apply scaling
        // let pixelmap = self.scale(1);
        // let pixelmap = self.extract_visible_region(self.camera.clone());
        let pixelmap = self.extract_and_scale_visible_region(&self.camera);

        // Render buffer to texture
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
    }

    fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}
