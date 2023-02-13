use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::atomic::AtomicUsize};

use interner::global::GlobalPool;
use lazy_static::lazy_static;
use log::{debug, error, info};

use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World,
};
use winit::dpi::{LogicalPosition, Position};

use crate::GlobalPixelMap;

// Cell
#[derive(Copy, Clone)]
pub struct Cell {
    entity_id: usize,
    label: &'static str,

    pos: LogicalPosition<u32>,
    color: [u8; 4],
    state: bool,
    heat: u8,
}

impl Cell {
    pub fn new(entity_id: usize, pos: LogicalPosition<u32>, alive: bool) -> Self {
        let color = if alive == true {
            [255, 0, 0, 255]
        } else {
            [0, 0, 0, 100]
        };

        Self {
            entity_id: entity_id,
            pos: pos,
            state: alive,
            heat: 0,
            label: "Cell",
            color: color,
        }
    }

    fn count_neibs(&mut self, storage: &mut Storage) -> u8 {
        let grid = storage
            .query_storage::<HashMap<LogicalPosition<u32>, bool>>("grid")
            .expect("Could not query grid");

        let (x, y) = (self.pos.x, self.pos.y);

        if x == 0 || y == 0 || x >= 399 || y >= 299 {
            return 0;
        }

        *grid.get(&LogicalPosition::new(x, y - 1)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x, y + 1)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x + 1, y - 1)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x + 1, y)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x + 1, y + 1)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x - 1, y - 1)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x - 1, y)).unwrap() as u8
            + *grid.get(&LogicalPosition::new(x - 1, y + 1)).unwrap() as u8
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
        let neibs = self.count_neibs(storage);
        if self.state == true {
            self.state = neibs == 2 || neibs == 3;
        } else {
            self.state = neibs == 3;
        }

        if true {
            {
                let pixelmap = storage
                    .query_storage::<GlobalPixelMap>("pixelmap")
                    .expect("Could not query Pixel Map");

                debug!("State has changed between frames - should draw");
                pixelmap.draw_pos(self.pos, self.color);
            }
        }
    }
}

impl Update for Cell {
    fn update(&mut self, storage: &mut Storage) {
        self.heat = if self.state == true {
            255
        } else if self.heat > 10 {
            self.heat - 10
        } else {
            0
        };

        // Update cell color
        self.color = if self.state == true {
            [255, 0, 0, 255]
        } else {
            [self.heat, 0, 0, 50]
        };

        // Fetch & Update cell in grid
        let grid = storage
            .query_storage::<HashMap<LogicalPosition<u32>, bool>>("grid")
            .expect("Could not get grid");

        let mut grid_pixel = grid.get_mut(&self.pos).expect("Could not get grid_pixel");
        debug!("state: {}, next_state: {}", grid_pixel, self.state);
        *grid_pixel = self.state;
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
