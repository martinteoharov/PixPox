#![allow(unused_imports)]
#![allow(dead_code)]

pub mod components;

extern crate dotenv;

use components::Player;
use dotenv::dotenv;

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
    // TODO: config read from file
    let config = AppConfig {
        WINDOW_TITLE: "pixpox!",
        WINDOW_WIDTH: 1024,
        WINDOW_HEIGHT: 760,
        WINDOW_FULLSCREEN: false,
    };

    let mut app = App::new(config).await;

    let entity = app.world.entities.create();
    let component = Player::new();
    app.world.add_component_to_entity(entity, component);

    app.run().await;
}
