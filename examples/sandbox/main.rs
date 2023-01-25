#![allow(unused_imports)]
#![allow(dead_code)]

pub mod components;

extern crate dotenv;

use std::time::Instant;

use components::{Pixel, Player};
use dotenv::dotenv;

use log::{debug, error, info};
use pixpox::pixpox_app::App;
use pixpox::pixpox_utils;
use pixpox_app::AppConfig;
use pixpox_ecs::entity::Entity;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;

const WINDOW_TITLE: &str = "pixpox!";

fn main() {
    dotenv().ok();
    pollster::block_on(run());
}

async fn run() {
    // TODO: read config from file
    let config = AppConfig {
        WINDOW_TITLE: "pixpox!",
        WINDOW_WIDTH: 1024,
        WINDOW_HEIGHT: 760,
        WINDOW_FULLSCREEN: false,
        DEBUG: true,
    };

    let mut app = App::new(config).await;

    let pixel_component = Pixel::new();

    let now = Instant::now();

    for _ in 1..1_000 {
        let entity = app.world.new_entity();
        app.world.add_component_to_entity(entity, pixel_component);
    }

    error!(
        "Main::run() create 1000000 entities in {} micros",
        now.elapsed().as_micros().to_string()
    );

    // app.run().await;
    app.event_loop.run(move |event, _target, control_flow| {
        app.world.run();

        if app.quit {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                if !app.renderer.input(event) {
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
                            app.renderer.resize(*physical_size);
                        },

                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.renderer.resize(**new_inner_size);
                        },

                        _ => {},
                    }
                }
            },

            Event::RedrawRequested(window_id) if window_id == app.window.id() => {
                match app.renderer.render() {
                    Ok(_) => {},
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => app.renderer.resize(app.renderer.size.clone()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                app.window.request_redraw();
            },
            _ => {},
        }
    });
}
