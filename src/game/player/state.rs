use glam::Vec3;

pub struct Player {
    pub position: Vec3,

    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.5, 0.0),

            yaw: -90.0f32.to_radians(),
            pitch: 0.0f32.to_radians(),
        }
    }
}

impl Player {
    pub fn look_direction(&self) -> Vec3 {
        let yaw = self.yaw;
        let pitch = self.pitch;

        Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        ).normalize()
    }

    pub fn movement_forward(&self) -> Vec3 {
        let yaw = self.yaw;

        Vec3::new(
            yaw.cos(),
            0.0,
            yaw.sin()
        ).normalize()
    }

    pub fn movement_right(&self) -> Vec3 {
        self.movement_forward()
            .cross(Vec3::Y)
            .normalize()
    }

    pub fn movement_up(&self) -> Vec3 {
        Vec3::new(
            0.0,
            1.0,
            0.0
        ).normalize()
    }
}