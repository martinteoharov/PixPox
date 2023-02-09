use log::{info, debug};
use winit::dpi::LogicalPosition;

/*
 * Base traits that all components must implement
 */
pub trait Label {
    fn label(&mut self) -> &'static str;
}

pub trait Run {
    fn run(&mut self);
}

pub struct BaseComponent {
    label: &'static str,
}

impl Label for BaseComponent {
    fn label(&mut self) -> &'static str {
        {
            self.label
        }
    }
}

impl Run for BaseComponent {
    fn run(&mut self) {
        info!("Kur kapan");
    }
}

// Texture
#[derive(Copy, Clone)]
pub struct TexturePixel {
    pub pos: LogicalPosition<u32>,
    pub color: [u8; 4],
    label: &'static str,
}

impl TexturePixel {
    pub fn new(pos: LogicalPosition<u32>, color: [u8; 4]) -> Self {

        Self {
            pos,
            color,
            label: "TexturePixel",
        }
    }
}

impl Label for TexturePixel {
    fn label(&mut self) -> &'static str {
        return self.label;
    }
}

impl Run for TexturePixel {
    fn run(&mut self) {
        debug!("Running component {}", self.label);
        // self.alive = false;
    }
}