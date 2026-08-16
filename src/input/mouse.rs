use winit::window::{CursorGrabMode, Window};

pub(super) struct MouseState {
    delta_x: f32,
    delta_y: f32,

    locked: bool
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            delta_x: 0.0,
            delta_y: 0.0,
            locked: false
        }
    }
}

impl MouseState {
    pub fn add_motion(&mut self, dx: f64, dy: f64) {
        if !self.locked {
            return;
        }

        self.delta_x += dx as f32;
        self.delta_y += dy as f32;
    }

    pub fn reset_motion(&mut self) {
        self.delta_x = 0.0;
        self.delta_y = 0.0;
    }

    pub fn take_motion(&mut self) -> (f32, f32) {
        let dx = self.delta_x;
        let dy = self.delta_y;
        self.delta_x = 0.0;
        self.delta_y = 0.0;
        (dx, dy)
    }

    pub fn lock(&mut self, window: &Window) {
        window
            .set_cursor_grab(CursorGrabMode::Locked)
            .expect("failed to lock cursor");

        window.set_cursor_visible(false);

        self.locked = true;
    }

    pub fn unlock(&mut self, window: &Window) {
        window
            .set_cursor_grab(CursorGrabMode::None)
            .expect("failed to unlock cursor");

        window.set_cursor_visible(true);

        self.locked = false;

        self.reset_motion()
    }

    pub fn is_locked(&self) -> bool {
     self.locked
    }
}
