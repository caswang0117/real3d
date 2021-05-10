use crate::geom::*;

pub struct CameraController {
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update(&mut self, engine: &mut crate::Engine) {
        use crate::events::KeyCode;
        let (upk, downk, leftk, rightk) = (
            engine.events.key_held(KeyCode::Up),
            engine.events.key_held(KeyCode::Down),
            engine.events.key_held(KeyCode::Left),
            engine.events.key_held(KeyCode::Right),
        );
        let camera = engine.camera_mut();
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        // if upk && forward_mag > self.speed {
        //     camera.eye += forward_norm * self.speed;
        // }
        // if downk {
        //     camera.eye -= forward_norm * self.speed;
        // }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if rightk {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if leftk {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
