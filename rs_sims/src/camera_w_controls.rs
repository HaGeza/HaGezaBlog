use std::f32::consts::FRAC_PI_4;

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
        let longitude = 0.;
        let latitude = FRAC_PI_4;
        let radius = 5.;

        Self {
            camera: Camera3D {
                position: _get_position_in_orbit(longitude, latitude, radius),
                target: vec3(0., 0., 0.),
                z_near: 0.0001,
                z_far: 1000.0,
                ..Camera3D::default()
            },
            longitude: longitude,
            latitude: latitude,
            radius: radius,
            orbit_sensitivity: vec2(0.5, 0.5),
            pan_sensitivity: vec2(1.0, 1.0),
            zoom_sensitivity: 0.05,
        }
    }
}

impl CameraWControls {
    pub fn update(&mut self, force_update: bool) {
        let mut updated = force_update;
        let mut position = self.camera.position;
        let mut target = self.camera.target;

        // Orbit / Rotate camera around target with left mouse
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            self.longitude += self.orbit_sensitivity.x * delta.x;
            self.latitude = (self.latitude + self.orbit_sensitivity.y * delta.y).clamp(-1.5, 1.5);
            updated = true;
        }
        // Move target and camera with right mouse
        else if is_mouse_button_down(MouseButton::Right) {
            let camera_to_target = target - position;
            let right = camera_to_target.cross(vec3(0., 1., 0.)).normalize();
            let up = right.cross(camera_to_target).normalize();

            let delta_2d = mouse_delta_position() * self.pan_sensitivity;
            target += right * delta_2d.x + up * delta_2d.y;
            updated = true;
        }
        // Zoom with mouse wheel
        else {
            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
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
