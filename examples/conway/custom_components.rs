use std::{collections::HashMap, sync::RwLock};

use log::{debug, error};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    InputHandler, Label, Run, Storage, Texture, Update, World,
};
use pixpox_utils::{conway::ConwayGrid, Stats};
use winit::{
    dpi::{LogicalPosition, Position},
    event::VirtualKeyCode,
};
use winit_input_helper::WinitInputHelper;

use pixpox_renderer::global_pixel_map::GlobalPixelMap;

#[derive(Clone)]
pub struct ConwayGridComponent {
    inner: ConwayGrid,
    paused: bool,
}

impl ConwayGridComponent {
    pub fn new(height: u32, width: u32, gen_chance: f64) -> Self {
        Self {
            inner: ConwayGrid::new(height, width, gen_chance),
            paused: true,
        }
    }
}

impl Run for ConwayGridComponent {
    fn run(&mut self, _storage: &pixpox_ecs::Storage) {
        if self.paused {
            return;
        }

        self.inner.next_state();
    }
}

impl Update for ConwayGridComponent {
    fn update(&mut self, storage: &RwLock<pixpox_ecs::Storage>, input: &InputHandler, stats: &RwLock<Stats>) {
        let mut storage = storage.write().unwrap();

        if input.winit.key_pressed(VirtualKeyCode::P) {
            log::info!("Toggled world");
            self.paused = !self.paused;
        }

        if input.winit.key_pressed(VirtualKeyCode::C) {
            log::info!("Clear grid");
            self.inner.clear_grid();
        }

        if input.winit.mouse_held(0) {
            log::info!("mouse pos: [{}, {}]", input.mouse.0, input.mouse.1);
            self.inner.set_line(input.mouse, input.mouse_prev, true);
        }

        // Fetch PixelMap
        let pixelmap = storage
            .query_storage_mut::<GlobalPixelMap>("pixelmap")
            .expect("Could not query Pixel Map");

        pixelmap.draw_flat_vec(&mut self.inner.get_color_vec());
    }
}

impl Label for ConwayGridComponent {
    fn label(&mut self) -> &'static str {
        "ConwayGrid"
    }
}
