use macroquad::math::{Vec2, Vec3};

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    basis: [Vec3; 2],
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        let basis_u = normal.cross(if normal.y.abs() < 0.99 { Vec3::Y } else { Vec3::X }).normalize();
        let basis_v = normal.cross(basis_u).normalize();
        Self { origin, normal, basis: [basis_u, basis_v] }
    }

    pub fn project_2d(&self, point: &Vec2) -> Vec3 {
        self.origin + self.basis[0] * point.x + self.basis[1] * point.y
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

pub const XY_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::Z, basis: [Vec3::X, Vec3::Y] };
pub const XZ_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::Y, basis: [Vec3::X, Vec3::Z] };
pub const YZ_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::X, basis: [Vec3::Y, Vec3::Z] };
