#![allow(unused_imports)]
#![allow(dead_code)]

use pixpox::pixpox_renderer::Renderer;
use pixpox::pixpox_utils;
use pixpox::pixpox_app::App;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut app = App::new(WINDOW_TITLE).await;

    app.run().await;
}

