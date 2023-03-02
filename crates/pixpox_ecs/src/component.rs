use std::sync::RwLock;

use winit_input_helper::WinitInputHelper;

use crate::{Storage, InputHandler};

/// The Label trait is used to give a human-readable label to an ECS component.
/// Every component must implement this trait in order to be used with the PixPox game engine.
/// 
/// ### Example
///
/// ```
/// struct Cell {
///     label: &'static str,
/// }
///
/// impl Label for Cell {
///    fn label(&mut self) -> &'static str {
///       return self.label;
///     }
/// }
/// ```
pub trait Label {
    fn label(&mut self) -> &'static str;
}


/// The `Run` trait specifies a `run()` method that is executed for each component 
/// whenever `world.run()` is called. This function is parallelized using multi-threading 
/// and only has read access to the storage. It is designed for heavy computation, and no 
/// updates are allowed.
/// 
/// ### Example
///
/// ```
/// struct Cell {
///     label: &'static str,
/// }
///
/// impl Run for Cell {
///     fn run(&mut self, storage: &Storage) {
///         let grid = storage
///             .query_storage::<HashMap<LogicalPosition<u32>, bool>>("grid")
///             .expect("Could not query storage: grid");
///     }
/// }
///
/// ```
pub trait Run {
    fn run(&mut self, storage: &Storage);
}

/// The `Update` trait specifies an `update()` method, which much like `run()`  is 
/// executed on every pass of `world.run()`. It is also parallelized, but its given a 
/// `RwLock` instead of an immutable reference. This method is meant to update the 
/// world when a change is present, and it should not attempt to obtain a `.write()` lock 
/// on the storage when no changes are present, as doing so would adversely impact performance.
/// 
/// ### Example
///
/// ```
/// struct Cell {
///     label: &'static str,
/// }
///
/// impl Update for Cell {
///     fn update(&mut self, storage: &RwLock<Storage>) {
///         let mut storage = storage.write().unwrap();
///     }
/// }
/// ```
pub trait Update {
    fn update(&mut self, storage: &RwLock<Storage>, input: &InputHandler);
}

/// The Texture trait defines how a component should be rendered.
/// The `render()` method is responsible for rendering the texture, given a `pixels` buffer as an argument, 
/// and the `size()` method returns the size of the texture.
///
/// ### Example
/// ```
/// impl Texture for GlobalPixelMap {
///     fn render(&self, pixels: &mut [u8]) {
///         debug!("Rendering GlobalPixelMap");
///         for (c, pix) in self.pixelmap
///             .iter()
///             .zip(pixels.chunks_exact_mut(4)) {
///                 pix.copy_from_slice(c);
///         }
///     }
///
///     fn size(&self) -> (u32, u32) {
///         return (self.width, self.height);
///     }
/// }
/// ```
pub trait Texture {
    fn render(&self, pixels: &mut [u8]);
    fn size(&self) -> (u32, u32);
}
