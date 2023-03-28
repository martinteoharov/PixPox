use log::{debug, error, info};
use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    WATER,
    FIRE,
    STONE,
    WOOD,
    EMPTY,
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_type: CellType,
    flow: Option<(i32, i32)>,
}

impl Cell {
    pub fn new(cell_type: CellType, flow: Option<(i32, i32)>) -> Self {
        Self { cell_type, flow }
    }

    pub fn get_color(&self) -> [u8; 4] {
        match self.cell_type {
            CellType::WATER => [0, 0, 200, 255],
            CellType::FIRE => [255, 0, 0, 0],
            CellType::STONE => [100, 100, 100, 255],
            CellType::WOOD => [139, 69, 19, 255],
            CellType::EMPTY => [0, 0, 0, 0],
        }
    }

    pub fn next_state(&self, n: Vec<Cell>) -> Cell {
        match self.cell_type {
            CellType::WATER => {
                if n[6].cell_type == CellType::EMPTY {
                    return Cell::new(CellType::EMPTY, Some((0, 1)));
                }

                // If both left and right are empty
                if n[3].cell_type == CellType::EMPTY && n[4].cell_type == CellType::EMPTY {
                    let mut rng = rand::thread_rng();
                    let flow_x = if rng.gen_bool(0.5) { 1 } else { -1 };
                    return Cell::new(CellType::EMPTY, Some((flow_x, 0)));
                }

                if n[3].cell_type == CellType::WATER && n[4].cell_type == CellType::EMPTY {
                    return Cell::new(CellType::EMPTY, Some((1, 0)));
                }

                if n[3].cell_type == CellType::EMPTY && n[4].cell_type == CellType::WATER {
                    return Cell::new(CellType::EMPTY, Some((-1, 0)));
                }

                Cell::new(CellType::WATER, Some((0, 0)))
            },
            CellType::FIRE => Cell::new(CellType::FIRE, Some((0, 0))),
            CellType::STONE => Cell::new(CellType::STONE, Some((0, 0))),
            CellType::WOOD => Cell::new(CellType::WOOD, Some((0, 0))),
            CellType::EMPTY => {
                // Above
                if n[1].flow.unwrap().1 > 0 {
                    return Cell::new(CellType::WATER, Some((0, 0)));
                }

                // Left
                if n[3].flow.unwrap().0 > 0 {
                    return Cell::new(CellType::WATER, Some((0, 0)));
                }

                // Right
                if n[4].flow.unwrap().0 < 0 {
                    return Cell::new(CellType::WATER, Some((0, 0)));
                }

                Cell::new(CellType::EMPTY, Some((0, 0)))
            },
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
        let mut rng = rand::thread_rng();
        let mut cells: Vec<Cell> = Vec::new();

        for _ in 0..height {
            for _ in 0..width {
                let cell = Cell::new(CellType::EMPTY, None);
                cells.push(cell);
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

    pub fn set_pos(&mut self, pos: (isize, isize), cell: CellType) {
        let idx = self.get_idx(pos);
        self.cells[idx] = Cell::new(cell, None);
    }

    /// Implement Bresenham's line algorithm
    pub fn set_line(&mut self, pos1: (isize, isize), pos2: (isize, isize), cell: CellType) {
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

    fn next_state_cell(&self, pos: (isize, isize), cell: &Cell) -> Cell {
        if pos.0 == 0
            || pos.0 == self.width as isize - 1
            || pos.1 == 0
            || pos.1 == self.height as isize - 1
        {
            return Cell::new(CellType::EMPTY, None);
        }

        let neibs = vec![
            self.cells[self.get_idx((pos.0 - 1, pos.1 - 1))],
            self.cells[self.get_idx((pos.0, pos.1 - 1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1 - 1))],
            self.cells[self.get_idx((pos.0 - 1, pos.1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1))],
            self.cells[self.get_idx((pos.0 - 1, pos.1 + 1))],
            self.cells[self.get_idx((pos.0, pos.1 + 1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1 + 1))],
        ];

        cell.next_state(neibs)
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<Cell> = Vec::with_capacity(self.cells.len());

        self.cells
            .clone()
            .par_iter()
            .enumerate()
            .map(|(index, cell)| self.next_state_cell(self.get_pos(index as isize), cell))
            .collect_into_vec(&mut cells_next);

        // self.cells = cells_next;
        std::mem::swap(&mut self.cells, &mut cells_next);
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells.iter().map(|cell| cell.get_color()).collect()
    }
}
