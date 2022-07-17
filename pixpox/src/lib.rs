use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

mod renderer;

mod constants;
use constants::WINDOW_TITLE;

pub struct GameState {
    pub renderer: renderer::State,
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub exit: bool,
}

impl GameState {
    pub async fn init() -> Self {
        // Initialize WGPU logging
        env_logger::init();

        // Main event loop
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .build(&event_loop)
            .unwrap();

        let mut state = renderer::State::new(&window).await;

        Self {
            renderer: state,
            event_loop,
            window,
            exit: false,
        }
    }

    pub fn run(self) {
        let Self {
            mut event_loop,
            renderer,
            window,
            ..
        } = self;

        event_loop.run_return(|event, _target, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !renderer.input(event) {
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

                        // WindowEvent::KeyboardInput {
                        //     input:
                        //         KeyboardInput {
                        //             state: ElementState::Pressed,
                        //             virtual_keycode: Some(VirtualKeyCode::Space),
                        //             ..
                        //         },
                        //     ..
                        // } => {
                        //     show = !show;
                        // },
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        },

                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(**new_inner_size);
                        },

                        _ => {},
                    }
                }
            },
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                renderer.update();
                match renderer.render(None, None) {
                    Ok(_) => {},
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(self.renderer.size),
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
        });
    }
}
