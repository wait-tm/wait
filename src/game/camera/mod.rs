use glam::{Mat4, Vec3};
use crate::game::Player;

pub struct Camera {
    pub aspect: f32,
    pub fov: f32,

    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            aspect,
            fov: 70.0f32.to_radians(),

            near: 0.1,
            far: 1000.0,
        }
    }


    pub fn view_matrix(&self, player: &Player) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(
            player.position,
            player.position + player.look_direction(),
            Vec3::Y
        )
    }

    pub fn projection_matrix(&self) -> Mat4 {
        glam::camera::rh::proj::directx::perspective(
            self.fov,
            self.aspect,
            self.near,
            self.far,
        )
    }

    pub fn view_projection_matrix(&self, player: &Player) -> Mat4 {
        self.projection_matrix() * self.view_matrix(player)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        if width == 0.0f32 || height == 0.0f32 {
            return;
        }

        self.aspect = width / height;
    }
}