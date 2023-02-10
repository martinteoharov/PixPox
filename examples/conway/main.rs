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

use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::AppConfig;
use pixpox_ecs::entity::Entity;
use pixpox_ecs::Run;
use pixpox_ecs::{world, Texture};
use rand::Rng;
use winit::dpi::{LogicalPosition, Position};

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

async fn run() {
    // TODO: read config from file

    let config = AppConfig {
        WINDOW_TITLE: "Conway",
        WINDOW_WIDTH: 400,
        WINDOW_HEIGHT: 300,
        WINDOW_FULLSCREEN: false,
        DEBUG: true,
    };

    let mut app = App::new(config).await;

    let now = Instant::now();
    let mut entities_count = 0;
    let mut rng = rand::thread_rng();

    let global_pixel_map =
        GlobalPixelMap::new_empty(config.WINDOW_WIDTH, config.WINDOW_HEIGHT, [0, 0, 0, 255]);

    for y in 0..config.WINDOW_HEIGHT {
        for x in 0..config.WINDOW_WIDTH {
            let entity = app.world.spawn();

            let pos = LogicalPosition::new(x, y);
            let alive = rng.gen_bool(0.1);

            let cell_component = Cell::new(entity.id, pos, alive);

            app.world.add_component_to_entity(entity, cell_component);

            entities_count += 1;
        }
    }

    app.world
        .storage
        .new_bucket::<GlobalPixelMap>("pixelmap", global_pixel_map);


    info!(
        "Main::run() create {} entities in {} seconds",
        entities_count,
        now.elapsed().as_secs_f32().to_string()
    );

    app.run::<GlobalPixelMap>().await;
}

pub struct GlobalPixelMap {
    pixelmap: HashMap<LogicalPosition<u32>, [u8; 4]>,
    width: u32,
    height: u32,
    clear_color: [u8; 4],
}

impl GlobalPixelMap {
    pub fn new_empty(width: u32, height: u32, clear_color: [u8; 4]) -> Self {
        let mut pixelmap = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                pixelmap.insert(LogicalPosition::new(x, y), [0, 0, 0, 255]);
            }
        }

        Self {
            pixelmap,
            width,
            height,
            clear_color,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixelmap.get_mut(&LogicalPosition::new(x, y)).unwrap();
                let color: [u8; 4] = [0, 0, 0, 255];
                pixel.copy_from_slice(&color);
            }
        }
    }

    pub fn set_pos(&mut self, pos: LogicalPosition<u32>, color: [u8; 4]) {
        debug!("Setting pos");
        self.pixelmap.entry(pos).or_insert(color);
    }

    pub fn run(&self) {}
}

impl Texture for GlobalPixelMap {
    fn render(&mut self, pixels: &mut [u8]) {
        debug!("Rendering GlobalPixelMap");
        let pixelmap = self.pixelmap.values().cloned().collect::<Vec<[u8; 4]>>();
        for (c, pix) in pixelmap.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(c);
        }
    }
}
