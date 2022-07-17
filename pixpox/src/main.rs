use pixpox;

fn main() {
    pollster::block_on(pixpox::GameState::init());
    pollster::block_on(pixpox::GameState::run());
}
