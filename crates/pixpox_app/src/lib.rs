use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

use pixpox_renderer::{gui::Gui, Pixels, SurfaceTexture, GlobalPixelMap};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
    window::{Fullscreen, WindowBuilder},
};

use pixpox_common::Camera;

use pixpox_ecs::{component::Texture as RenderTexture, GlobalPixelMap as GlobalPixelMapTrait, World};
use winit_input_helper::WinitInputHelper;

use log::{error, info};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub window_title: String,
    pub window_height: u32,
    pub window_width: u32,
    pub window_scale: f32,
    pub window_fullscreen: bool,
}

pub struct App<'a> {
    pub world: World,
    pixels: Pixels,
    pub gui: Gui<'a>,
    event_loop: EventLoop<()>,
    window: Window,
    input: WinitInputHelper,
    config: Config,
}

impl<'a> App<'a> {
    // Create a new application. Panics if renderer can not be initialized.
    pub fn new(config: Config) -> App<'a> {
        // Initialize WGPU logging
        env_logger::init();

        let world = World::new();

        // Define the event loop
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(config.window_width as f64, config.window_height as f64);
            let scaled_size = LogicalSize::new(
                config.window_width as f32 * config.window_scale,
                config.window_height as f32 * config.window_scale,
            );
            let mut window = WindowBuilder::new()
                .with_title(config.window_title.clone())
                .with_inner_size(scaled_size)
                .with_min_inner_size(size);

            if config.window_fullscreen {
                window = window.with_fullscreen(Some(Fullscreen::Borderless(None)));
            }

            window.build(&event_loop).unwrap()
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);

            match Pixels::new(config.window_width, config.window_height, surface_texture) {
                Ok(v) => v,
                Err(_e) => {
                    println!("Could not initialize renderer");
                    panic!()
                },
            }
        };

        let gui = Gui::new(&window, &pixels);

        Self {
            world,
            pixels,
            gui,
            input,
            event_loop,
            window,
            config,
        }
    }

    pub async fn run(&mut self) {
        self.event_loop.run_return(|event, _target, control_flow| {
            // debug!("Event loop");
            let mut camera: Camera;

            // The one and only event that winit_input_helper doesn't have for us...
            if let Event::RedrawRequested(_) = event {
                // Run components
                self.world.run();

                // Get screen frame to render to
                let pixels = self.pixels.get_frame_mut();

                // Lock storage
                let mut storage = self.world.storage.write().unwrap();

                // Fetch Global Pixelmap
                let pixelmap = storage
                    .query_global_pixel_map::<GlobalPixelMap>()
                    .expect("Could not query Pixel Map");

                camera = pixelmap.get_camera();

                // Render Global Pixelmap to frame
                pixelmap.render(pixels);

                // Prepare Dear ImGui
                self.gui
                    .prepare(&self.window)
                    .expect("gui.prepare() failed");

                let _render_result = self.pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render Dear ImGui
                    self.gui.render(
                        &self.window,
                        encoder,
                        render_target,
                        context,
                        &self.world.stats,
                    )?;

                    Ok(())
                });
            }

            // Handle input events
            self.gui.handle_event(&self.window, &event);
            let mut mouse_cell: (isize, isize) = (0, 0);
            let mut mouse_prev_cell: (isize, isize) = (0, 0);

            // For everything else, for let winit_input_helper collect events to build its state.
            // It returns `true` when it is time to update our game state and request a redraw.
            if self.input.update(&event) {
                // Close events
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                let mut storage = self.world.storage.write().unwrap();

                let pixelmap = storage
                    .query_global_pixel_map::<GlobalPixelMap>()
                    .expect("Could not query Pixel Map");

                camera = pixelmap.get_camera();

                // Handle mouse. This is a bit involved since support some simple
                // line drawing (mostly because it makes nice looking patterns).
                (mouse_cell, mouse_prev_cell) = self
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

                        let sf_width = self.config.window_width as f32 / camera.get_width() as f32;
                        let sf_height =
                            self.config.window_height as f32 / camera.get_height() as f32;

                        let camera_mx_i =
                            (mx_i as f32 / sf_width).floor() as isize + camera.get_x() as isize;
                        let camera_my_i =
                            (my_i as f32 / sf_height).floor() as isize + camera.get_y() as isize;

                        let camera_px_i =
                            (px_i as f32 / sf_width).floor() as isize + camera.get_x() as isize;
                        let camera_py_i =
                            (py_i as f32 / sf_height).floor() as isize + camera.get_y() as isize;

                        ((camera_mx_i, camera_my_i), (camera_px_i, camera_py_i))
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

            self.world.input.update(&event, mouse_cell, mouse_prev_cell);
        });
    }
}
