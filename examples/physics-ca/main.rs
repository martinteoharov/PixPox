#![allow(unused_imports)]
#![allow(dead_code)]

pub mod custom_components;

extern crate dotenv;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use std::{collections::HashMap, time::Instant};

use dotenv::dotenv;

use imgui::Ui;
use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::Config;
use pixpox_ecs::entity::Entity;
use pixpox_ecs::{world, Texture, World, InputHandler};
use pixpox_ecs::{Run, Update};
use pixpox_renderer::gui::{GuiChild, GuiParent};
use pixpox_utils::CA::cell_realm::CellRealm;
use pixpox_utils::{conway::ConwayGrid, Stats};
use rand::Rng;
use winit::dpi::{LogicalPosition, Position};
use winit::event::{DeviceEvent, Event, MouseButton, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use crate::custom_components::CellRealmComponent;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

fn show_metrics(ui: &mut Ui, state: &mut bool) {
    ui.show_metrics_window(&mut true);
}

async fn run() {
    let cfg: Config =
        confy::load_path("./examples/physics-ca/AppConfig.toml").expect("Could not load config.");

    dbg!(cfg.clone());

    let mut app = App::new(cfg.clone());

    // Define global data structures
    let global_pixel_map =
        GlobalPixelMap::new_empty(cfg.window_width + 2, cfg.window_height + 2, [0, 0, 0, 0]);

    // Initialise world; fill global data structures
    let entity = app.world.spawn();

    let grid_component = CellRealmComponent::new(cfg.window_height, cfg.window_width);

    app.world.add_component_to_entity(entity, grid_component);

    // Define UI Callbacks and States
    let show_metrics_state = &mut true;
    let mut show_metrics_closure = |ui: &mut Ui, state: &mut bool, stats: &Stats| {
        // ui.show_metrics_window(state);
        ui.window("Conway Performance (World)")
            .position([60.0, 60.0], imgui::Condition::Once)
            .size([200.0, 200.0], imgui::Condition::FirstUseEver)
            .collapsible(true)
            .build(|| {
                for s in stats.get_formatted_stats().iter() {
                    ui.text(s);
                }
            });
    };

    let show_about_state = &mut false;
    let mut show_about_closure = |ui: &mut Ui, state: &mut bool, _stats: &Stats| {
        ui.show_about_window(state);
    };

    // Setup GUI
    app.gui.register_parent("Help");
    app.gui.register_parent("Debug");

    let mut performance_metrics = GuiChild::new(
        "Performance Metrics",
        &mut show_metrics_closure,
        show_metrics_state,
    );
    let mut about = GuiChild::new("About", &mut show_about_closure, show_about_state);

    app.gui.register_child("Help", &mut about);
    app.gui.register_child("Debug", &mut performance_metrics);

    // Get write lock for storage
    {
        let mut storage = app.world.storage.write().unwrap();

        storage.new_bucket::<GlobalPixelMap>("pixelmap", global_pixel_map);

        storage.new_bucket::<usize>("selected-tool", 0);
    }

    app.run::<GlobalPixelMap>().await;
}

#[derive(Debug)]
pub struct GlobalPixelMap {
    pixelmap: Vec<[u8; 4]>,
    width: u32,
    height: u32,
    clear_color: [u8; 4],
}

impl GlobalPixelMap {
    pub fn new_empty(width: u32, height: u32, clear_color: [u8; 4]) -> Self {
        let mut pixelmap: Vec<[u8; 4]> = Vec::new();

        for _y in 0..height {
            for _x in 0..width {
                let c: [u8; 4] = [0, 0, 0, 0];
                pixelmap.push(c);
            }
        }

        Self {
            pixelmap,
            width,
            height,
            clear_color,
        }
    }

    pub fn draw_pos(&mut self, pos: (u32, u32), color: [u8; 4]) {
        let idx = pos.1 * self.width + pos.0;
        self.pixelmap[idx as usize] = color;
    }

    pub fn draw_flat_vec(&mut self, vec: &mut Vec<[u8; 4]>) {
        std::mem::swap(&mut self.pixelmap, vec);
    }

    pub fn run(&self) {}
}

impl Texture for GlobalPixelMap {
    fn render(&self, pixels: &mut [u8]) {
        debug!("Rendering GlobalPixelMap");
        for (c, pix) in self.pixelmap.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(c);
        }
    }

    fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    fn update(&mut self, input: &InputHandler) {
        debug!("Updating GlobalPixelMap");
    }
}
