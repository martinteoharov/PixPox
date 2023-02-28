use log::{debug, error};
use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ConwayGrid {
    height: u32,      // real height
    width: u32,       // real width
    cells: Vec<bool>, // real cells
}

impl ConwayGrid {
    pub fn new(height: u32, width: u32, gen_chance: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<bool> = Vec::new();

        // Create a layer of 0s
        for _ in 0..height {
            for _ in 0..width {
                let alive = rng.gen_bool(gen_chance);
                cells.push(alive);
            }
        }

        Self {
            cells,
            width: width,
            height: height,
        }
    }

    /// Private function to map between real pos and idx
    fn get_idx(&self, pos: (i32, i32)) -> usize {
        let idx = pos.1 * self.width as i32 + pos.0;
        idx as usize
    }

    // Private function to map idx to real pos
    fn get_pos(&self, idx: i32) -> (i32, i32) {
        let x = idx % self.width as i32;
        let y = idx / self.width as i32;

        (x, y)
    }

    pub fn set_cell(&mut self, pos: (i32, i32), state: bool) {
        let idx = self.get_idx(pos);
        self.cells[idx] = state;
    }

    pub fn count_neibs(&self, pos: (i32, i32)) -> usize {
        if pos.0 == 0
            || pos.0 == self.width as i32 - 1
            || pos.1 == 0
            || pos.1 == self.height as i32 - 1
        {
            return 0;
        }

        self.cells[self.get_idx((pos.0 - 1, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0 - 1, pos.1))] as usize
            + self.cells[self.get_idx((pos.0 - 1, pos.1 + 1))] as usize
            + self.cells[self.get_idx((pos.0, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0, pos.1 + 1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1 - 1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1))] as usize
            + self.cells[self.get_idx((pos.0 + 1, pos.1 + 1))] as usize
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<bool> = Vec::with_capacity(self.cells.len());

        self.cells
            .par_iter()
            .enumerate()
            .map(|(index, state)| {
                let neibs = self.count_neibs(self.get_pos(index as i32));

                let new_state = if *state {
                    neibs == 2 || neibs == 3
                } else {
                    neibs == 3
                };

                new_state
            })
            .collect_into_vec(&mut cells_next);

        // self.cells = cells_next;
        std::mem::swap(&mut self.cells, &mut cells_next);
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells
            .iter()
            .map(|state| {
                if *state {
                    [255, 0, 0, 255]
                } else {
                    [0, 0, 0, 255]
                }
            })
            .collect()
    }
}
