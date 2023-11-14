use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::random_between;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

pub type Vector = Point;

impl From<Point> for image::Rgb<u8> {
    fn from(p: Point) -> Self {
        let r = (p.x().clamp(0.0, 1.0) * 255.0) as u8;
        let g = (p.y().clamp(0.0, 1.0) * 255.0) as u8;
        let b = (p.z().clamp(0.0, 1.0) * 255.0) as u8;
        Self([r, g, b])
    }
}

pub fn dot(lhs: &Point, rhs: &Point) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn cross(lhs: &Point, rhs: &Point) -> Point {
    Point::new(
        lhs.y() * rhs.z() - lhs.z() * rhs.y(),
        lhs.z() * rhs.x() - lhs.x() * rhs.z(),
        lhs.x() * rhs.y() - lhs.y() * rhs.x(),
    )
}

pub fn reflect(lhs: &Point, rhs: &Point) -> Point {
    *lhs - *rhs * 2.0 * dot(lhs, rhs)
}

pub fn refract(lhs: &Point, rhs: &Point, etai_over_etat: f64) -> Point {
    let cos_theta = dot(&-*lhs, rhs).min(1.0);
    let r_out_perp = etai_over_etat * (*lhs + *rhs * cos_theta);
    let r_out_parallel = *rhs * (-(1.0 - r_out_perp.len_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.z
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn unit(&self) -> Option<Self> {
        *self / self.len()
    }

    pub fn random() -> Self {
        Self {
            x: random_between(0.0, 1.0),
            y: random_between(0.0, 1.0),
            z: random_between(0.0, 1.0),
        }
    }

    pub fn random_between(min: f64, max: f64) -> Self {
        Self {
            x: random_between(min, max),
            y: random_between(min, max),
            z: random_between(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_between(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_vector() -> Option<Self> {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Option<Self> {
        let on_unit_sphere = Self::random_in_unit_vector()?;
        if dot(&on_unit_sphere, normal) > 0.0 {
            Some(on_unit_sphere)
        } else {
            Some(-on_unit_sphere)
        }
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vector::new(random_between(-1.0, 1.0), random_between(-1.0, 1.0), 0.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Point> for Point {
    type Output = Self;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Point {
    type Output = Option<Point>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            None
        } else {
            Some(Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            })
        }
    }
}

#[test]
fn can_create_a_point_default() {
    let point = Point::default();
    assert!(point.x() == 0.0 && point.y() == 0.0 && point.z() == 0.0);
}

#[test]
fn can_create_a_point() {
    let point = Point::new(1.0, 2.0, 3.0);
    assert!(point.x() == 1.0 && point.y() == 2.0 && point.z() == 3.0);
}

#[test]
fn can_compare_point() {
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(1.0, 2.0, 3.0);
    assert_eq!(point1, point2);
    let point1 = Point::new(0.0, 2.0, 3.0);
    assert_ne!(point1, point2);
}

#[test]
fn can_add_point() {
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(3.0, 2.0, 1.0);
    assert_eq!(point1 + point2, Point::new(4.0, 4.0, 4.0));
}

#[test]
fn can_add_assign_point() {
    let mut point1 = Point::new(1.0, 2.0, 3.0);
    point1 += Point::new(3.0, 2.0, 1.0);
    assert_eq!(point1, Point::new(4.0, 4.0, 4.0));
}

#[test]
fn can_mul_scalar() {
    assert_eq!(
        4.0 * Point::new(2.0, 2.0, 2.0) * 3.0,
        Point::new(24.0, 24.0, 24.0)
    );
}

#[test]
fn can_mul_assign_scalar() {
    let mut point = Point::new(2.0, 2.0, 2.0);
    point *= 3.0;
    assert_eq!(point, Point::new(6.0, 6.0, 6.0));
}

#[test]
fn can_div_scalar() {
    match Point::new(6.0, 6.0, 6.0) / 2.0 {
        Some(point) => assert_eq!(point, Point::new(3.0, 3.0, 3.0)),
        None => assert!(false),
    };

    assert!(matches!(Point::new(1.0, 1.0, 1.0) / 0.0, None));
}

#[test]
fn can_negate_point() {
    assert_eq!(-Point::new(1.0, 2.0, 3.0), Point::new(-1.0, -2.0, -3.0));
}

#[test]
fn can_mutate_coordinates() {
    let mut point = Point::default();
    let x = point.x_mut();
    *x = 1.0;
    let y = point.y_mut();
    *y = 1.0;
    let z = point.z_mut();
    *z = 1.0;
    assert_eq!(point, Point::new(1.0, 1.0, 1.0));
}

#[test]
fn can_calculate_length() {
    assert_eq!(Point::new(2.0, 2.0, 2.0).len_squared(), 12.0);
}

#[test]
fn test_dot() {
    assert_eq!(
        dot(&Point::new(1.0, 1.0, 1.0), &Point::new(1.0, 1.0, 1.0)),
        3.0
    );
}

#[test]
fn test_cross() {
    assert_eq!(
        cross(&Point::new(1.0, 1.0, 1.0), &Point::new(1.0, 1.0, 1.0)),
        Point::default()
    );
}
