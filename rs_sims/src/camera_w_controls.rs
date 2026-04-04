use crate::config::load_camera_config;
use macroquad::prelude::*;

pub struct CameraWControls {
    pub camera: Camera3D,
    pub longitude: f32,
    pub latitude: f32,
    pub radius: f32,

    pub orbit_sensitivity: Vec2,
    pub pan_sensitivity: Vec2,
    pub zoom_sensitivity: f32,
}

fn _get_position_in_orbit(longitude: f32, latitude: f32, radius: f32) -> Vec3 {
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
                position: _get_position_in_orbit(longitude, latitude, radius),
                target: vec3(0., 0., 0.),
                z_near: config.z_near,
                z_far: config.z_far,
                ..Camera3D::default()
            },
            longitude: longitude,
            latitude: latitude,
            radius: radius,
            orbit_sensitivity: config.orbit_sensitivity,
            pan_sensitivity: config.pan_sensitivity,
            zoom_sensitivity: config.zoom_sensitivity,
        }
    }
}

impl CameraWControls {
    pub fn update(&mut self, force_update: bool) {
        let mut updated = force_update;
        let mut position = self.camera.position;
        let mut target = self.camera.target;

        if is_mouse_button_down(MouseButton::Left) {
            // Orbit / Rotate camera around target with left mouse
            set_cursor_grab(true);

            let delta = mouse_delta_position();
            self.longitude += self.orbit_sensitivity.x * delta.x;
            self.latitude = (self.latitude + self.orbit_sensitivity.y * delta.y).clamp(-1.5, 1.5);
            updated = true;
        } else if is_mouse_button_down(MouseButton::Right) {
            // Move target and camera with right mouse
            set_cursor_grab(true);

            let camera_to_target = target - position;
            let right = camera_to_target.cross(vec3(0., 1., 0.)).normalize();
            let up = right.cross(camera_to_target).normalize();

            let delta_2d = mouse_delta_position() * self.pan_sensitivity;
            target += right * delta_2d.x + up * delta_2d.y;
            updated = true;
        } else {
            set_cursor_grab(false);

            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                // Zoom with mouse wheel
                self.radius = (self.radius - wheel_y * self.zoom_sensitivity).max(0.01);
                updated = true;
            }
        }

        if updated {
            position = target + _get_position_in_orbit(self.longitude, self.latitude, self.radius);

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
