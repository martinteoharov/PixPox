use log::{debug, error};
use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum CellType {
    WATER,
    FIRE,
    STONE,
    WOOD,
    EMPTY
}

#[derive(Clone)]
pub struct CellRealm {
    height: u32,      // real height
    width: u32,       // real width
    cells: Vec<CellType>, // real cells
}

impl CellRealm {
    pub fn new(height: u32, width: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<CellType> = Vec::new();

        for _ in 0..height {
            for _ in 0..width {
                let cell_type = CellType::WATER;
                cells.push(cell_type);
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

    fn next_state_cell(&self, pos: (i32, i32)) -> CellType {
        CellType::EMPTY
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<CellType> = Vec::with_capacity(self.cells.len());

        self.cells
            .par_iter()
            .enumerate()
            .map(|(index, state)| {
                self.next_state_cell(self.get_pos(index as i32))
            })
            .collect_into_vec(&mut cells_next);

        // self.cells = cells_next;
        std::mem::swap(&mut self.cells, &mut cells_next);
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells
            .iter()
            .map(|state| {
                if *state == CellType::EMPTY {
                    [0, 0, 0, 255]
                } else if *state == CellType::WATER {
                    [0, 0, 255, 255]
                } else {
                    [0, 0, 0, 0]
                }
            })
            .collect()
    }
}
