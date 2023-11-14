use crate::{hittable::HitRecord, point::Point, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)>;
}
