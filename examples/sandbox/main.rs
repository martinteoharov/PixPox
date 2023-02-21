#![allow(unused_imports)]
#![allow(dead_code)]

pub mod custom_components;

extern crate dotenv;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::{collections::HashMap, time::Instant};

use custom_components::Cell;
use dotenv::dotenv;

use imgui::Ui;
use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::{Config};
use pixpox_ecs::entity::Entity;
use pixpox_ecs::Run;
use pixpox_ecs::{world, Texture};
use pixpox_renderer::gui::{GuiChild, GuiParent};
use rand::Rng;
use winit::dpi::{LogicalPosition, Position};

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

fn show_metrics(ui: &mut Ui, state: &mut bool) {
    ui.show_metrics_window(&mut true);
}

async fn run() {
    let cfg: Config = confy::load_path("./examples/sandbox/AppConfig.toml").expect("Could not load config.");

    dbg!(cfg.clone());
    let mut app = App::new(cfg.clone());


    let now = Instant::now();
    let mut entities_count = 0;
    let mut rng = rand::thread_rng();

    // Define UI Callbacks and States
    let mut show_metrics_state = &mut false;
    let mut show_metrics_closure = |ui: &mut Ui, state: &mut bool| {
        ui.show_metrics_window(state);
    };

    let mut show_about_state = &mut true;
    let mut show_about_closure = |ui: &mut Ui, state: &mut bool| {
        ui.show_about_window(state);
    };
    

    // ui.show_about_window(&mut self.about_open);

    // Setup GUI
    app.gui.register_parent("Help");

    let mut performance_metrics = GuiChild::new("Performance Metrics", &mut show_metrics_closure, show_metrics_state);
    let mut about = GuiChild::new("About", &mut show_about_closure, show_about_state);

    app.gui.register_child("Help", &mut about);
    app.gui.register_child("Help", &mut performance_metrics);

    // Define global data structures
    let global_pixel_map =
        GlobalPixelMap::new_empty(cfg.window_width, cfg.window_height, [0, 0, 0, 0]);

    let mut grid: HashMap<LogicalPosition<u32>, bool> = HashMap::new();

    // Initialise world; fill global data structures
    for y in 0..cfg.window_height {
        for x in 0..cfg.window_width {
            let entity = app.world.spawn();

            let pos = LogicalPosition::new(x, y);
            let alive = rng.gen_bool(0.10);

            let cell_component = Cell::new(entity.id, pos, alive);

            app.world.add_component_to_entity(entity, cell_component);

            grid.insert(pos, alive);

            entities_count += 1;
        }
    }

    // Lock storage
    {
        let mut storage = app.world.storage.write().unwrap();

        storage.new_bucket::<GlobalPixelMap>("pixelmap", global_pixel_map);

        storage.new_bucket::<HashMap<LogicalPosition<u32>, bool>>("grid", grid);

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

    pub fn draw_pos(&mut self, pos: LogicalPosition<u32>, color: [u8; 4]) {
        let idx = pos.y * self.width + pos.x;
        self.pixelmap[idx as usize] = color;
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
}
