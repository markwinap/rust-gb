use gb_core::hardware::input::{Controller, Button};
use std::collections::HashMap;

pub struct GliumController {
    state: HashMap<Button, bool>
}

impl GliumController {
    pub fn new() -> Self {
        Self {
            state: HashMap::new()
        }
    }

    pub fn key_pressed(&mut self, button: Button) {
        self.state.insert(button, true);
    }

    pub fn key_released(&mut self, button: Button) {
        self.state.insert(button, false);
    }
}

impl Controller for GliumController {
    fn is_pressed(&self, button: Button) -> bool {
        match self.state.get(&button) {
            Some(value) => *value,
            None => false
        }
    }

    fn tick(&self) {}
}