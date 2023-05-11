use std::{collections::HashMap, sync::RwLock};

use imgui::InputFloat2;
use log::{debug, error};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World, InputHandler,
};
use pixpox_utils::conway::ConwayGrid;
use winit::dpi::{LogicalPosition, Position};
use winit_input_helper::WinitInputHelper;

use crate::global_pixel_map::GlobalPixelMap;

// Cell
#[derive(Copy, Clone)]
pub struct Cell {
    entity_id: usize,
    label: &'static str,

    pos: (isize, isize),
    state: bool,
    heat: u8,
    color: [u8; 4],
    change: bool
}

impl Cell {
    pub fn new(entity_id: usize, pos: (isize, isize), state: bool) -> Self {
        let color = if state {
            [255, 0, 0, 255]
        } else {
            [0, 0, 0, 255]
        };

        Self {
            entity_id,
            label: "Cell",
            pos,
            state,
            heat: 0,
            color,
            change: false
        }
    }
}

impl Label for Cell {
    fn label(&mut self) -> &'static str {
        return self.label;
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
    fn update(&mut self, rw_storage: &RwLock<Storage>, input: &InputHandler) {
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

            pixelmap.draw_pos((self.pos.0, self.pos.1), self.color);
        }
    }
}
