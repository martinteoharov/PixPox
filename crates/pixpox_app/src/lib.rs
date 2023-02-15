use std::{fmt::Debug, time::Instant};

use pixpox_renderer::{wgpu::Texture, Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
    window::WindowBuilder,
};

use pixpox_ecs::{component::Texture as RenderTexture, World};
use winit_input_helper::WinitInputHelper;

use log::{debug, error, info, warn};

#[derive(Copy, Clone)]
pub struct AppConfig {
    pub WINDOW_TITLE: &'static str,
    pub WINDOW_WIDTH: u32,
    pub WINDOW_HEIGHT: u32,
    pub WINDOW_FULLSCREEN: bool,
    pub DEBUG: bool,
}

pub struct App {
    pub world: World,
    pixels: Pixels,
    event_loop: EventLoop<()>,
    window: Window,
    input: WinitInputHelper,
    paused: bool,
    quit: bool,
    last_update: Instant,
}

impl App {
    // Create a new application. Panics if renderer can not be initialized.
    pub async fn new(config: AppConfig) -> App {
        // Initialize WGPU logging
        env_logger::init();

        let world = World::new();

        // Define the event loop
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(config.WINDOW_WIDTH as f64, config.WINDOW_HEIGHT as f64);
            let scaled_size = LogicalSize::new(
                config.WINDOW_WIDTH as f64 * 3.0,
                config.WINDOW_HEIGHT as f64 * 3.0,
            );
            WindowBuilder::new()
                .with_title(config.WINDOW_TITLE)
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);

            match Pixels::new(config.WINDOW_WIDTH, config.WINDOW_HEIGHT, surface_texture) {
                Ok(v) => v,
                Err(e) => {
                    println!("Could not initialize renderer");
                    panic!()
                },
            }
        };

        Self {
            world,
            pixels,
            input,
            event_loop,
            window,
            paused: false,
            quit: false,
            last_update: Instant::now(),
        }
    }

    pub async fn run<T: 'static + RenderTexture>(&mut self) {
        self.event_loop.run_return(|event, _target, control_flow| {
            debug!("Event loop");

            // The one and only event that winit_input_helper doesn't have for us...
            if let Event::RedrawRequested(_) = event {
                let elapsed = self.last_update.elapsed().as_secs_f32();
                self.last_update = Instant::now();

                info!("FPS: {}", 1.0 / elapsed);

                // Run components
                self.world.run();

                // Get screen frame to render to
                let pixels = self.pixels.get_frame_mut();

                // Fetch Global Pixelmap
                let pixelmap = self
                    .world
                    .storage
                    .query_global_pixel_map::<T>("pixelmap")
                    .expect("Could not query Pixel Map");

                // Render Global Pixelmap to frame
                pixelmap.render(pixels);

                if let Err(err) = self.pixels.render() {
                    error!("pixels.render() failed: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // For everything else, for let winit_input_helper collect events to build its state.
            // It returns `true` when it is time to update our game state and request a redraw.
            if self.input.update(&event) {
                // Close events
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                if self.input.key_pressed(VirtualKeyCode::P) {
                    self.paused = !self.paused;
                }
                if self.input.key_pressed_os(VirtualKeyCode::Space) {
                    // Space is frame-step, so ensure we're paused
                    self.paused = true;
                }
                // Handle mouse. This is a bit involved since support some simple
                // line drawing (mostly because it makes nice looking patterns).
                let (mouse_cell, mouse_prev_cell) = self
                    .input
                    .mouse()
                    .map(|(mx, my)| {
                        let (dx, dy) = self.input.mouse_diff();
                        let prev_x = mx - dx;
                        let prev_y = my - dy;

                        let (mx_i, my_i) = self
                            .pixels
                            .window_pos_to_pixel((mx, my))
                            .unwrap_or_else(|pos| self.pixels.clamp_pixel_pos(pos));

                        let (px_i, py_i) = self
                            .pixels
                            .window_pos_to_pixel((prev_x, prev_y))
                            .unwrap_or_else(|pos| self.pixels.clamp_pixel_pos(pos));

                        (
                            (mx_i as isize, my_i as isize),
                            (px_i as isize, py_i as isize),
                        )
                    })
                    .unwrap_or_default();

                // Resize the window
                if let Some(size) = self.input.window_resized() {
                    info!("Resize detected");
                    if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                        error!("pixels.resize_surface() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                self.window.request_redraw();
            }
        });
    }
}
