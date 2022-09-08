use winit::{event_loop::{EventLoop, ControlFlow}, window::Window, window::WindowBuilder, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}, platform::run_return::EventLoopExtRunReturn};

use pixpox_ecs::World;
use pixpox_renderer::Renderer;

pub struct App {
    pub world: World,
    pub renderer: Renderer,
    pub event_loop: EventLoop<()>,
    pub window: Window,
    quit: bool
}

impl App {
    pub async fn new(title: &str) -> App {
        // Initialize WGPU logging
        env_logger::init();

        let world = World::new();

        // Define the event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        let renderer = Renderer::new(&window).await;

        Self {
            world,
            renderer,
            event_loop,
            window,
            quit: false
        }
    }

    pub async fn run(&mut self) {
        self.event_loop.run_return(|event, _target, control_flow| {
            if self.quit {
                *control_flow = ControlFlow::Exit;
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.renderer.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    }
                                    | KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Q),
                                        ..
                                    },
                                ..
                            } => {
                                *control_flow = ControlFlow::Exit;
                            },

                            WindowEvent::Resized(physical_size) => {
                                self.renderer.resize(*physical_size);
                            },

                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.renderer.resize(**new_inner_size);
                            },

                            _ => {},
                        }
                    }
                },

                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    self.renderer.update();
                    match self.renderer.render() {
                        Ok(_) => {},
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => {
                            self.renderer.resize(self.renderer.size.clone())
                        },
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                },

                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.window.request_redraw();
                },
                _ => {},
            }
        });
    }
}
