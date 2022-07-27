use pixpox;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut game = pixpox::GameState::init().await;

    game.run().await;
}