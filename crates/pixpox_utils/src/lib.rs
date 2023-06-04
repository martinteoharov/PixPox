pub mod stats;
pub use stats::Stats;

pub mod CA;
pub use CA::conway;
pub use CA::cell_realm;
pub use CA::letters;
use winit_input_helper::WinitInputHelper;
use winit::event::{VirtualKeyCode, Event};

/// A macro similar to `vec![$elem; $size]` which returns a boxed array.
///
/// ```rustc
///     let _: Box<[u8; 1024]> = box_array![0; 1024];
/// ```
#[macro_export]
macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        // Use a generic function so that the pointer cast remains type-safe
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();

            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];

            unsafe { Box::from_raw(ptr) }
        }

        vec_to_boxed_array(vec![$val; $len])
    }};
}

pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub struct InputHandler {
    pub winit: WinitInputHelper,
    pub mouse: (isize, isize),
    pub mouse_prev: (isize, isize),
    pub scale: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            winit: WinitInputHelper::new(),
            mouse: (0, 0),
            mouse_prev: (0, 0),
            scale: 1.0
        }
    }

    pub fn update(&mut self, event: &Event<()>, mouse_pos: (isize, isize), prev_mouse_pos: (isize, isize)) {
        self.winit.update(event);
        self.mouse = mouse_pos;
        self.mouse_prev = prev_mouse_pos;
    }

    pub fn update_scale (&mut self, scale: f32) {
        self.scale = scale;
    }

    // Calculate mousepos based on scale
    pub fn get_mouse_pos(&self) -> (isize, isize) {
        (self.mouse.0 / self.scale as isize, self.mouse.1 / self.scale as isize)
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}