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

    fn update_next(
        &self,
        x: isize,
        y: isize,
        next_x: isize,
        next_y: isize,
        next_cell: Cell,
        next_positions: &mut Vec<(isize, isize)>,
        next_states: &mut Vec<Cell>,
    ) {
        let idx = self.get_idx((x, y));
        next_positions[idx] = (next_x, next_y);
        next_states[idx] = next_cell;
    }

    fn is_valid_pos(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    fn next_state_cell(
        &mut self,
        x: isize,
        y: isize,
        cell: Cell,
        next_positions: &mut Vec<(isize, isize)>,
        next_states: &mut Vec<Cell>,
    ) {
        match cell {
            Cell::SAND => {
                let dirs = [(0, 1), (-1, 1), (1, 1)]; //down, left-down, right-down
                for (dx, dy) in dirs.iter() {
                    let next_x = x + dx;
                    let next_y = y + dy;
                    if self.is_valid_pos(next_x, next_y)
                        && (self.cells[self.get_idx((next_x, next_y))] == Cell::EMPTY
                            || self.cells[self.get_idx((next_x, next_y))] == Cell::WATER)
                    {
                        if self.cells[self.get_idx((next_x, next_y))] == Cell::WATER {
                            self.update_next(x, y, x, y, Cell::WATER, next_positions, next_states);
                        }
                        self.update_next(
                            x,
                            y,
                            next_x,
                            next_y,
                            Cell::SAND,
                            next_positions,
                            next_states,
                        );
                        return;
                    }
                }
                // If no suitable spot was found, remain in the same position
                self.update_next(x, y, x, y, Cell::SAND, next_positions, next_states);
            },

            Cell::WATER => {
                let dirs = [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)]; //down, left-down, right-down, left, right
                for (dx, dy) in dirs.iter() {
                    let next_x = x + dx;
                    let next_y = y + dy;
                    if self.is_valid_pos(next_x, next_y)
                        && self.cells[self.get_idx((next_x, next_y))] == Cell::EMPTY
                    {
                        self.update_next(
                            x,
                            y,
                            next_x,
                            next_y,
                            Cell::WATER,
                            next_positions,
                            next_states,
                        );
                        return;
                    }
                }
                // If no suitable spot was found, remain in the same position
                self.update_next(x, y, x, y, Cell::WATER, next_positions, next_states);
            },
            _ => {
                // For any other cell type, it stays in the same position and state
                self.update_next(x, y, x, y, cell, next_positions, next_states);
            },
        }
    }

    /// Updates the cells vec to the next logical state
    pub fn next_state(&mut self) {
        let mut cells_next: Vec<Cell> = vec![Cell::EMPTY; self.cells.len()];
        let mut next_positions: Vec<(isize, isize)> = vec![(0, 0); self.cells.len()];
        let mut next_states: Vec<Cell> = self.cells.clone();

        for y in (0..self.height as isize).rev() {
            for x in (0..self.width as isize).rev() {
                let cell = self.cells[self.get_idx((x, y))];
                self.next_state_cell(x, y, cell, &mut next_positions, &mut next_states);
            }
        }

        for idx in 0..self.cells.len() {
            let next_pos = next_positions[idx];
            let next_idx = self.get_idx(next_pos);
            if cells_next[next_idx] == Cell::EMPTY {
                cells_next[next_idx] = next_states[idx];
            }
        }

        self.cells = cells_next;
    }

    pub fn get_color_vec(&mut self) -> Vec<[u8; 4]> {
        self.cells.iter().map(|cell| cell.get_color()).collect()
    }
}
