use log::{debug, info};
use winit::dpi::LogicalPosition;

use crate::Storage;

/*
 * Base traits that all components must implement
 */
pub trait Label {
    fn label(&mut self) -> &'static str;
}

pub trait Run {
    fn run(&mut self, storage: &mut Storage);
}

pub struct BaseComponent {
    label: &'static str,
}

impl Label for BaseComponent {
    fn label(&mut self) -> &'static str {
        {
            self.label
        }
    }
}

impl Run for BaseComponent {
    fn run(&mut self, storage: &mut Storage) {
        info!("Kur kapan");
    }
}

/*
 * Base trait that all Texture components need to implement
*/

pub trait Texture {
    fn render(&mut self, pixels: &mut [u8]);
}