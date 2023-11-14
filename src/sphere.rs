use std::cell::RefCell;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable, Interval};
use crate::material::Material;
use crate::point::{dot, Point};
use crate::ray::Ray;

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Rc<RefCell<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Rc<RefCell<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().len_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let normal = ((p - self.center) / self.radius)?;
        let front_face = dot(&r.direction(), &normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Some(HitRecord {
            t,
            p,
            normal,
            front_face,
            mat: self.mat.clone(),
        })
    }
}
