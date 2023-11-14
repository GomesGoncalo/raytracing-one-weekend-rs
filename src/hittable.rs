use std::{cell::RefCell, rc::Rc};

use crate::{
    material::Material,
    point::{Point, Vector},
    ray::Ray,
};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<RefCell<dyn Material>>,
}

pub enum Interval {
    Empty,
    Universe,
    Some(f64, f64),
}

impl Interval {
    pub fn new_set_interval(min: f64, max: f64) -> Self {
        if min == f64::MIN && max == f64::MAX {
            Self::Universe
        } else if min == f64::MAX && max == f64::MIN {
            Self::Empty
        } else {
            Self::Some(min, max)
        }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        match self {
            Self::Universe => true,
            Self::Empty => false,
            Self::Some(min, max) => *min < x && x < *max,
        }
    }

    pub fn min(&self) -> f64 {
        match self {
            Self::Universe => f64::MIN,
            Self::Empty => f64::MAX,
            Self::Some(min, _) => *min,
        }
    }

    pub fn max(&self) -> f64 {
        match self {
            Self::Universe => f64::MAX,
            Self::Empty => f64::MIN,
            Self::Some(_, max) => *max,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Option<Vec<Box<dyn Hittable>>>) -> Self {
        match list {
            None => Self { list: Vec::new() },
            Some(objs) => Self { list: objs },
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.list.iter().fold(None, |hit_record, x| {
            match x.hit(
                r,
                &Interval::new_set_interval(
                    ray_t.min(),
                    hit_record.as_ref().map_or(ray_t.max(), |x: &HitRecord| x.t),
                ),
            ) {
                Some(hr) => Some(hr),
                None => hit_record,
            }
        })
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.list.push(obj);
    }
}
