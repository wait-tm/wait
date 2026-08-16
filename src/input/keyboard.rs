use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub(super) struct KeyboardState {
    pressed: HashSet<KeyCode>
}

impl KeyboardState {
    pub fn set_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.pressed.insert(key);
        } else {
            self.pressed.remove(&key);
        }
    }
    
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }
    
    
}