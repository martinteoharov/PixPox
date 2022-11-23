#![allow(unused_imports)]
#![allow(dead_code)]

pub mod components;

extern crate dotenv;

use std::time::Instant;

use components::{Pixel, Player};
use dotenv::dotenv;

use log::{error, info, debug};
use pixpox::pixpox_app::App;
use pixpox::pixpox_renderer::Renderer;
use pixpox::pixpox_utils;
use pixpox_app::AppConfig;
use pixpox_ecs::entity::Entity;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

async fn run() {
    // TODO: read config from file
    let config = AppConfig {
        WINDOW_TITLE: "pixpox!",
        WINDOW_WIDTH: 1024,
        WINDOW_HEIGHT: 760,
        WINDOW_FULLSCREEN: false,
        DEBUG: true,
    };

    let mut app = App::new(config).await;

    let pixel_component = Pixel::new();

    let now = Instant::now();

    for _ in 1..1_000_000 {
        let entity = app.world.new_entity();
        app.world.add_component_to_entity(entity, pixel_component);
    }

    debug!(
        "Main::run() create 1000000 entities in {} micros",
        now.elapsed().as_micros().to_string()
    );

    app.run().await;
}
