use std::{collections::HashMap, sync::RwLock};

use log::{debug, error};
use pixpox_app::App;
use pixpox_ecs::{
    entity::{self, Entity},
    Label, Run, Storage, Texture, Update, World,
};
use pixpox_utils::conway::ConwayGrid;
use winit::dpi::{LogicalPosition, Position};

use crate::GlobalPixelMap;

#[derive(Clone)]
pub struct ConwayGridComponent {
    inner: ConwayGrid,
}

impl ConwayGridComponent {
    pub fn new(height: u32, width: u32, gen_chance: f64) -> Self {
        Self {
            inner: ConwayGrid::new(height, width, gen_chance),
        }
    }
}

impl Run for ConwayGridComponent {
    fn run(&mut self, _storage: &pixpox_ecs::Storage) {
        self.inner.next_state();
    }
}

impl Update for ConwayGridComponent {
    fn update(&mut self, storage: &RwLock<pixpox_ecs::Storage>) {
        let mut storage = storage.write().unwrap();

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