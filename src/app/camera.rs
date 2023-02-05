use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::math::{
    lerp,
    linear_algebra::{matrix::Matrix4, quaternion::Quaternion, vector::Vector4},
};

pub struct Camera {
    pub input: WinitInputHelper,
    pub position: Vector4,
    pub direction: Vector4,
    pub turbo: bool,
    pub speed: f32,
    pub h_speed: f32,
    pub v_speed: f32,
    pub h_angle: f32,
    pub v_angle: f32,
    pub h_angle_f: f32,
    pub v_angle_f: f32,
    pub move_friction: f32,
    pub look_friction: f32,
}

impl Camera {
    pub fn new(position: Vector4, direction: Vector4) -> Self {
        Self {
            input: WinitInputHelper::new(),
            position,
            direction,
            turbo: false,
            speed: 0.0,
            v_speed: 0.0,
            h_speed: 0.0,
            h_angle: 0.0,
            v_angle: 0.0,
            h_angle_f: 0.0,
            v_angle_f: 0.0,
            move_friction: 10.0,
            look_friction: 10.0,
        }
    }

    pub fn transform(&self) -> Matrix4 {
        let mut matrix = Matrix4::new_identity();
        matrix.look_at(self.position, self.position + self.direction, Vector4::UP);
        return matrix;
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) {
        self.input.update(event);
    }

    pub fn update(&mut self, dt: f32) {
        // @todo: move into member fields
        let move_speed = 100.0;
        let look_speed = 3.0;

        if self.input.key_held(VirtualKeyCode::W) {
            self.speed += move_speed * dt;
        } else if self.input.key_held(VirtualKeyCode::S) {
            self.speed -= move_speed * dt;
        }

        if self.input.key_held(VirtualKeyCode::E) {
            self.v_speed += move_speed * dt;
        } else if self.input.key_held(VirtualKeyCode::Q) {
            self.v_speed -= move_speed * dt;
        }

        if self.input.key_held(VirtualKeyCode::A) {
            self.h_speed += move_speed * dt;
        } else if self.input.key_held(VirtualKeyCode::D) {
            self.h_speed -= move_speed * dt;
        }

        if self.input.key_held(VirtualKeyCode::Up) {
            self.v_angle += look_speed * dt;
        } else if self.input.key_held(VirtualKeyCode::Down) {
            self.v_angle -= look_speed * dt;
        }

        if self.input.key_held(VirtualKeyCode::Left) {
            self.h_angle -= look_speed * dt;
        } else if self.input.key_held(VirtualKeyCode::Right) {
            self.h_angle += look_speed * dt;
        }

        self.h_angle_f = lerp(self.h_angle_f, self.h_angle, self.look_friction * dt);
        self.v_angle_f = lerp(self.v_angle_f, self.v_angle, self.look_friction * dt);

        let horizontal_quat = Quaternion::from_angle(self.h_angle_f, Vector4::UP);
        let vertical_quat = Quaternion::from_angle(self.v_angle_f, Vector4::RIGHT);

        let view_quat = horizontal_quat * vertical_quat;

        self.direction = -Vector4::FORWARD;
        self.direction = self.direction.rotate_quaternion(view_quat);

        self.position = self.position + self.direction * self.speed * dt;
        self.speed = lerp(self.speed, 0.0, self.move_friction * dt);

        let right = Vector4::new(-self.direction.z, 0.0, self.direction.x, 0.0);

        self.position = self.position + right * self.h_speed * dt;
        self.h_speed = lerp(self.h_speed, 0.0, self.move_friction * dt);

        self.position = self.position + Vector4::UP * self.v_speed * dt;
        self.v_speed = lerp(self.v_speed, 0.0, self.move_friction * dt);
    }
}
