use crate::Camera;

#[derive(Debug)]
pub struct GlobalPixelMap {
    pixelmap: Vec<[u8; 4]>,
    window_width: u32,
    window_height: u32,
    camera: Camera,
}