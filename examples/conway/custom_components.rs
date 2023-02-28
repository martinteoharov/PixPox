use std::{collections::HashMap, sync::RwLock};

use log::{debug, error};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World,
};
use pixpox_utils::ConwayGrid;
use winit::dpi::{LogicalPosition, Position};

use crate::GlobalPixelMap;

#[derive(Clone)]
pub struct ConwayGridComponent {
    inner: ConwayGrid,
}

impl ConwayGridComponent {
    pub fn new(height: u32, width: u32, gen_chance: f64) -> Self {
        Self {
            inner: ConwayGrid::new(height, width, gen_chance),
        }
    }
}

impl Run for ConwayGridComponent {
    fn run(&mut self, _storage: &pixpox_ecs::Storage) {
        self.inner.next_state();
    }
}

impl Update for ConwayGridComponent {
    fn update(&mut self, storage: &RwLock<pixpox_ecs::Storage>) {
        let mut storage = storage.write().unwrap();

        // Fetch PixelMap
        let pixelmap = storage
            .query_storage_mut::<GlobalPixelMap>("pixelmap")
            .expect("Could not query Pixel Map");
        
        pixelmap.draw_flat_vec(&mut self.inner.get_color_vec());
    }
}

impl Label for ConwayGridComponent {
    fn label(&mut self) -> &'static str {
        "ConwayGrid"
    }
}

// Cell
#[derive(Copy, Clone)]
pub struct Cell {
    entity_id: usize,

    pos: (i32, i32),
    state: bool,
    heat: u8,
    color: [u8; 4],
    change: bool,
}

impl Cell {
    pub fn new(entity_id: usize, pos: (i32, i32), state: bool) -> Self {
        let color = if state {
            [255, 0, 0, 255]
        } else {
            [0, 0, 0, 255]
        };

        Self {
            entity_id,
            pos,
            state,
            heat: 0,
            color,
            change: false,
        }
    }
}

impl Label for Cell {
    fn label(&mut self) -> &'static str {
        "Cell"
    }
}

impl Run for Cell {
    fn run(&mut self, storage: &Storage) {
        let optim_grid = storage
            .query_storage::<ConwayGrid>("optim_grid")
            .expect("Could not query optim_grid");

        let neibs = optim_grid.count_neibs(self.pos);
        // error!("neibs: {}", neibs);

        if self.state {
            self.state = neibs == 2 || neibs == 3;
        } else {
            self.state = neibs == 3;
        }

        self.heat = if self.state == true {
            255
        } else if self.heat > 0 {
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
                .query_storage_mut::<ConwayGrid>("optim_grid")
                .expect("Could not get optim_grid");

            grid.set_cell(self.pos, self.state);

            let pixelmap = storage
                .query_storage_mut::<GlobalPixelMap>("pixelmap")
                .expect("Could not query Pixel Map");

            pixelmap.draw_pos((self.pos.0 as u32, self.pos.1 as u32), self.color);
        }
    }
}
