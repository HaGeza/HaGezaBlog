use macroquad::material::{Material, gl_use_default_material, gl_use_material};

pub trait Model3d {
    fn draw(&self);

    fn draw_with_material(&self, material: &Material) {
        gl_use_material(material);
        self.draw();
        gl_use_default_material();
    }
}
