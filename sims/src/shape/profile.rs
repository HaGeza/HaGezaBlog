use macroquad::math::Vec2;

pub struct Profile {
    outline: Vec<Vec2>,
    holes: Vec<Vec<Vec2>>,
}

impl Profile {
    pub fn new(outline: Vec<Vec2>, holes: Vec<Vec<Vec2>>) -> Self {
        Profile { outline, holes }
    }

    pub fn whole(outline: Vec<Vec2>) -> Self {
        Profile { outline, holes: vec![] }
    }

    pub fn outline(&self) -> &[Vec2] {
        &self.outline
    }

    pub fn holes(&self) -> &[Vec<Vec2>] {
        &self.holes
    }
}
