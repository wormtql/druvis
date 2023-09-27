use cgmath::{Matrix4, SquareMatrix};
use winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta};


pub trait GetViewProjectionMatrix {
    fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    fn view_matrix(&self) -> Matrix4<f32>;

    fn projection_matrix(&self) -> Matrix4<f32>;
}

pub trait CameraController<T> {
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        false
    }

    fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {

    }

    fn process_scroll(&mut self, delta: &MouseScrollDelta) {

    }

    fn update_camera(&mut self, camera: &mut T, delta_time: instant::Duration);
}
