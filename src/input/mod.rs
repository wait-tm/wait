mod mouse;
mod keyboard;

use winit::keyboard::KeyCode;
use winit::window::Window;
use keyboard::KeyboardState;
use mouse::MouseState;

#[derive(Default)]
pub struct Input {
    keyboard: KeyboardState,
    mouse: MouseState
}

impl Input {
    pub fn new() -> Input {
        Default::default()
    }

    pub fn set_key(&mut self, key: KeyCode, pressed: bool) {
        self.keyboard.set_key(key, pressed)
    }
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn add_mouse_motion(&mut self, delta: (f64, f64)) {
        self.mouse.add_motion(delta.0, delta.1)
    }
    pub fn take_mouse_motion(&mut self) -> (f32, f32) {
        self.mouse.take_motion()
    }

    pub fn lock_mouse(&mut self, window: &Window) {
        self.mouse.lock(window);
    }
    pub fn unlock_mouse(&mut self, window: &Window) {
        self.mouse.unlock(window);
    }
    pub fn is_mouse_locked(&self) -> bool {
        self.mouse.is_locked()
    }
}
