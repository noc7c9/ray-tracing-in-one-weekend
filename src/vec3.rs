use super::utils;

pub type Point3 = Vec3;
pub type Color = Vec3;

/***
 * struct definition
 */

#[derive(Clone, Copy)]
pub struct Vec3(f64, f64, f64);

/***
 * methods
 */

impl Vec3 {
    pub const BLACK: Color = Vec3(0., 0., 0.);
    pub const WHITE: Color = Vec3(1., 1., 1.);

    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self(a, b, c)
    }

    pub fn random() -> Self {
        Self::new(utils::rand(), utils::rand(), utils::rand())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            utils::rand_range(min, max),
            utils::rand_range(min, max),
            utils::rand_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random_range(-1., 1.);
            if point.length_squared() < 1. {
                break point;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let point = Vec3::new(utils::rand_range(-1., 1.), utils::rand_range(-1., 1.), 0.);
            if point.length_squared() < 1. {
                break point;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let Vec3(a, b, c) = self;
        a * a + b * b + c * c
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        let Vec3(a, b, c) = self;
        let s = 1.0e-18;
        a.abs() < s && b.abs() < s && c.abs() < s
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2. * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

/***
 * operators
 */

// indexing

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }
}

// negation

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// addition

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

// subtraction

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other
    }
}

// multiplication

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other
    }
}

// division

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        self * (1. / other)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1. / other;
    }
}
