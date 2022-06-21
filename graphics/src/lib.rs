use std::path::Path;

use camera::{Camera, CameraController, MoveMode};
use cgmath::{Vector3, Matrix4};
use curve::BezierCurve;
use render::{ControlEvent, Render2D, Render3D, Renderer};
use wgpu::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod curve;
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

    let mut state = Render3D::new(&window, camera, camera_controller).await;
    let model = obj::load_model(Path::new("./data/sphere.obj")).expect("model loading failed");
    state.add_model(model);

    let camera_controller = CameraController::new(0.2, &MoveMode {});
    let camera = Camera::new(
        (0.0, 10.0, 0.0).into(),
        (0.0, 0.0, 0.0).into(),
        Vector3::unit_y(),
        window.inner_size().width as f32 / window.inner_size().height as f32,
        45.0,
        0.1,
        100.0,
    );
    let mut render_2d = Render2D::new(&window, camera, camera_controller).await;

    let curve = BezierCurve{
        control_points: Matrix4::new(0.0, 0.0, 0.0, 0.0, 0.25, 1.0, 0.0, 0.0, 0.75, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
    };
    render_2d.add_curve(curve);    

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !render_2d.input(&ControlEvent::WindowEvent(event)) {
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
                        render_2d.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_2d.resize(**new_inner_size);
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
            render_2d.input(&ControlEvent::DeviceEvent(event));
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            render_2d.update();
            match render_2d.render() {
                Ok(_) => {}
                Err(SurfaceError::Lost) => render_2d.recreate(),
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
