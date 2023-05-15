use crate::letters;
use log::{debug, error};
use rand::{rngs::StdRng, Rng, SeedableRng};
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

    pub fn new_pixpox(height: u32, width: u32, gen_chance: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<bool> = vec![false; (height * width) as usize];

        for y in 0..height {
            for x in 0..width {
                let mut alive = rng.gen_bool(gen_chance);

                if x > 100 && x < 200 {
                    if y > 70 && y < 140 {
                        alive = false;
                    }
                }

                let idx = (y * width) + x;
                cells[idx as usize] = alive;
            }
        }

        // Calculate the starting position for the first letter ('P') to center "PIXPOX"
        let pixpox_width = 5 * 6 + 1 * 5; // Each letter is 5 cells wide, and there are 5 gaps between the letters
        let pixpox_height = 5; // Each letter is 5 cells tall
        let start_x = (width - pixpox_width) / 2;
        let start_y = (height - pixpox_height) / 2;

        // Draw "PIXPOX" in the middle
        letters::draw_p(
            &mut cells,
            width as usize,
            start_x as usize,
            start_y as usize,
        );
        letters::draw_i(
            &mut cells,
            width as usize,
            (start_x + 6) as usize,
            start_y as usize,
        );
        letters::draw_x(
            &mut cells,
            width as usize,
            (start_x + 12) as usize,
            start_y as usize,
        );
        letters::draw_p(
            &mut cells,
            width as usize,
            (start_x + 18) as usize,
            start_y as usize,
        );
        letters::draw_o(
            &mut cells,
            width as usize,
            (start_x + 24) as usize,
            start_y as usize,
        );
        letters::draw_x(
            &mut cells,
            width as usize,
            (start_x + 30) as usize,
            start_y as usize,
        );

        Self {
            cells,
            width: width,
            height: height,
        }
    }

    /// Private function to map between real pos and idx
    fn get_idx(&self, pos: (isize, isize)) -> usize {
        let idx = pos.1 * self.width as isize + pos.0;
        idx as usize
    }

    // Private function to map idx to real pos
    fn get_pos(&self, idx: isize) -> (isize, isize) {
        let x = idx % self.width as isize;
        let y = idx / self.width as isize;

        (x, y)
    }

    pub fn clear_grid(&mut self) {
        self.cells = self.cells.iter().map(|_| false).collect();
    }

    pub fn set_cell(&mut self, pos: (isize, isize), state: bool) {
        let idx = self.get_idx(pos);
        self.cells[idx] = state;
    }

    pub fn count_neibs(&self, pos: (isize, isize)) -> usize {
        if pos.0 == 0
            || pos.0 == self.width as isize - 1
            || pos.1 == 0
            || pos.1 == self.height as isize - 1
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

    /// Set pos
    pub fn set_pos(&mut self, pos: (isize, isize), cell: bool) {
        let idx = self.get_idx(pos);
        self.cells[idx] = cell;
    }

    /// Implement Bresenham's line algorithm
    pub fn set_line(&mut self, pos1: (isize, isize), pos2: (isize, isize), cell: bool) {
        let (x1, y1) = pos1;
        let (x2, y2) = pos2;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            self.set_pos((x, y), cell.clone());

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<bool> = Vec::with_capacity(self.cells.len());

        self.cells
            .par_iter()
            .enumerate()
            .map(|(index, state)| {
                let pos = self.get_pos(index as isize);
                let neibs = self.count_neibs(pos);

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
                    [0, 0, 200, 255]
                } else {
                    [0, 0, 0, 255]
                }
            })
            .collect()
    }
}
