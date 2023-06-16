#![allow(unused_imports)]
#![allow(dead_code)]

pub mod custom_components;
use pixpox_renderer::global_pixel_map::GlobalPixelMap;
use pixpox_common::{Camera, Direction};

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
use pixpox_ecs::{world, Texture};
use pixpox_ecs::{Run, Update};
use pixpox_renderer::gui::{GuiChild, GuiParent};
use pixpox_utils::{conway::ConwayGrid, Stats};
use rand::Rng;
use winit::dpi::{LogicalPosition, Position};
use winit_input_helper::WinitInputHelper;


use crate::custom_components::ConwayGridComponent;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

fn show_metrics(ui: &mut Ui, _state: &mut bool) {
    ui.show_metrics_window(&mut true);
}

async fn run() {
    let cfg: Config =
        confy::load_path("./examples/conway/AppConfig.toml").expect("Could not load config.");

    dbg!(cfg.clone());

    let mut app = App::new(cfg.clone());

    // Create a camera
    let camera = Camera::new(
        0,
        0,
        cfg.window_height,
        cfg.window_width,
        cfg.window_height,
        cfg.window_width,
    );

    // Define global data structure
    let global_pixel_map = GlobalPixelMap::new_empty(cfg.window_height, cfg.window_width, camera);

    // Initialise world; fill global data structures
    let entity = app.world.spawn();

    let grid_component = ConwayGridComponent::new(cfg.window_height, cfg.window_width, 0.10);

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

    // write storage
    {
        let mut storage = app.world.storage.write().unwrap();

        storage.new_bucket::<GlobalPixelMap>("pixelmap", global_pixel_map);
    }

    app.run().await;
}
