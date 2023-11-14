mod camera;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod point;
mod ray;
mod sphere;
mod utils;
use std::{cell::RefCell, rc::Rc};

use camera::{Camera, CameraInit};
use dielectric::Dielectric;
use hittable::HittableList;
use lambertian::Lambertian;
use metal::Metal;
use point::{Point, Vector};
use sphere::Sphere;
use utils::random_between;

fn main() {
    let camera_init = CameraInit {
        vfov: 20.0,
        lookfrom: Point::new(13.0, 2.0, 3.0),
        lookat: Point::new(0.0, 0.0, 0.0),
        vup: Vector::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.6,
        samples_per_pixel: 500,
    };
    let camera = Camera::new(16.0 / 9.0, 720, camera_init);
    let mut world = HittableList::new(None);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_between(0.0, 1.0);
            let center = Point::new(
                f64::from(a) + 0.9 * random_between(0.0, 1.0),
                0.2,
                f64::from(b) + 0.9 * random_between(0.0, 1.0),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Point::random() * Point::random();
                    let material = Rc::new(RefCell::new(Lambertian::new(albedo)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = Point::random();
                    let fuzz = random_between(0.0, 0.5);
                    let material = Rc::new(RefCell::new(Metal::new(albedo, fuzz)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Rc::new(RefCell::new(Dielectric::new(1.5)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, -1.0),
        1000.0,
        Rc::new(RefCell::new(Lambertian::new(Point::new(0.5, 0.5, 0.5)))),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(RefCell::new(Lambertian::new(Point::new(0.4, 0.2, 0.1)))),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(RefCell::new(Dielectric::new(1.5))),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(RefCell::new(Metal::new(Point::new(0.7, 0.6, 0.5), 0.0))),
    )));
    camera.render(&world);
}
