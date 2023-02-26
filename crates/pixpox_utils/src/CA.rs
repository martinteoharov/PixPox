use log::{debug, error};
use rand::Rng;
use std::collections::HashMap;

pub struct ConwayGrid {
    height: u32, // real height
    width: u32,  // real width
    cells: Vec<bool>,
}

impl ConwayGrid {
    pub fn new(height: u32, width: u32, gen_chance: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<bool> = Vec::new();

        for _ in 0..width {
            cells.push(false);
        }

        for _ in 0..height {
            cells.push(false);
            for _ in 0..width {
                let alive = rng.gen_bool(gen_chance);
                cells.push(alive);
            }
            cells.push(false);
        }

        for _ in 0..width {
            cells.push(false);
        }

        Self {
            cells,
            width: width + 2,
            height: height + 2,
        }
    }

    fn get_idx(&self, pos: (i32, i32)) -> usize {
        let idx = (pos.1 + 1) * self.width as i32 + (pos.0 + 1);
        idx as usize
    }

    pub fn set_cell(&mut self, pos: (i32, i32), state: bool) {
        let idx = self.get_idx(pos);
        self.cells[idx] = state;
    }

    pub fn count_neibs(&self, pos: (i32, i32)) -> usize {
        self.cells[self.get_idx((pos.0 - 1, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0 - 1, pos.1))] as usize
            + self.cells[self.get_idx((pos.0 - 1, pos.1 + 1))] as usize
            + self.cells[self.get_idx((pos.0, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0, pos.1 + 1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1 + 1))] as usize
    }
}
