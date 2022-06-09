use cgmath::*;
use winit::event::*;

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
    fn update_camera(&self, controller: &CameraController, camera: &mut Camera);
}

pub struct CameraController<'a> {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
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
            mode: mode,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
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
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        self.mode.update_camera(self, camera);
    }
}

pub struct RotateMode;

impl ControlMode for RotateMode {
    fn update_camera(&self, controller: &CameraController, camera: &mut Camera) {
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
            camera.eye = camera.target - (forward + right * controller.speed).normalize() * forward_mag;
        }
        if controller.is_left_pressed {
            camera.eye = camera.target - (forward - right * controller.speed).normalize() * forward_mag;
        }
    }
}

pub struct MoveMode;

impl ControlMode for MoveMode {
    fn update_camera(&self, controller: &CameraController, camera: &mut Camera) {
        if controller.is_forward_pressed {
            camera.eye.z -= controller.speed;
            camera.target.z -= controller.speed;
        }

        if controller.is_backward_pressed {
            camera.eye.z += controller.speed;
            camera.target.z += controller.speed;
        }

        if controller.is_right_pressed {
            camera.eye.x += controller.speed;
            camera.target.x += controller.speed;
        }

        if controller.is_left_pressed {
            camera.eye.x -= controller.speed;
            camera.target.x -= controller.speed;
        }

        if controller.is_up_pressed {
            camera.eye.y += controller.speed;
            camera.target.y += controller.speed;
        }

        if controller.is_down_pressed {
            camera.eye.y -= controller.speed;
            camera.target.y -= controller.speed;
        }
    }
}