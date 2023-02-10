use std::{cell::RefCell, collections::HashMap, rc::Rc};

use log::{debug, info};

use pixpox_app::App;
use pixpox_ecs::{entity::Entity, Label, Run, Storage, Texture, World};
use winit::dpi::{LogicalPosition, Position};

use crate::GlobalPixelMap;

// Cell
#[derive(Copy, Clone)]
pub struct Cell {
    entity_id: usize,
    label: &'static str,

    pos: LogicalPosition<u32>,
    color: [u8; 4],
    alive: bool,
    heat: u8,
}

impl Cell {
    pub fn new(entity_id: usize, pos: LogicalPosition<u32>, alive: bool) -> Self {
        let color: [u8; 4] = if alive {
            [0, 255, 255, 255]
        } else {
            [0, 0, 0, 255]
        };

        Self {
            entity_id,
            pos,
            alive,
            heat: 0,
            label: "Cell",
            color,
        }
    }
}

impl Label for Cell {
    fn label(&mut self) -> &'static str {
        return self.label;
    }
}

impl Run for Cell {
    fn run(&mut self, storage: &mut Storage) {
        // debug!("Running component {}", self.label);
        // self.alive = false;
        
        if let Some(pixelmap) = storage.query_storage::<GlobalPixelMap>("pixelmap") {
            pixelmap.set_pos(self.pos, self.color);
        }
    }
}

// TexturePixel
/*
#[derive(Copy, Clone)]
pub struct TexturePixel {
    pub entity_id: usize,
    label: &'static str,

    pub pos: LogicalPosition<u32>,
    pub color: [u8; 4],
}

impl TexturePixel {
    pub fn new(entity_id: usize, pos: LogicalPosition<u32>, color: [u8; 4]) -> Self {
        Self {
            entity_id,
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
    fn run(&mut self, storage: &mut Storage) {
        debug!("Running component {}", self.label);
        // self.alive = false;
    }
}

impl Texture for TexturePixel {
    fn color(&mut self, storage: &mut Storage) -> [u8; 4] {
        return self.color;
    }

    fn pos(&mut self) -> LogicalPosition<u32> {
        return self.pos;
    }
}
*/
