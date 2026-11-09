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

    pub fn iter_pts(&self) -> impl Iterator<Item = &Vec2> {
        self.outline.iter().chain(self.holes.iter().flatten())
    }

    pub fn num_pts(&self) -> usize {
        self.outline.len() + self.holes.iter().fold(0, |acc, hole| acc + hole.len())
    }
}
