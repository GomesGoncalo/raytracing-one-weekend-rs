use crate::point::{Point, Vector};

#[derive(Default, Copy, PartialEq, Clone, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
