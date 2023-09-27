use std::f32::consts::FRAC_PI_2;

use cgmath::{Point3, Rad, Matrix4, Vector3, InnerSpace, perspective, Deg};
use winit::{event::{ElementState, VirtualKeyCode, MouseScrollDelta}, dpi::PhysicalPosition};

use super::{camera::{GetViewProjectionMatrix, CameraController}, camera_uniform::UpdateCameraUniform};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct PerspectiveCamera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self {
            position: Point3 { x: 0.0, y: 0.0, z: 0.0 },
            yaw: Rad(0.0),
            pitch: Rad(0.0),
            aspect: 16.0 / 9.0,
            fovy: Deg(60.0).into(),
            znear: 0.1,
            zfar: 1000.0
        }
    }
}

impl PerspectiveCamera {
    pub fn new<
        A: Into<Point3<f32>>,
        B: Into<Rad<f32>>,
        C: Into<Rad<f32>>,
        D: Into<Rad<f32>>,
    > (
        position: A,
        yaw: B,
        pitch: C,
        aspect: f32,
        fovy: D,
        znear: f32,
        zfar: f32
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            aspect,
            fovy: fovy.into(),
            znear,
            zfar
        }
    }
}

impl GetViewProjectionMatrix for PerspectiveCamera {
    fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            Vector3::unit_y(),
        )
    }

    fn projection_matrix(&self) -> Matrix4<f32> {
        // wgpu use z ranges [0, 1], while OpenGL is [-1, 1]
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

impl UpdateCameraUniform for PerspectiveCamera {
    fn update_uniform(&self, uniform: &mut super::camera_uniform::CameraUniform) {
        uniform.druvis_world_space_camera_position = [self.position.x, self.position.y, self.position.z, 1.0];
        uniform.druvis_view_matrix = self.view_matrix().into();
        uniform.druvis_projection_matrix = self.projection_matrix().into();
        uniform.druvis_projection_params = [1.0, self.znear, self.zfar, 1.0 / self.zfar];
        
    }
}

pub struct SimplePerspectiveCameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,

    speed: f32,
    sensitivity: f32,
}

impl SimplePerspectiveCameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }
}

impl Default for SimplePerspectiveCameraController {
    fn default() -> Self {
        Self::new(4.0, 0.4)
    }
}

impl CameraController<PerspectiveCamera> for SimplePerspectiveCameraController {
    fn process_keyboard(&mut self, key: winit::event::VirtualKeyCode, state: winit::event::ElementState) -> bool {
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    fn process_scroll(&mut self, delta: &winit::event::MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    fn update_camera(&mut self, camera: &mut PerspectiveCamera, delta_time: instant::Duration) {
        let dt = delta_time.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}
