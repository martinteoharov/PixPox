#![allow(unused_imports)]
#![allow(dead_code)]

pub mod custom_components;

extern crate dotenv;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use std::{collections::HashMap, time::Instant};

use custom_components::Cell;
use dotenv::dotenv;

use imgui::Ui;
use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::Config;
use pixpox_ecs::entity::Entity;
use pixpox_ecs::{Run, InputHandler};
use pixpox_ecs::{world, Texture};
use pixpox_renderer::gui::{GuiChild, GuiParent};
use pixpox_common::Camera;
use pixpox_utils::{Stats, conway::ConwayGrid, CA::letters};
use rand::Rng;
use winit::dpi::{LogicalPosition, Position};

use pixpox_renderer::global_pixel_map::GlobalPixelMap;

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
        confy::load_path("./examples/ecs/AppConfig.toml").expect("Could not load config.");

    dbg!(cfg.clone());
    let mut app = App::new(cfg.clone());

    let now = Instant::now();
    let mut entities_count = 0;
    let mut rng = rand::thread_rng();

    // Create a camera
    let camera = Camera::new(
        0,
        0,
        cfg.window_height,
        cfg.window_width,
        cfg.window_height,
        cfg.window_width,
    );

    // Define global data structures
    let global_pixel_map =
        GlobalPixelMap::new_empty(cfg.window_height, cfg.window_width, camera);

    let mut optim_grid: ConwayGrid = ConwayGrid::new(cfg.window_width, cfg.window_height, 0.10);

    // Initialise world; fill global data structures
    for y in 0..cfg.window_height {
        for x in 0..cfg.window_width {
            let entity = app.world.spawn();

            let pos = (x as isize, y as isize);
            let alive = rng.gen_bool(0.10);

            let cell_component = Cell::new(entity.id, pos, alive);

            app.world.add_component_to_entity(entity, cell_component);

            optim_grid.set_cell(pos, alive);

            entities_count += 1;
        }
    }

    // Define UI Callbacks and States
    let mut show_metrics_state = &mut false;
    let mut show_metrics_closure = |ui: &mut Ui, state: &mut bool, stats: &Stats| {
        ui.show_metrics_window(state);
        ui.window("ECS Performance (World)")
            .position([60.0, 390.0], imgui::Condition::Once)
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .collapsible(true)
            .build(|| {
                ui.text("entities: ".to_owned() + &entities_count.to_string());

                for s in stats.get_formatted_stats().iter() {
                    ui.text(s);
                };
            });
    };

    let mut show_about_state = &mut true;
    let mut show_about_closure = |ui: &mut Ui, state: &mut bool, _stats: &Stats| {
        ui.show_about_window(state);
    };

    // Setup GUI
    app.gui.register_parent("Help");
    app.gui
        .register_parent("Debug");

    let mut performance_metrics = GuiChild::new(
        "Performance Metrics",
        &mut show_metrics_closure,
        show_metrics_state,
    );
    let mut about = GuiChild::new("About", &mut show_about_closure, show_about_state);

    app.gui.register_child("Help", &mut about);
    app.gui
        .register_child("Debug", &mut performance_metrics);

    // write storage
    {
        let mut storage = app.world.storage.write().unwrap();

        storage.new_bucket::<GlobalPixelMap>("pixelmap", global_pixel_map);

        storage.new_bucket::<ConwayGrid>("optim_grid", optim_grid);

        let (width, height) = (cfg.window_width, cfg.window_height);
        storage.new_bucket::<(u32, u32)>("grid-size", (width, height));
    }

    info!(
        "Main::run() create {} entities in {} seconds",
        entities_count,
        now.elapsed().as_secs_f32().to_string()
    );

    app.run::<GlobalPixelMap>().await;
}