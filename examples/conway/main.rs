#![allow(unused_imports)]
#![allow(dead_code)]

pub mod custom_components;

extern crate dotenv;

use std::{collections::HashMap, time::Instant};

use custom_components::Cell;
use dotenv::dotenv;

use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::AppConfig;
use pixpox_ecs::components::TexturePixel;
use pixpox_ecs::entity::Entity;
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

    let mut grid = HashMap::<LogicalPosition<u32>, Entity>::new();

    for y in 0..config.WINDOW_HEIGHT {
        for x in 0..config.WINDOW_WIDTH {
            let entity = app.world.spawn();

            let pos = LogicalPosition::new(x, y);
            let alive = rng.gen_bool(0.5);

            let cell_component = Cell::new(pos, alive);

            let color: [u8; 4] = if alive {
                [0, 255, 255, 255]
            } else {
                [0, 0, cell_component.heat, 255]
            };

            let texture_pixel_component = TexturePixel::new(pos, color);

            app.world.add_component_to_entity(entity, cell_component);
            app.world
                .add_component_to_entity(entity, texture_pixel_component);

            entities_count += 1;

            grid.insert(pos, entity);
        }
    }

    app.world
        .new_hashmap_bucket::<LogicalPosition<u32>, Entity>("grid", Some(grid));

    info!(
        "Main::run() create {} entities in {} seconds",
        entities_count,
        now.elapsed().as_secs_f32().to_string()
    );

    app.run().await;
}