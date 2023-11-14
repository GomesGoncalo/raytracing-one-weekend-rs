use crate::{
    hittable::{HittableList, Interval},
    point::{cross, Point, Vector},
    ray::Ray,
    utils::random_between,
};
use image::ImageBuffer;
use radians::Deg;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    samples_per_pixel: u32,
    max_depth: u32,
    defocus_angle: f64,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct CameraInit {
    pub vfov: f64,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vector,
    pub focus_dist: f64,
    pub defocus_angle: f64,
    pub samples_per_pixel: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, init_params: CameraInit) -> Self {
        let image_height = std::cmp::max((f64::from(image_width) / aspect_ratio) as u32, 1);

        let camera_center = init_params.lookfrom;

        let theta = Deg::new(init_params.vfov).rad();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * init_params.focus_dist;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let w = (init_params.lookfrom - init_params.lookat)
            .unit()
            .unwrap_or_default();
        let u = cross(&init_params.vup, &w);
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = (viewport_u / f64::from(image_width)).unwrap();
        let pixel_delta_v = (viewport_v / f64::from(image_height)).unwrap();

        let viewport_upper_left = camera_center
            - (init_params.focus_dist * w)
            - (viewport_u / 2.0).unwrap()
            - (viewport_v / 2.0).unwrap();

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            init_params.focus_dist * (Deg::new(init_params.defocus_angle / 2.0).rad()).tan();

        Self {
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: init_params.samples_per_pixel,
            max_depth: 50,
            defocus_angle: init_params.defocus_angle,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_height);

        let mut bar = progress::BarBuilder::new()
            .left_cap("<")
            .right_cap(">")
            .empty_symbol("-")
            .filled_symbol("#")
            .build();

        bar.set_job_title("Rendering...");

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let sum = (0..self.samples_per_pixel)
                .map(|_| {
                    let ray = self.get_ray(f64::from(x), f64::from(y));
                    Camera::ray_color(&ray, self.max_depth, world)
                })
                .fold(Point::new(0.0, 0.0, 0.0), |acc, point| acc + point);
            let sum = (sum / f64::from(self.samples_per_pixel)).unwrap_or_default();
            let sum = sum.sqrt();

            *pixel = image::Rgb::from(sum);
            bar.reach_percent((y as f64 / self.image_height as f64 * 100.0) as i32);
        }
        imgbuf.save("fractal.png").unwrap();
    }

    fn ray_color(ray: &Ray, depth: u32, world: &HittableList) -> Point {
        if depth == 0 {
            return Point::default();
        }
        if let Some(record) = world.hit(ray, &Interval::new_set_interval(0.001, f64::MAX)) {
            match record.mat.borrow().scatter(ray, &record) {
                Some((attenuation, scattered)) => {
                    Camera::ray_color(&scattered, depth - 1, world) * attenuation
                }
                None => Point::new(0.0, 0.0, 0.0),
            }
        } else {
            let unit_direction = ray.direction().unit().unwrap();
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Vector::new(1.0, 1.0, 1.0) + a * Vector::new(0.5, 0.7, 1.0)
        }
    }

    fn pixel_sample_square(&self) -> Vector {
        let px = -0.5 + random_between(0.0, 1.0);
        let py = -0.5 + random_between(0.0, 1.0);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let pixel_center = self.pixel00_loc + (x * self.pixel_delta_u) + (y * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Point::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
