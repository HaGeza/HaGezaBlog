pub mod three_d {
    pub mod camera_w_controls;
}
pub mod mesh {
    pub mod common;
    pub mod frag_feature;
    pub(in crate::mesh) mod lathe_mesh;
    pub mod material;
    pub mod models {
        pub mod light_bulb;
        pub mod light_switch;
        pub mod wire;
    }
}
pub mod shape {
    pub mod common;
    pub mod lathe_profile;
}
pub mod util {
    pub mod config;
    pub(crate) mod linalg;
    pub mod macroquad;
    pub(crate) mod vec;
}
