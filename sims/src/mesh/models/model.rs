use macroquad::{
    material::{Material, gl_use_default_material, gl_use_material},
    math::Mat4,
    models::{Mesh, draw_mesh},
    window::get_internal_gl,
};

use crate::mesh::material::draw_transparent_mesh;

pub struct MeshWMaterial {
    mesh: Mesh,
    material: Material,
    transparent: bool,
}

impl MeshWMaterial {
    pub fn new(mesh: Mesh, material: Material, transparent: bool) -> MeshWMaterial {
        MeshWMaterial { mesh, material, transparent }
    }

    pub fn basic(mesh: Mesh, material: Material) -> MeshWMaterial {
        MeshWMaterial { mesh, material, transparent: false }
    }

    pub fn draw(&self) {
        gl_use_material(&self.material);
        if self.transparent {
            draw_transparent_mesh(&self.mesh);
        } else {
            draw_mesh(&self.mesh);
        }
    }
}

pub fn draw_model(mesh_w_material_list: &[&MeshWMaterial], model_matrix: &Mat4) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(*model_matrix);
    }
    for mesh_w_material in mesh_w_material_list {
        mesh_w_material.draw();
    }
    gl_use_default_material();
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}
