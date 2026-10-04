use std::ops::{Add, Div};

use macroquad::math::Vec2;
use thiserror::Error;

use super::common::get_normal;

pub struct LatheProfile {
    points: Vec<Vec2>,
    normals: Vec<Vec2>,
}

#[derive(Error, Debug)]
pub enum LatheProfileError {
    #[error("profile contains point with negative x")]
    NegativeX,
}

impl LatheProfile {
    pub fn new(points: &[Vec2]) -> Result<Self, LatheProfileError> {
        if points.iter().any(|pt| pt.x < 0.) {
            return Err(LatheProfileError::NegativeX);
        }
        Ok(LatheProfile { points: points.to_vec(), normals: Self::get_normals(points) })
    }

    pub fn points(&self) -> &[Vec2] {
        &self.points
    }

    pub fn point(&self, ind: usize) -> &Vec2 {
        &self.points[ind]
    }

    pub fn normals(&self) -> &[Vec2] {
        &self.normals
    }

    pub fn normal(&self, ind: usize) -> &Vec2 {
        &self.normals[ind]
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    fn get_normals(points: &[Vec2]) -> Vec<Vec2> {
        let num_pts = points.len();
        let mut normals = vec![Vec2::ZERO; num_pts];

        normals[0] = get_normal([&points[0], &points[1]]);
        for pt_ind in 1..num_pts - 1 {
            normals[pt_ind] = get_normal([&points[pt_ind - 1], &points[pt_ind]])
                .add(get_normal([&points[pt_ind], &points[pt_ind + 1]]))
                .div(2.);
        }
        normals[num_pts - 1] = get_normal([&points[num_pts - 2], &points[num_pts - 1]]);

        normals
    }
}
