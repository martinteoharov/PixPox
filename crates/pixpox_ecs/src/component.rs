use std::sync::RwLock;

use crate::Storage;

/*
 * Base Traits
 *
 * All components should implement them.
 */
pub trait Label {
    fn label(&mut self) -> &'static str;
}

pub trait Run {
    fn run(&mut self, storage: &Storage);
}

pub trait Update {
    fn update(&mut self, storage: &RwLock<Storage>);
}

/*
 * Optional Traits
 *
 * These traits can be optionally implemented.
 */
pub trait Texture {
    fn render(&mut self, pixels: &mut [u8]);
}