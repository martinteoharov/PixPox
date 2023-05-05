use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    EMPTY,
    SAND,
    WATER,
    SOLID,
}

impl Cell {
    pub fn get_color(&self) -> [u8; 4] {
        match self {
            Cell::EMPTY => [0, 0, 0, 0],
            Cell::SAND => [255, 255, 0, 255],
            Cell::WATER => [0, 0, 255, 255],
            Cell::SOLID => [255, 255, 255, 255],
        }
    }
}

#[derive(Clone)]
pub struct CellRealm {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}

impl CellRealm {
    pub fn new(height: u32, width: u32) -> Self {
        let mut cells: Vec<Cell> = Vec::new();

        for _ in 0..height {
            for _ in 0..width {
                cells.push(Cell::EMPTY);
            }
        }

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

    pub fn set_pos(&mut self, pos: (isize, isize), cell: Cell) {
        let idx = self.get_idx(pos);
        self.cells[idx] = cell;
    }

    /// Implement Bresenham's line algorithm
    pub fn set_line(&mut self, pos1: (isize, isize), pos2: (isize, isize), cell: Cell) {
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

    // Circle paint brush
    pub fn set_circle(&mut self, center: (isize, isize), radius: isize, cell: Cell) {
        let (cx, cy) = center;
        let r_squared = radius * radius;

        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared <= r_squared {
                    self.set_pos((x, y), cell.clone());
                }
            }
        }
    }

    pub fn clear_grid(&mut self) {
        self.cells = self.cells.iter().map(|_| Cell::EMPTY).collect();
    }

    fn next_state_cell(
        &mut self,
        x: isize,
        y: isize,
        cell: Cell,
        cells_next: &mut Vec<Cell>,
    ) -> Cell {
        match cell {
            Cell::SAND => {
                if y < (self.height as isize - 1)
                    && self.cells[self.get_idx((x, y + 1))] == Cell::EMPTY
                {
                    cells_next[self.get_idx((x, y + 1))] = Cell::SAND;
                    Cell::EMPTY
                } else if x > 0
                    && y < (self.height as isize - 1)
                    && self.cells[self.get_idx((x - 1, y + 1))] == Cell::EMPTY
                {
                    cells_next[self.get_idx((x - 1, y + 1))] = Cell::SAND;
                    Cell::EMPTY
                } else if x < (self.width as isize - 1)
                    && y < (self.height as isize - 1)
                    && self.cells[self.get_idx((x + 1, y + 1))] == Cell::EMPTY
                {
                    cells_next[self.get_idx((x + 1, y + 1))] = Cell::SAND;
                    Cell::EMPTY
                } else {
                    cell
                }
            },
            Cell::WATER => {
                let mut rng = rand::thread_rng();
                let is_empty_below = y < (self.height as isize - 1)
                    && self.cells[self.get_idx((x, y + 1))] == Cell::EMPTY;
                let is_empty_left = x > 0 && self.cells[self.get_idx((x - 1, y))] == Cell::EMPTY;
                let is_empty_right = x < (self.width as isize - 1)
                    && self.cells[self.get_idx((x + 1, y))] == Cell::EMPTY;
                let is_water_left = x > 0 && self.cells[self.get_idx((x - 1, y))] == Cell::WATER;
                let is_water_right = x < (self.width as isize - 1)
                    && self.cells[self.get_idx((x + 1, y))] == Cell::WATER;

                if is_empty_below {
                    cells_next[self.get_idx((x, y + 1))] = Cell::WATER;
                    Cell::EMPTY
                } else if is_empty_left && is_empty_right {
                    let direction = rng.gen_range(0..2); // Randomly choose between 0 and 1
                    if direction == 0 {
                        cells_next[self.get_idx((x - 1, y))] = Cell::WATER;
                    } else {
                        cells_next[self.get_idx((x + 1, y))] = Cell::WATER;
                    }
                    Cell::EMPTY
                } else if is_empty_left {
                    cells_next[self.get_idx((x - 1, y))] = Cell::WATER;
                    Cell::EMPTY
                } else if is_empty_right {
                    cells_next[self.get_idx((x + 1, y))] = Cell::WATER;
                    Cell::EMPTY
                } else if is_water_left && is_water_right {
                    // Swap water cells
                    let direction = rng.gen_range(0..2); // Randomly choose between 0 and 1
                    if direction == 0 {
                        cells_next[self.get_idx((x - 1, y))] = Cell::WATER;
                    } else {
                        cells_next[self.get_idx((x + 1, y))] = Cell::WATER;
                    }
                    cell
                } else if is_water_left {
                    cells_next[self.get_idx((x - 1, y))] = Cell::WATER;
                    cell
                } else if is_water_right {
                    cells_next[self.get_idx((x + 1, y))] = Cell::WATER;
                    cell
                } else {
                    cell
                }
            },
            _ => cell,
        }
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<Cell> = self.cells.clone();

        for y in (0..self.height as isize).rev() {
            for x in 0..self.width as isize {
                let cell = self.cells[self.get_idx((x, y))];
                let next_cell = self.next_state_cell(x, y, cell, &mut cells_next);
                cells_next[self.get_idx((x, y))] = next_cell;
            }
        }

        std::mem::swap(&mut self.cells, &mut cells_next);
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells.iter().map(|cell| cell.get_color()).collect()
    }
}
