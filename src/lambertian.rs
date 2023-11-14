use crate::{hittable::HitRecord, material::Material, point::Point, ray::Ray};

pub struct Lambertian {
    color: Point,
}

impl Lambertian {
    pub fn new(color: Point) -> Self {
        Self { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)> {
        let mut scatter_direction = rec.normal + Point::random_in_unit_vector()?;

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.color, Ray::new(rec.p, scatter_direction)))
    }
}
