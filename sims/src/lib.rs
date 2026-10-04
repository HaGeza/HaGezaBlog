pub mod camera_w_controls;
pub mod config {
    pub mod camera;
    pub mod macroquad;
}
pub mod mesh {
    pub mod common;
    pub(in crate::mesh) mod lathe_mesh;
    pub mod material;
    pub mod models {
        pub mod light_bulb;
        pub mod light_switch;
        pub mod model_3d;
        pub mod wire;
    }
}
pub mod shape {
    pub mod common;
    pub mod lathe_profile;
}
pub(crate) mod util {
    pub(crate) mod linalg;
    pub(crate) mod vec;
}
