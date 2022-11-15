#![allow(unused_imports)]
#![allow(dead_code)]

pub mod components;

extern crate dotenv;

use components::{Player, Pixel};
use dotenv::dotenv;

use log::{error, info};
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

    let component = Pixel::new();
    for _ in 1..100 {
        let entity = app.world.entities.create();
        app.world.add_component_to_entity(entity, component);
    }

    app.run().await;
}
