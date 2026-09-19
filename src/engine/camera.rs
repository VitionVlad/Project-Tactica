use crate::engine::math::vec4::Vec4;

use super::{
    math::{mat4::Mat4, vec3::Vec3},
    physics::physics::PhysicsObject,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Camera {
    pub physic_object: PhysicsObject,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    pub is_orthographic: bool,
    pub rotation_colision_calc: bool,
}

impl Camera {
    #[allow(dead_code)]
    pub fn look_at(&mut self, target: Vec3) {
        let direction = target - self.physic_object.pos;
        let horizontal_length = (direction.x * direction.x + direction.z * direction.z).sqrt();

        if direction.length_squared() <= f32::EPSILON {
            return;
        }

        let pitch = f32::atan2(direction.y, horizontal_length);
        let yaw = f32::atan2(direction.x, direction.z);
        self.physic_object.rot = Vec3 {
            x: pitch,
            y: yaw - 3.14159,
            z: 0.0,
        }
        .to_quat();
    }
    #[allow(dead_code)]
    pub fn get_projection(&mut self, aspect: f32) -> Mat4 {
        self.physic_object.enable_rotation = self.rotation_colision_calc;
        let mut ubm = Mat4::new();
        if !self.is_orthographic {
            ubm.perspective(self.fov, self.zfar, self.znear, aspect);
        } else {
            ubm.orthographic(
                self.fov * aspect,
                -self.fov * aspect,
                self.fov,
                -self.fov,
                self.znear,
                self.zfar,
            );
        }
        let mut t: Mat4 = Mat4::new();
        //t.xrot(self.physic_object.rot.x);
        //ubm *= t;
        //t = Mat4::new();
        //t.yrot(self.physic_object.rot.y);
        //ubm *= t;
        //t = Mat4::new();
        //t.zrot(self.physic_object.rot.z);
        //ubm *= t;
        let view_rot = Vec4 {
            x: -self.physic_object.rot.x,
            y: -self.physic_object.rot.y,
            z: -self.physic_object.rot.z,
            w: self.physic_object.rot.w,
        };
        t.quat_rot(view_rot);
        //t.quat_rot(self.physic_object.rot);
        ubm *= t;
        t = Mat4::new();
        t.trans(Vec3 {
            x: -self.physic_object.pos.x,
            y: -self.physic_object.pos.y,
            z: -self.physic_object.pos.z,
        });
        ubm *= t;
        ubm.transpose();
        return ubm;
    }
}
