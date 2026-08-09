use std::collections::HashMap;

use crate::config::load_camera_config;
use macroquad::prelude::*;

pub struct CameraWControls {
    pub camera: Camera3D,

    longitude: f32,
    latitude: f32,
    radius: f32,

    rotate_mouse_sensitivity: Vec2,
    pan_mouse_sensitivity: Vec2,
    zoom_mouse_sensitivity: f32,

    rotate_touch_sensitivity: Vec2,
    pan_touch_sensitivity: Vec2,
    zoom_touch_sensitivity: f32,

    touch_zoom_threshold: f32,

    updated: bool,
    touch_positions: HashMap<u64, Vec2>,
}

fn _get_camera_relative_position(longitude: f32, latitude: f32, radius: f32) -> Vec3 {
    vec3(
        radius * latitude.cos() * longitude.sin(),
        radius * latitude.sin(),
        radius * latitude.cos() * longitude.cos(),
    )
}

impl Default for CameraWControls {
    fn default() -> Self {
        let config = load_camera_config().unwrap_or_default();

        let longitude = config.longitude;
        let latitude = config.latitude;
        let radius = config.radius;

        Self {
            camera: Camera3D {
                position: _get_camera_relative_position(longitude, latitude, radius),
                target: vec3(0., 0., 0.),
                z_near: config.z_near,
                z_far: config.z_far,
                ..Camera3D::default()
            },
            longitude: longitude,
            latitude: latitude,
            radius: radius,
            rotate_mouse_sensitivity: config.rotate_mouse_sensitivity,
            pan_mouse_sensitivity: config.pan_mouse_sensitivity,
            zoom_mouse_sensitivity: config.zoom_mouse_sensitivity,
            rotate_touch_sensitivity: config.rotate_touch_sensitivity,
            pan_touch_sensitivity: config.pan_touch_sensitivity,
            zoom_touch_sensitivity: config.zoom_touch_sensitivity,
            touch_zoom_threshold: config.touch_zoom_threshold,
            updated: false,
            touch_positions: HashMap::default(),
        }
    }
}

enum TouchAction {
    Rotate(Vec2),
    Pan(Vec2),
    Zoom(f32),
}

impl CameraWControls {
    fn _rotate(&mut self, delta: Vec2, sensitivity: Vec2) {
        self.longitude += sensitivity.x * delta.x;
        self.latitude = (self.latitude + sensitivity.y * delta.y).clamp(-1.5, 1.5);
        self.updated = true;
    }

    fn _pan(&mut self, delta: Vec2, sensitivity: Vec2) -> Vec3 {
        let camera_to_target = self.camera.target - self.camera.position;
        let right = camera_to_target.cross(vec3(0., 1., 0.)).normalize();
        let up = right.cross(camera_to_target).normalize();

        self.updated = true;
        let adjusted_delta = delta * sensitivity;
        self.camera.target + right * adjusted_delta.x + up * adjusted_delta.y
    }

    fn _zoom(&mut self, delta: f32, sensitivity: f32) {
        if delta != 0.0 {
            self.radius = (self.radius - delta * sensitivity).max(0.01);
            self.updated = true;
        }
    }

    fn _process_touch_and_get_delta(&mut self, touch: &Touch) -> Vec2 {
        let mut delta = Vec2::ZERO;

        match touch.phase {
            TouchPhase::Started | TouchPhase::Stationary => {
                self.touch_positions.insert(touch.id, touch.position);
            },
            TouchPhase::Moved => {
                if let Some(prev_pos) = self.touch_positions.get(&touch.id) {
                    delta = touch.position - *prev_pos;
                }
                self.touch_positions.insert(touch.id, touch.position);
            },
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.touch_positions.remove(&touch.id);
            },
        }
        delta
    }

    fn _process_touches(&mut self, touches: &Vec<Touch>) -> Option<TouchAction> {
        if touches.len() == 1 {
            return Some(TouchAction::Rotate(self._process_touch_and_get_delta(&touches[0])));
        } else if touches.len() > 1 {
            let curr_dist = touches[0].position.distance(touches[1].position);

            let deltas = (
                self._process_touch_and_get_delta(&touches[0]),
                self._process_touch_and_get_delta(&touches[1]),
            );

            let prev_dist =
                (touches[0].position - deltas.0).distance(touches[1].position - deltas.1);

            let dist_delta = curr_dist - prev_dist;
            if dist_delta.abs() < self.touch_zoom_threshold {
                return Some(TouchAction::Pan((deltas.0 + deltas.1) / 2.));
            } else {
                return Some(TouchAction::Zoom(dist_delta));
            }
        }
        None
    }

    pub fn update(&mut self, force_update: bool) {
        self.updated = force_update;
        let mut target = self.camera.target;

        simulate_mouse_with_touch(false);
        let touches = touches();

        if touches.is_empty() {
            if is_mouse_button_down(MouseButton::Left) {
                set_cursor_grab(true);
                self._rotate(mouse_delta_position(), self.rotate_mouse_sensitivity);
            } else if is_mouse_button_down(MouseButton::Right) {
                set_cursor_grab(true);
                target = self._pan(mouse_delta_position(), self.pan_mouse_sensitivity);
            } else {
                set_cursor_grab(false);
                self._zoom(mouse_wheel().1, self.zoom_mouse_sensitivity);
            }
        } else {
            println!("touches: {:?}", &touches);
            match self._process_touches(&touches) {
                Some(TouchAction::Rotate(delta)) => {
                    self._rotate(delta, self.rotate_touch_sensitivity);
                },
                Some(TouchAction::Pan(delta)) => {
                    target = self._pan(delta, self.pan_touch_sensitivity);
                },
                Some(TouchAction::Zoom(delta)) => {
                    self._zoom(delta, self.zoom_touch_sensitivity);
                },
                None => {},
            }
        }

        if self.updated {
            let position =
                target + _get_camera_relative_position(self.longitude, self.latitude, self.radius);

            self.camera = Camera3D {
                position: position,
                target: target,
                up: vec3(0., 1., 0.),
                ..Default::default()
            };
            set_camera(&self.camera);
        }
    }
}
