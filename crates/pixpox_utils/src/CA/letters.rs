/// This file contains helper functions used to write letters to the cell grid
/// Used in the presentation demo

pub fn draw_p(cells: &mut Vec<bool>, width: usize, start_x: usize, start_y: usize) {
    let p: [[bool; 3]; 5] = [
        [true, true, false],
        [true, false, true],
        [true, true, false],
        [true, false, false],
        [true, false, false],
    ];

    for y in 0..5 {
        for x in 0..3 {
            cells[((start_y + y) * width + (start_x + x)) as usize] = p[y][x];
        }
    }
}

pub fn draw_i(cells: &mut Vec<bool>, width: usize, start_x: usize, start_y: usize) {
    let i: [[bool; 3]; 5] = [
        [false, true, false],
        [false, true, false],
        [false, true, false],
        [false, true, false],
        [false, true, false],
    ];

    for y in 0..5 {
        for x in 0..3 {
            cells[((start_y + y) * width + (start_x + x)) as usize] = i[y][x];
        }
    }
}

pub fn draw_o(cells: &mut Vec<bool>, width: usize, start_x: usize, start_y: usize) {
    let o: [[bool; 3]; 5] = [
        [true, true, true],
        [true, false, true],
        [true, false, true],
        [true, false, true],
        [true, true, true],
    ];

    for y in 0..5 {
        for x in 0..3 {
            cells[((start_y + y) * width + (start_x + x)) as usize] = o[y][x];
        }
    }
}


pub fn draw_x(cells: &mut Vec<bool>, width: usize, start_x: usize, start_y: usize) {
    let lx: [[bool; 3]; 5] = [
        [true, false, true],
        [false, true, false],
        [false, true, false],
        [false, true, false],
        [true, false, true],
    ];

    for y in 0..5 {
        for x in 0..3 {
            cells[((start_y + y) * width + (start_x + x)) as usize] = lx[y][x];
        }
    }
}
