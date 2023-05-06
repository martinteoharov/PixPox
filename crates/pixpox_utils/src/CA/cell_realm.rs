use log::error;
use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    EMPTY,
    SAND,
    WATER,
    SOLID,
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_type: CellType,
    updated: bool,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            updated: false,
        }
    }

    pub fn get_color(&self) -> [u8; 4] {
        match self.cell_type {
            CellType::EMPTY => [0, 0, 0, 0],
            CellType::SAND => [255, 255, 0, 255],
            CellType::WATER => [0, 0, 255, 255],
            CellType::SOLID => [255, 255, 255, 255],
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
                cells.push(Cell::new(CellType::EMPTY));
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
        self.cells = self
            .cells
            .iter()
            .map(|_| Cell::new(CellType::EMPTY))
            .collect();
    }

    pub fn is_empty(&self, pos: (isize, isize)) -> bool {
        let idx = self.get_idx(pos);
        self.cells[idx].cell_type == CellType::EMPTY && !self.cells[idx].updated
    }

    fn next_state_cell(
        &mut self,
        x: isize,
        y: isize,
        cell: Cell,
        cells_next: &mut Vec<Cell>,
    ) -> Cell {

        let mut updated_cell = cell;
        updated_cell.updated = true;

        match cell.cell_type {
            CellType::SAND => {
                if y < (self.height as isize - 1) && self.is_empty((x, y + 1)) {
                    cells_next[self.get_idx((x, y + 1))] = Cell::new(CellType::SAND);
                    return Cell::new(CellType::EMPTY);
                } else if x > 0 && y < (self.height as isize - 1) && self.is_empty((x - 1, y + 1)) {
                    cells_next[self.get_idx((x - 1, y + 1))] = Cell::new(CellType::SAND);
                    return Cell::new(CellType::EMPTY);
                } else if x < (self.width as isize - 1)
                    && y < (self.height as isize - 1)
                    && self.is_empty((x + 1, y + 1))
                {
                    cells_next[self.get_idx((x + 1, y + 1))] = Cell::new(CellType::SAND);
                    return Cell::new(CellType::EMPTY);
                } else {
                    updated_cell
                }
            },
            CellType::WATER => {
                let mut rng = rand::thread_rng();
                let is_empty_below = y < (self.height as isize - 1) && self.is_empty((x, y + 1));
                let is_empty_left = x > 0 && self.is_empty((x - 1, y));
                let is_empty_right = x < (self.width as isize - 1) && self.is_empty((x + 1, y));

                // Check if water can go below
                if is_empty_below {
                    cells_next[self.get_idx((x, y + 1))] = Cell::new(CellType::WATER);
                    updated_cell.cell_type = CellType::EMPTY;
                    return updated_cell;
                }

                // Check if water can go left or right without overriding sand or water blocks
                if is_empty_left && is_empty_right {
                    let direction = rng.gen_range(0..2);
                    if direction == 0 {
                        cells_next[self.get_idx((x - 1, y))] = Cell::new(CellType::WATER);
                        updated_cell.cell_type = CellType::EMPTY;
                        return updated_cell;
                    } else {
                        cells_next[self.get_idx((x + 1, y))] = Cell::new(CellType::WATER);
                        updated_cell.cell_type = CellType::EMPTY;
                        return updated_cell;
                    }
                } else if is_empty_left {
                    cells_next[self.get_idx((x - 1, y))] = Cell::new(CellType::WATER);
                    updated_cell.cell_type = CellType::EMPTY;
                    return updated_cell;
                } else if is_empty_right {
                    cells_next[self.get_idx((x + 1, y))] = Cell::new(CellType::WATER);
                    updated_cell.cell_type = CellType::EMPTY;
                    return updated_cell;
                }
                
                updated_cell.cell_type = CellType::WATER;
                return updated_cell;
            },
            _ => updated_cell,
        }
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<Cell> = self.cells.clone();
        for y in (0..self.height as isize).rev() {
            for x in (0..self.width as isize) {
                let cell = self.cells[self.get_idx((x, y))];
                let next_cell = self.next_state_cell(x, y, cell, &mut cells_next);
                cells_next[self.get_idx((x, y))] = next_cell;
            }
        }

        std::mem::swap(&mut self.cells, &mut cells_next);

        // Reset updated flag
        for cell in self.cells.iter_mut() {
            cell.updated = false;
        }
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells.iter().map(|cell| cell.get_color()).collect()
    }
}
