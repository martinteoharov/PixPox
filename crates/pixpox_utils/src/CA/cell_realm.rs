use log::{debug, error, info};
use rand::Rng;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::collections::HashMap;

const MAX_MASS: f64 = 1.0;
const MAX_COMPRESS: f64 = 0.02;
const MIN_MASS: f64 = 0.0001;

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
    mass: f64,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        let mass = match cell_type {
            CellType::WATER => 0.5,
            CellType::FIRE => 0.0,
            CellType::STONE => 1.0,
            CellType::WOOD => 1.0,
            CellType::EMPTY => 0.0,
        };

        Self { cell_type, mass }
    }

    pub fn get_color(&self) -> [u8; 4] {
        match self.cell_type {
            CellType::WATER => {
                let b = std::cmp::min((255.0 * self.mass) as usize, 255) as u8;
                [0, 0, b, 255]
            },
            CellType::FIRE => [255, 0, 0, 0],
            CellType::STONE => [100, 100, 100, 255],
            CellType::WOOD => [139, 69, 19, 255],
            CellType::EMPTY => [0, 0, 0, 0],
        }
    }

    fn calculate_flow_outgoing(&self, source_cell: &mut Cell, target_cell: Cell) -> (Cell, Cell) {
        match source_cell.cell_type {
            CellType::WATER => {
                let max_fill = MAX_MASS - target_cell.mass;
                let diff = max_fill.min(source_cell.mass);
                source_cell.mass -= diff;

                // error!("mass: {}", source_cell.mass);

                (*source_cell, target_cell)
            },
            CellType::FIRE => (*source_cell, target_cell),
            CellType::STONE => (*source_cell, target_cell),
            CellType::WOOD => (*source_cell, target_cell),
            CellType::EMPTY => (*source_cell, target_cell),
        }
    }

    fn calculate_flow_incoming(&self, source_cell: Cell, target_cell: &mut Cell) -> (Cell, Cell) {
        match target_cell.cell_type {
            CellType::WATER => {
                let max_fill = MAX_MASS - target_cell.mass;
                let diff = max_fill.min(source_cell.mass);
                target_cell.mass += diff;

                (source_cell, *target_cell)
            },
            CellType::FIRE => (source_cell, *target_cell),
            CellType::STONE => (source_cell, *target_cell),
            CellType::WOOD => (source_cell, *target_cell),
            CellType::EMPTY => {
                target_cell.cell_type = CellType::WATER;
                let max_fill = MAX_MASS - target_cell.mass;
                let diff = max_fill.min(source_cell.mass);
                target_cell.mass += diff;

                (source_cell, *target_cell)
            },
        }
    }

    pub fn next_state(&self, n: Vec<Cell>) -> Cell {
        match self.cell_type {
            CellType::WATER => {
                let mut cell = Cell::new(self.cell_type);

                // Calculate outgoing flow
                self.calculate_flow_outgoing(&mut cell, n[6]); // cell below
                self.calculate_flow_outgoing(&mut cell, n[4]); // cell right
                self.calculate_flow_outgoing(&mut cell, n[3]); // cell right

                // Calculate incoming flow
                self.calculate_flow_incoming(n[1], &mut cell); // cell above
                self.calculate_flow_incoming(n[3], &mut cell); // cell left
                self.calculate_flow_incoming(n[4], &mut cell); // cell left


                cell
            },
            CellType::FIRE => Cell::new(CellType::FIRE),
            CellType::STONE => Cell::new(CellType::STONE),
            CellType::WOOD => Cell::new(CellType::WOOD),
            CellType::EMPTY => {
                let mut cell = Cell::new(self.cell_type);

                // Calculate incoming flow
                self.calculate_flow_incoming(n[1], &mut cell); // cell above
                self.calculate_flow_incoming(n[3], &mut cell); // cell left
                self.calculate_flow_incoming(n[4], &mut cell); // cell right

                cell
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
                let cell = Cell::new(CellType::EMPTY);
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
        self.cells[idx] = Cell::new(cell);
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
            return Cell::new(CellType::EMPTY);
        }

        let neibs = vec![
            self.cells[self.get_idx((pos.0 - 1, pos.1 - 1))],
            self.cells[self.get_idx((pos.0 - 1, pos.1))],
            self.cells[self.get_idx((pos.0 - 1, pos.1 + 1))],
            self.cells[self.get_idx((pos.0, pos.1 - 1))],
            self.cells[self.get_idx((pos.0, pos.1 + 1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1 - 1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1))],
            self.cells[self.get_idx((pos.0 + 1, pos.1 + 1))],
        ];

        cell.next_state(neibs)
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<Cell> = Vec::with_capacity(self.cells.len());

        self.cells
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
