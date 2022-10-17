use log::info;
use pixpox_ecs::{Label, Run};
pub struct Player {
    label: &'static str;
}

impl Player {
    pub fn new() {}
}

impl Label for Player {
    fn label() -> &'static str {
        {
            "Player"
        }
    }
}

impl Run for Player {
    fn run() {
        info!("Kur kapan");
    }
}
