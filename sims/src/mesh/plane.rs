use macroquad::math::{Vec2, Vec3};

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    basis: [Vec3; 2],
}

impl Plane {
    pub fn from_normal(origin: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        let basis_u = normal.cross(if normal.y.abs() < 0.99 { Vec3::Y } else { Vec3::X }).normalize();
        let basis_v = normal.cross(basis_u).normalize();
        Self { origin, normal, basis: [basis_u, basis_v] }
    }

    pub fn from_basis(origin: Vec3, basis: [Vec3; 2]) -> Self {
        let basis_u = basis[0].normalize();
        let basis_v = basis[1].normalize();
        let normal = basis_u.cross(basis_v).normalize();
        Self { origin, normal, basis: [basis_u, basis_v] }
    }

    pub fn project_2d(&self, point: &Vec2) -> Vec3 {
        self.origin + self.basis[0] * point.x + self.basis[1] * point.y
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}
