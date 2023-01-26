use crate::math::{
    lerp,
    linear_algebra::{matrix::Matrix4, quaternion::Quaternion, vector::Vector4},
};

#[derive(Debug)]
pub struct Camera {
    pub position: Vector4,
    pub direction: Vector4,
    pub turbo: bool,
    pub speed: f32,
    pub h_speed: f32,
    pub v_speed: f32,
    pub friction: f32,
    pub h_angle: f32,
    pub v_angle: f32,
    pub h_angle_f: f32,
    pub v_angle_f: f32,
}

impl Camera {
    pub fn new(position: Vector4, direction: Vector4) -> Self {
        Self {
            position,
            direction,
            turbo: false,
            speed: 0.0,
            v_speed: 0.0,
            h_speed: 0.0,
            friction: 0.8,
            h_angle: 0.0,
            v_angle: 0.0,
            h_angle_f: 0.0,
            v_angle_f: 0.0,
        }
    }

    pub fn transform(&self) -> Matrix4 {
        let mut matrix = Matrix4::new_identity();
        matrix.look_at(self.position, self.position + self.direction, Vector4::UP);
        matrix
    }

    pub fn update(&mut self, dt: f32) {
        // println!("{:?}", self.position);

        self.h_angle_f = lerp(self.h_angle_f, self.h_angle, dt * 10.0);
        self.v_angle_f = lerp(self.v_angle_f, self.v_angle, dt * 10.0);

        // Quaternions
        let horizontal_quat = Quaternion::from_angle(self.h_angle_f, Vector4::UP);
        let vertical_quat = Quaternion::from_angle(self.v_angle_f, Vector4::RIGHT);

        let view_quat = horizontal_quat * vertical_quat;

        self.direction = -Vector4::FORWARD;
        self.direction = self.direction.rotate_quaternion(view_quat);

        self.position = self.position + self.direction * self.speed * dt;
        self.speed *= self.friction;

        let right = Vector4::new(-self.direction.z, 0.0, self.direction.x, 0.0);

        self.position = self.position + right * self.h_speed * dt;
        self.h_speed *= self.friction;

        self.position = self.position + Vector4::UP * self.v_speed * dt;
        self.v_speed *= self.friction;

        // log::info!("{:?}", self.h_angle);
    }

    // void update(const float& dt) {

    //             // Movement
    //             if(sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) turbo = true;
    //             else turbo = false;
    //             if(sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) speed += turbo ? 4.0f : 1.0f;
    //             else if(sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) speed -= turbo ? 4.0f : 1.0f;

    //             // Orientation
    //             if(sf::Keyboard::isKeyPressed(sf::Keyboard::A)) hAngle -= 2.0f * dt;
    //             else if(sf::Keyboard::isKeyPressed(sf::Keyboard::D)) hAngle += 2.0f * dt;
    //             if(sf::Keyboard::isKeyPressed(sf::Keyboard::S)) vAngle -= 2.0f * dt;
    //             else if(sf::Keyboard::isKeyPressed(sf::Keyboard::W)) vAngle += 2.0f * dt;

    //             hAnglef = lerp(hAnglef, hAngle, dt * 10.0f);
    //             vAnglef = lerp(vAnglef, vAngle, dt * 10.0f);

    //             // Quaternions
    //             Quaternion horizontalQuat (hAnglef, Vector4(0.0f, 1.0f, 0.0f));
    //             Quaternion verticalQuat   (vAnglef, Vector4(1.0f, 0.0f, 0.0f));

    //             Quaternion viewQuat = horizontalQuat * verticalQuat;

    //             direction = Vector4(0.0f, 0.0f, -1.0f, 0.0f);
    //             direction = direction.rotate(viewQuat);

    //             position = position + direction * speed * dt;
    //             speed *= friction;
    //         }
}
