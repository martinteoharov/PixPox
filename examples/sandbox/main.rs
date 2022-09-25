#![allow(unused_imports)]
#![allow(dead_code)]

use pixpox::pixpox_app::App;
use pixpox::pixpox_renderer::Renderer;
use pixpox::pixpox_utils;
use pixpox_app::AppConfig;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let config = AppConfig {
        WINDOW_TITLE: "pixpox!",
        WINDOW_WIDTH: 1024,
        WINDOW_HEIGHT: 760,
        WINDOW_FULLSCREEN: false,
    };

    let mut app = App::new(config).await;

    app.run().await;
}
