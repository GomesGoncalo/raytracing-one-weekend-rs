use crate::{
    hittable::HitRecord,
    material::Material,
    point::{reflect, Point},
    ray::Ray,
};

pub struct Metal {
    color: Point,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Point, fuzz: f64) -> Self {
        Self { color, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)> {
        let reflected = reflect(&r_in.direction().unit()?, &rec.normal);

        Some((
            self.color,
            Ray::new(
                rec.p,
                reflected + self.fuzz * Point::random_in_unit_vector()?,
            ),
        ))
    }
}
