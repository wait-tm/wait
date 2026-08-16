use crate::game::Player;
use winit::keyboard::KeyCode;
use crate::input::Input;

pub struct PlayerController {
    speed: f32,
    sensitivity: f32,
    pitch_limit: f32,
}

impl Default for PlayerController {
    fn default() -> PlayerController {
        PlayerController {
            speed: 4.0,
            sensitivity: 0.1f32.to_radians(),
            pitch_limit: 89.0f32.to_radians(),
        }
    }
}

impl PlayerController {
    pub fn update(
        &self,
        player: &mut Player,
        input: &mut Input,
        delta_time: f32
    ) {
        let forward = player.movement_forward();
        let right = player.movement_right();
        let up = player.movement_up();

        if input.is_key_pressed(KeyCode::KeyW) {
            player.position += forward * self.speed * delta_time
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            player.position -= forward *self.speed * delta_time
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            player.position -= right * self.speed * delta_time
        }
        if input.is_key_pressed(KeyCode::KeyD) {
            player.position += right * self.speed * delta_time
        }
        if input.is_key_pressed(KeyCode::Space) {
            player.position += up * self.speed * delta_time
        }
        if input.is_key_pressed(KeyCode::ShiftLeft) {
            player.position -= up * self.speed * delta_time
        }

        let (dx, dy) = input.take_mouse_motion();
        let sensitivity = self.sensitivity;

        player.yaw += dx * sensitivity;
        player.pitch -= dy * sensitivity;

        player.pitch = player.pitch.clamp(-self.pitch_limit, self.pitch_limit);
    }
}