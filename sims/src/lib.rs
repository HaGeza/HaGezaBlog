pub mod camera_w_controls;
pub mod config;
pub mod mesh {
    pub(in crate::mesh) mod common;
    pub(in crate::mesh) mod lathe_mesh;
    pub mod models {
        pub mod light_bulb;
        pub mod light_switch;
        pub mod wire;
    }
}
pub mod shape {
    pub mod common;
}
pub(crate) mod util {
    pub(crate) mod linalg;
    pub(crate) mod vec;
}
