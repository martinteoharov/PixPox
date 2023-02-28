use std::{collections::HashMap, sync::RwLock};

use log::{debug, error, info};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World,
};
use pixpox_utils::{conway::ConwayGrid, CA::cell_realm::CellRealm};
use winit::{dpi::{LogicalPosition, Position}, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use crate::GlobalPixelMap;

#[derive(Clone)]
pub struct CellRealmComponent {
    inner: CellRealm,
    paused: bool
}

impl CellRealmComponent {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            inner: CellRealm::new(height, width),
            paused: false
        }
    }
}

impl Run for CellRealmComponent {
    fn run(&mut self, _storage: &pixpox_ecs::Storage) {
        self.inner.next_state();
    }
}

impl Update for CellRealmComponent {
    fn update(&mut self, storage: &RwLock<pixpox_ecs::Storage>, input: &WinitInputHelper) {
        let mut storage = storage.write().unwrap();

        if input.key_pressed(VirtualKeyCode::P) {
            info!("Toggled world");
            self.paused = !self.paused;
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
