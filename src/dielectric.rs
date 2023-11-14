use crate::{
    hittable::HitRecord,
    material::Material,
    point::{dot, reflect, refract, Point},
    ray::Ray,
    utils::random_between,
};

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Point, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit()?;
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_between(0.0, 1.0)
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        Some((Point::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}
