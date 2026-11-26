pub mod three_d {
    pub mod camera_w_controls;
}
pub mod mesh {
    pub(in crate::mesh) mod common;
    pub(in crate::mesh) mod extrusion_mesh;
    pub mod frag_feature;
    pub(in crate::mesh) mod lathe_mesh;
    pub(in crate::mesh) mod material;
    pub mod models {
        pub mod light_bulb;
        pub mod light_switch;
        pub(in crate::mesh) mod model;
        pub mod wire;
    }
    pub(in crate::mesh) mod plane;
}
pub(crate) mod shape {
    pub mod common;
    pub mod lathe_profile;
    pub mod profile;
}
pub mod util {
    pub mod config;
    pub mod linalg;
    pub mod macroquad;
    pub mod vec;
}
