use std::path::Path;

use camera::{Camera, MoveMode, CameraController};
use cgmath::Vector3;
use render::{ControlEvent, State};
use wgpu::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod model;
mod obj;
mod render;
mod swp;
mod texture;
mod transform;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_cursor_grab(true).expect("could not grab cursor");

    let camera = Camera::new(
        (0.0, 10.0, 0.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vector3::unit_y(),
        window.inner_size().width as f32 / window.inner_size().height as f32,
        45.0,
        0.1,
        100.0,
    );
    let camera_controller = CameraController::new(0.2, &MoveMode {});

    let mut state = State::new(&window, camera, camera_controller).await;
    let model = obj::load_model(Path::new("./data/sphere.obj")).expect("model loading failed");
    state.add_model(model);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(&ControlEvent::WindowEvent(event)) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::R),
                                ..
                            },
                        ..
                    } => {
                        let model = obj::load_model(Path::new("./data/cube.obj"))
                            .expect("model loading failed");
                        state.add_model(model);
                    }
                    _ => {}
                }
            }
        }
        Event::DeviceEvent { event, .. } => {
            state.input(&ControlEvent::DeviceEvent(event));
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(SurfaceError::Lost) => state.recreate(),
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    })
}
