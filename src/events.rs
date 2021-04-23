use std::collections::{BTreeMap, BTreeSet};
pub use winit::event::VirtualKeyCode as KeyCode;

#[derive(Default)]
pub struct Events {
    // how long has each been held?
    held: BTreeMap<KeyCode, usize>,
    // which have just been released?
    released: BTreeSet<KeyCode>,
    mouse_pos: (f32, f32),
    mouse_delta: (f32, f32),
    mouse_buttons: Vec<Option<usize>>,
    mouse_buttons_released: Vec<bool>,
}

impl Events {
    pub(crate) fn device_event(&mut self, ev: &winit::event::DeviceEvent) {
        match ev {
            winit::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
                self.mouse_delta = (*x as f32, *y as f32)
            }
            _ => {}
        }
    }
    pub(crate) fn window_event(&mut self, event: &winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let pressed = *state == winit::event::ElementState::Pressed;
                if pressed {
                    self.held.entry(*keycode).or_insert(0);
                } else {
                    self.released.insert(*keycode);
                }
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32)
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == winit::event::ElementState::Pressed;
                let button = match button {
                    winit::event::MouseButton::Left => 0,
                    winit::event::MouseButton::Right => 1,
                    winit::event::MouseButton::Middle => 2,
                    winit::event::MouseButton::Other(num) => *num,
                } as usize;
                self.mouse_buttons.reserve(button);
                self.mouse_buttons_released.reserve(button);
                while self.mouse_buttons.len() <= button {
                    self.mouse_buttons.push(None);
                    self.mouse_buttons_released.push(false);
                }
                if pressed {
                    self.mouse_buttons[button] = Some(0);
                } else {
                    self.mouse_buttons_released[button] = true;
                }
            }
            _ => {} // mouse, etc
        }
    }
    pub(crate) fn next_frame(&mut self) {
        let mut keep_release = vec![];
        for k in self.released.iter() {
            if let Some(0) = self.held.remove(k) {
                keep_release.push(*k);
            }
        }
        self.released.clear();
        self.released.extend(keep_release.into_iter());
        for (_k, d) in self.held.iter_mut() {
            *d += 1;
        }
        for (mcount, mreleased) in self
            .mouse_buttons
            .iter_mut()
            .zip(self.mouse_buttons_released.iter_mut())
        {
            // was the mouse button released?
            if *mreleased {
                // mcount will always end up None
                // if mcount was 0, stay released=true for a frame
                if let Some(0) = mcount.take() {
                    *mreleased = true;
                } else {
                    // otherwise (if it was none or > 0) set mreleased = false
                    *mreleased = false;
                }
            } else {
                *mcount = mcount.map(|num| num + 1);
            }
        }
        self.mouse_delta = (0.0, 0.0);
    }

    // Why does held need to ensure !released, and released need to check !pressed?
    // If a down and up event both arrive during rendering, we don't want to lose
    // the down, and we want to delay the up until the following simulation step.
    // In this setting, down up down could lead to a missed up event, but that
    // seems better than the alternative of missing down events.
    pub fn key_pressed(&self, k: KeyCode) -> bool {
        self.held.get(&k).map(|num| *num == 0).unwrap_or(false)
    }

    pub fn key_held(&self, k: KeyCode) -> bool {
        self.held.contains_key(&k) && !self.key_released(k)
    }

    pub fn key_released(&self, k: KeyCode) -> bool {
        self.released.contains(&k) && !self.key_pressed(k)
    }

    pub fn mouse_pressed(&self, button: usize) -> bool {
        self.mouse_buttons[button] == Some(0)
    }

    pub fn mouse_held(&self, button: usize) -> bool {
        self.mouse_buttons[button].is_some() && !self.mouse_released(button)
    }

    pub fn mouse_released(&self, button: usize) -> bool {
        self.mouse_buttons_released[button] && !self.mouse_pressed(button)
    }

    pub fn mouse_pos(&self) -> (f32, f32) {
        (self.mouse_pos.0 as f32, self.mouse_pos.1 as f32)
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        (self.mouse_delta.0 as f32, self.mouse_delta.1 as f32)
    }
}
