use std::{collections::HashMap, sync::RwLock};

use log::{debug, error, info};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World, InputHandler,
};
use pixpox_utils::{
    conway::ConwayGrid,
    CA::cell_realm::{CellRealm, Cell}, stats, Stats,
};
use winit::{
    dpi::{LogicalPosition, Position},
    event::VirtualKeyCode,
};
use winit_input_helper::WinitInputHelper;

use crate::GlobalPixelMap;

#[derive(Clone)]
pub struct CellRealmComponent {
    inner: CellRealm,
    paused: bool,
}

impl CellRealmComponent {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            inner: CellRealm::new(height, width),
            paused: false,
        }
    }
}

impl Run for CellRealmComponent {
    fn run(&mut self, _storage: &pixpox_ecs::Storage) {
        if !self.paused {
            self.inner.next_state();
        }
    }
}

impl Update for CellRealmComponent {
    fn update(&mut self, storage: &RwLock<pixpox_ecs::Storage>, input: &InputHandler, stats: &RwLock<Stats>) {
        let mut storage = storage.write().unwrap();

        if input.winit.key_pressed(VirtualKeyCode::P) {
            info!("Toggled world");
            self.paused = !self.paused;
        }

        // Left mouse click
        if input.winit.mouse_held(0) {
            info!("mouse pos: [{}, {}]", input.mouse.0, input.mouse.1);
            self.inner.set_circle(input.mouse, 30, Cell::SAND);
        }

        // Right mouse click
        if input.winit.mouse_held(1) {
            info!("mouse pos: [{}, {}]", input.mouse.0, input.mouse.1);
            self.inner.set_circle(input.mouse, 10, Cell::WATER);
        }

        // Middle mouse click
        if input.winit.mouse_held(2) {
            info!("mouse pos: [{}, {}]", input.mouse.0, input.mouse_prev.1);

            self.inner.set_line(input.mouse, input.mouse_prev, Cell::SOLID);
        }

        // clear grid
        if input.winit.key_pressed(VirtualKeyCode::C) {
            log::info!("Clear grid");
            self.inner.clear_grid();
        }

        // stats.write().expect("couldnt lock").update_sector("label".to_string(), 123.0);

        let cell_count = self.inner.get_cell_count();

        // iterate over cell_coutn and update stats
        for (label, count) in cell_count {
            stats.write().expect("couldnt lock").update_sector(label.to_string(), count as f32);
        }

        // Fetch PixelMap
        let pixelmap = storage
            .query_storage_mut::<GlobalPixelMap>("pixelmap")
            .expect("Could not query Pixel Map");

        pixelmap.draw_flat_vec(&mut self.inner.get_color_vec());
    }
}

impl Label for CellRealmComponent {
    fn label(&mut self) -> &'static str {
        "CellRealm"
    }
}
