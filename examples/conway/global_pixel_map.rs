use log::{debug, error};
use pixpox_ecs::{InputHandler, Texture, Update};

use crate::camera::Camera;

#[derive(Debug)]
pub struct GlobalPixelMap {
    pixelmap: Vec<[u8; 4]>,
    width: u32,
    height: u32,
    clear_color: [u8; 4],
    scale: u32,
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
    /// * `clear_color` - The color to fill the pixelmap with.
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
    pub fn new_empty(width: u32, height: u32, clear_color: [u8; 4], camera: Camera) -> Self {
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
            clear_color,
            scale: 1,
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
            camera.width as f32 / camera.height as f32
        );
        debug!(
            "self.width / self.height: {}",
            self.width as f32 / self.height as f32
        );

        // calculate scaling factor
        let mut sf = self.width / camera.width;
        sf = sf - sf % 2;

        let mut visible_pixelmap: Vec<[u8; 4]> = Vec::new();

        for y in camera.y..(camera.y + camera.height) {
            for x in camera.x..(camera.x + camera.width) {
                let idx = self.get_idx((x as isize, y as isize));
                visible_pixelmap.push(self.pixelmap[idx as usize]);
            }
        }

        let new_width = self.width as usize;
        let new_height = self.height as usize;

        let mut scaled = vec![[0; 4]; new_width * new_height];

        for y in 0..camera.height {
            for x in 0..camera.width {
                let o_idx = (camera.y + y) * self.width + (camera.x + x);
                let pixel_value = self.pixelmap[o_idx as usize];

                for dy in 0..sf {
                    for dx in 0..sf {
                        let s_x = (x * sf + dx) as usize;
                        let s_y = (y * sf + dy) as usize;
                        let s_idx = s_y * new_width + s_x;
                        scaled[s_idx] = pixel_value;
                    }
                }
            }
        }

        scaled
    }
}

impl Texture for GlobalPixelMap {
    fn render(&self, pixels: &mut [u8]) {
        debug!("Rendering GlobalPixelMap");
        // TODO: Apply scaling
        // let pixelmap = self.scale(1);
        let pixelmap = self.extract_visible_region(self.camera.clone());

        // Render buffer to texture
        let pixel_chunks = pixels.chunks_exact_mut(4);

        for (c, pix) in pixelmap.iter().zip(pixel_chunks) {
            pix.copy_from_slice(c);
        }
    }

    fn update(&mut self, input: &InputHandler) {
        let scroll_delta = input.winit.scroll_diff();
        log::debug!("update() mouse scroll delta: [{}]", scroll_delta);

        // if scroll up, zoom in
        if scroll_delta > 1.0 {
            self.camera.width *= 1.1;
            self.camera.height *= 1.1;
        }

        // if scroll down, zoom out
        if scroll_delta < -1.0 {
            self.camera.width -= 50;
            self.camera.height -= 50;
        }
    }

    fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}