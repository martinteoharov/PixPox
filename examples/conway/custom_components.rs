use std::any::Any;

use log::{info, debug};

use pixpox_app::App;
use pixpox_ecs::{entity::Entity, Label, Run};
use winit::dpi::{LogicalPosition, Position};


// Cell
#[derive(Copy, Clone)]
pub struct Cell {
    pos: LogicalPosition<u32>,
    alive: bool,
    pub heat: u8,
    label: &'static str,
}

impl Cell {
    pub fn new(pos: LogicalPosition<u32>, alive: bool) -> Self {
        Self {
            pos,
            alive,
            heat: 0,
            label: "Cell",
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            pos: LogicalPosition { x: 0, y: 0 },
            alive: false,
            heat: 0,
            label: "Cell"
        }
    }
}

impl Label for Cell {
    fn label(&mut self) -> &'static str {
        return self.label;
    }
}

impl Run for Cell {
    fn run(&mut self) {
        debug!("Running component {}", self.label);
        // self.alive = false;
    }
}