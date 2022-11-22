use log::info;
use pixpox_ecs::{Label, Run};
use winit::dpi::{LogicalPosition, Position};

// Player
#[derive(Copy, Clone)]
pub struct Player {
    label: &'static str,
    times_called: i32
}

impl Player {
    pub fn new() -> Self {
        Self { label: "Player", times_called: 0 }
    }
}

impl Label for Player {
    fn label(&mut self) -> &'static str {
        {
            self.label
        }
    }
}

impl Run for Player {
    fn run(&mut self) {
        info!("Running component {}, for {} time", self.label, self.times_called);
        self.times_called += 1;
    }
}
// Pixel
#[derive(Copy, Clone)]
pub struct Pixel {
    cords: Position,
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            cords: Position::Logical(LogicalPosition { x: 10.0, y: 10.0 }),
        }
    }
}

impl Label for Pixel {
    fn label(&mut self) -> &'static str {
        return ""
    }
}

impl Run for Pixel {
    fn run(&mut self) {
        //info!("Running component {}", self.label);
    }
}
