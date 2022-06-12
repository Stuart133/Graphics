use cgmath::*;
use winit::event::*;

use crate::ControlEvent;

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(
        eye: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Camera {
        Camera {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub trait ControlMode {
    fn update_camera(&self, controller: &mut CameraController, camera: &mut Camera);
}

pub struct CameraController<'a> {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    /// Mouse movement delta still to be applied
    x_delta: f32,
    y_delta: f32,

    mode: &'a dyn ControlMode,
}

impl<'a> CameraController<'a> {
    pub fn new(speed: f32, mode: &'a dyn ControlMode) -> CameraController<'a> {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            x_delta: 0.0,
            y_delta: 0.0,
            mode: mode,
        }
    }

    pub fn process_events(&mut self, event: &ControlEvent) -> bool {
        match event {
            ControlEvent::WindowEvent(event) => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    let is_pressed = *state == ElementState::Pressed;
                    match keycode {
                        VirtualKeyCode::W | VirtualKeyCode::Up => {
                            self.is_forward_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::A | VirtualKeyCode::Left => {
                            self.is_left_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::S | VirtualKeyCode::Down => {
                            self.is_backward_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::D | VirtualKeyCode::Right => {
                            self.is_right_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::E => {
                            self.is_up_pressed = is_pressed;
                            true
                        }
                        VirtualKeyCode::Q => {
                            self.is_down_pressed = is_pressed;
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            },
            ControlEvent::DeviceEvent(event) => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.x_delta -= delta.0 as f32;
                    self.y_delta -= delta.1 as f32;
                    true
                }
                _ => false,
            },
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        self.mode.update_camera(self, camera);
    }
}

pub struct RotateMode;

impl ControlMode for RotateMode {
    fn update_camera(&self, controller: &mut CameraController, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if controller.is_forward_pressed && forward_mag > controller.speed {
            camera.eye += forward_norm * controller.speed;
        }
        if controller.is_backward_pressed {
            camera.eye -= forward_norm * controller.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if controller.is_right_pressed {
            camera.eye =
                camera.target - (forward + right * controller.speed).normalize() * forward_mag;
        }
        if controller.is_left_pressed {
            camera.eye =
                camera.target - (forward - right * controller.speed).normalize() * forward_mag;
        }
    }
}

pub struct MoveMode;

impl ControlMode for MoveMode {
    fn update_camera(&self, controller: &mut CameraController, camera: &mut Camera) {
        let mut view = camera.target - camera.eye;

        // Handle look first, then move along the view vector
        let rotate = Matrix3::from_angle_x(Deg::<f32>(controller.y_delta * controller.speed))
            * Matrix3::from_angle_y(Deg::<f32>(controller.x_delta * controller.speed));
        view = rotate * view;
        camera.target = camera.eye + view;

        controller.x_delta = 0.0;
        controller.y_delta = 0.0;

        if controller.is_forward_pressed {
            camera.eye += view * controller.speed;
            camera.target += view * controller.speed;
        }

        if controller.is_backward_pressed {
            camera.eye -= view * controller.speed;
            camera.target -= view * controller.speed;
        }

        if controller.is_right_pressed {
            camera.eye += view.cross(camera.up) * controller.speed;
            camera.target += view.cross(camera.up) * controller.speed;
        }

        if controller.is_left_pressed {
            camera.eye -= view.cross(camera.up) * controller.speed;
            camera.target -= view.cross(camera.up) * controller.speed;
        }

        if controller.is_up_pressed {
            camera.eye += camera.up * controller.speed;
            camera.target += camera.up * controller.speed;
        }

        if controller.is_down_pressed {
            camera.eye -= camera.up * controller.speed;
            camera.target -= camera.up * controller.speed;
        }
    }
}

#[cfg(test)]
mod tests {
    use cgmath::*;

    use super::{Camera, MoveMode, CameraController};

    fn generate_test_camera() -> Camera {
        Camera{
            eye: Point3{ x: 0.0, y: 0.0, z: 0.0 },
            target: Point3{ x: 1.0, y: 1.0, z: 1.0 },
            up: Vector3{ x: 0.0, y: 1.0, z: 0.0 },
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    #[test]

    pub fn update_x_view_movemode() {
        let mode = MoveMode{};
        let mut controller = CameraController::new(1.0, &mode);
        let mut camera = generate_test_camera();

        controller.x_delta = 90.0;
        controller.update_camera(&mut camera);

        assert_abs_diff_eq!(camera.target, Point3{x: 1.0, y: 1.0, z: -1.0 });
    }
    #[test]
    pub fn update_y_view_movemode() {
        let mode = MoveMode{};
        let mut controller = CameraController::new(1.0, &mode);
        let mut camera = generate_test_camera();

        controller.y_delta = 90.0;
        controller.update_camera(&mut camera);

        assert_abs_diff_eq!(camera.target, Point3{x: 1.0, y: -1.0, z: 1.0 });
    }

    #[test] 
    pub fn update_both_view_movemode() {
        let mode = MoveMode{};
        let mut controller = CameraController::new(1.0, &mode);
        let mut camera = generate_test_camera();

        controller.x_delta = -90.0;
        controller.y_delta = -90.0;
        controller.update_camera(&mut camera);

        assert_abs_diff_eq!(camera.target, Point3{x: -1.0, y: 1.0, z: -1.0 });
    }
}