use macroquad::math::Vec3;

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
}

pub const XY_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::Z };
pub const XZ_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::Y };
pub const YZ_PLANE: Plane = Plane { origin: Vec3::ZERO, normal: Vec3::X };
