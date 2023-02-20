use std::{collections::HashMap, sync::RwLock};

use log::debug;
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
    change: bool,
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
            change: false,
        }
    }

    fn count_neibs(&mut self, storage: &Storage) -> u8 {
        let grid = storage
            .query_storage::<HashMap<LogicalPosition<u32>, bool>>("grid")
            .expect("Could not query grid");

        let (x, y) = (self.pos.x, self.pos.y);
        let (width, height) = storage.query_storage::<(u32, u32)>("grid-size").expect("boom boom cock");

        if x == 0 || y == 0 || x >= *width - 1 || y >= *height - 1 {
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
    fn run(&mut self, storage: &Storage) {
        let neibs = self.count_neibs(storage);

        if self.state == true {
            self.state = neibs == 2 || neibs == 3;
        } else {
            self.state = neibs == 3;
        }

        self.heat = if self.state == true {
            255
        } else if self.heat > 10 {
            self.heat - 1
        } else {
            0
        };

        // Update cell color
        let old_color = self.color;
        self.color = if self.state == true {
            [255, 0, 0, 255]
        } else {
            [self.heat, 0, 0, 50]
        };

        self.change = old_color != self.color;
    }
}

impl Update for Cell {
    fn update(&mut self, rw_storage: &RwLock<Storage>) {
        if self.change {
            let mut storage = rw_storage.write().unwrap();

            // Fetch & Update cell in grid
            let grid = storage
                .query_storage_mut::<HashMap<LogicalPosition<u32>, bool>>("grid")
                .expect("Could not get grid");

            let grid_pixel = grid.get_mut(&self.pos).expect("Could not get grid_pixel");
            debug!("state: {}, next_state: {}", grid_pixel, self.state);
            *grid_pixel = self.state;

            let pixelmap = storage
                .query_storage_mut::<GlobalPixelMap>("pixelmap")
                .expect("Could not query Pixel Map");

            pixelmap.draw_pos(self.pos, self.color);
        }
    }
}