use rand::{self, Rng};
use std::{fmt, ops};

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub type Point3 = Vec3;
impl Vec3 {
    pub fn new<A>(a: A, b: A, c: A) -> Self
    where
        A: Into<f32>,
    {
        let a: f32 = a.into();
        let b: f32 = b.into();
        let c: f32 = c.into();
        Vec3(a, b, c)
    }

    pub fn random_range(range: ops::Range<f32>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
        )
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(
            rng.gen_range(0f32..1f32),
            rng.gen_range(0f32..1f32),
            rng.gen_range(0f32..1f32),
        )
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            match Vec3::random_range(-1f32..1f32) {
                p if p.magnitude_squared() < 1. => return p,
                _ => continue,
            }
        }
    }

    fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            match Vec3::new(rng.gen_range(-1f32..1f32), rng.gen_range(-1f32..1f32), 0.) {
                p if p.magnitude_squared() < 1. => return p,
                _ => continue,
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if self.dot(&on_unit_sphere) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Tested
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Tested
    pub fn magnitude_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    /// Dot product with other vector
    /// Tested
    pub fn dot(&self, v: &Vec3) -> f32 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    /// Cross product with other vector
    /// Tested
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    /// Returns the unit vector corresponding to the current vector object
    /// Tested
    pub fn normalize(&self) -> Vec3 {
        self / self.magnitude()
    }

    /// Returns true if all axis of the vector is very close to zero, else
    /// return false
    pub fn near_zero(&self) -> bool {
        let v = 0.0000001f32;
        self.0 < v && self.1 < v && self.2 < v
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2. * self.dot(normal) * normal
    }

    /// This method does a naive snell's law operation on the input vectors.
    /// This method will not handle total internal reflection.
    ///
    /// Invariant:
    ///     - Assumes that input vectors are both unit vectors
    /// Tested
    pub fn refract(&self, normal: &Vec3, eta_ratio: f32) -> Vec3 {
        let cos_theta: f32 = (-self).dot(normal).min(1.);
        let perpendicular = eta_ratio * (self + (cos_theta * normal));
        let parallel = (-(1. - perpendicular.magnitude_squared()).abs().sqrt()) * normal;
        perpendicular + parallel
    }
}

impl From<usize> for Vec3 {
    fn from(value: usize) -> Self {
        let val = value as f32;
        Vec3(val, val, val)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        let (a, b, c) = value;
        Vec3(a, b, c)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

/// Tested
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(rhs.0 + self.0, rhs.1 + self.1, rhs.2 + self.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

/// Vector subtraction, done element-wise.
/// Tested
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// Tested
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// Tested
impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// Tested
impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

/// Implementing scalar multiplication
/// Tested
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

/// Implementing scalar multiplication
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

/// Implementing scalar multiplication
/// Tested
impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

/// Implementing scalar multiplication
impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

/// Implementing scalar multiplication
impl ops::Mul<Vec3> for usize {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let conv = self as f32;
        conv * rhs
    }
}

/// Implementing scalar multiplication
impl ops::Mul<usize> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: usize) -> Self::Output {
        rhs * self
    }
}

/// Implementing element wise multiplication
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

/// Implementing scalar division for vec3
/// Tested
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

/// Implementing scalar division for vec3
/// Tested
impl ops::Div<usize> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: usize) -> Self::Output {
        let conv = rhs as f32;
        self / conv
    }
}

/// Implementing scalar division for vec3
/// Tested
impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

/// Tested
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

/// Tested
impl ops::DivAssign<usize> for Vec3 {
    fn div_assign(&mut self, rhs: usize) {
        let converted = rhs as f32;
        self.0 /= converted;
        self.1 /= converted;
        self.2 /= converted;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::f32::consts::PI;

    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d && $y - $x < $d) {
                eprintln!("left: {}; right: {}", $x, $y);
                panic!();
            }
        };
    }

    #[test]
    fn construction() {
        let input = Vec3(1., 2., 3.);
        assert_eq!(input.0, 1.);
        assert_eq!(input.1, 2.);
        assert_eq!(input.2, 3.);
    }

    #[test]
    fn construction_from_single_usize() {
        let input = Vec3::from(1);
        assert_eq!(input.0, 1.);
        assert_eq!(input.1, 1.);
        assert_eq!(input.2, 1.);
    }

    #[test]
    fn indexing() {
        let input = Vec3(1., 2., 3.);
        assert_eq!(input[0], 1.);
        assert_eq!(input[1], 2.);
        assert_eq!(input[2], 3.);
    }

    #[test]
    fn vec3_near_zero() {
        let tests = [
            (Vec3(1., 1., 1.), false),
            (Vec3(0., 0., 0.), true),
            (Vec3(0.001, 0., 0.), false),
        ];

        for (input, expected) in tests {
            assert_eq!(input.near_zero(), expected);
        }
    }

    #[test]
    fn vec3_addition() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = left + right;
        let expected = Vec3(5., 7., 9.);

        assert_eq!(res, expected);

        assert_eq!(left, Vec3(1., 2., 3.));
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn vec3_left_ref_addition() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = &left + right;
        let expected = Vec3(5., 7., 9.);

        assert_eq!(res, expected);

        assert_eq!(left, Vec3(1., 2., 3.));
    }

    #[test]
    fn vec3_assign_add() {
        let mut left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        left += right;
        let expected = Vec3(5., 7., 9.);

        assert_eq!(left, expected);
    }

    #[test]
    fn sub_vec3() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = left - right;
        let expected = Vec3(-3., -3., -3.);

        assert_eq!(res, expected);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn sub_left_ref_vec3() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = &left - right;
        let expected = Vec3(-3., -3., -3.);

        assert_eq!(res, expected);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn sub_right_ref_vec3() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = left - &right;
        let expected = Vec3(-3., -3., -3.);

        assert_eq!(res, expected);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn sub_ref_vec3() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = &left - &right;
        let expected = Vec3(-3., -3., -3.);

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_assign_sub() {
        let mut left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        left -= right;
        let expected = Vec3(-3., -3., -3.);

        assert_eq!(left, expected);
    }

    #[test]
    fn vec3_scalar_multiplication() {
        let left = Vec3(1., 2., 3.);
        let res = left * 3.;
        let expected = Vec3(3., 6., 9.);

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_reversed_scalar_multiplication() {
        let left = Vec3(1., 2., 3.);
        let res = 3. * left;
        let expected = Vec3(3., 6., 9.);

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_scalar_assign_multiplication() {
        let mut left = Vec3(1., 2., 3.);
        left *= 3.;
        let expected = Vec3(3., 6., 9.);

        assert_eq!(left, expected);
    }

    #[test]
    fn vec3_scalar_division() {
        let left = Vec3(3., 6., 9.);
        let res = left / 2.;
        let expected = Vec3(1.5, 3., 4.5);

        assert_eq!(res, expected);
    }

    #[test]
    fn ref_vec3_scalar_division() {
        let left = Vec3(3., 6., 9.);
        let res = &left / 2.;
        let expected = Vec3(1.5, 3., 4.5);

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_assign_scalar_division() {
        let mut left = Vec3(3., 6., 9.);
        left /= 2.;
        let expected = Vec3(1.5, 3., 4.5);

        assert_eq!(left, expected);
    }

    #[test]
    fn vec3_assign_scalar_division_usize() {
        let mut left = Vec3(3., 6., 9.);
        left /= 2;
        let expected = Vec3(1.5, 3., 4.5);

        assert_eq!(left, expected);
    }

    #[test]
    fn squared_length() {
        let input = Vec3(3., 6., 9.);
        let expected = 126.;

        assert_eq!(input.magnitude_squared(), expected);
    }

    #[test]
    fn vec3_length() {
        let input = Vec3(3., 6., 9.);
        let expected = 3. * (14f32.sqrt());

        assert_delta!(input.magnitude(), expected, 0.000001);
    }

    #[test]
    fn vec3_dot() {
        let left = Vec3(3., 6., 9.);
        let right = Vec3(4., 5., 6.);
        let res = left.dot(&right);
        let expected = 3. * 4. + 6. * 5. + 9. * 6.;

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_cross() {
        let left = Vec3(3., 6., 9.);
        let right = Vec3(4., 5., 6.);
        let res = left.cross(&right);
        let expected = Vec3(-9., 18., -9.);

        assert_eq!(res, expected);
    }

    #[test]
    fn vec3_unit_vec() {
        let input = Vec3(3., 6., 9.);
        let scalar = 3. * 14f32.sqrt();
        let res = input.normalize();
        let expected = Vec3(3. / scalar, 6. / scalar, 9. / scalar);

        assert_delta!(res.0, expected.0, 0.0000001);
        assert_delta!(res.1, expected.1, 0.0000001);
        assert_delta!(res.2, expected.2, 0.0000001);
        assert_eq!(res.magnitude(), 1.);
    }

    #[test]
    fn vec3_string() {
        let input = Vec3(120., 66., 70.);
        let expected = "120 66 70";

        let output = format!("{}", input);
        assert_eq!(output.as_str(), expected);
    }

    #[test]
    fn vec3_string_2() {
        let input = Vec3(120.2, 66.3004, 70.4321);
        let expected = "120.2 66.3004 70.4321";

        let output = format!("{}", input);
        assert_eq!(output.as_str(), expected);
    }

    #[test]
    fn vec3_refract_basic() {
        let input = Vec3(1., -1., 0.).normalize();
        let normal = Vec3(0., 1., 0.);
        let eta = 1.;
        let eta_prime = 1.;
        let sin_theta = (eta / eta_prime) * (PI / 4.).sin();
        let cos_theta = (1. - (sin_theta * sin_theta)).sqrt();

        let expected = Vec3(sin_theta, -cos_theta, 0.);

        assert_eq!(input.refract(&normal, eta / eta_prime), expected);
    }

    #[test]
    fn vec3_refract_basic2() {
        let input = Vec3(-1., -1., 0.).normalize();
        let normal = Vec3(0., 1., 0.);
        let eta = 1.;
        let eta_prime = 1.;
        let sin_theta = (eta / eta_prime) * (PI / 4.).sin();
        let cos_theta = (1. - (sin_theta * sin_theta)).sqrt();

        let expected = Vec3(-sin_theta, -cos_theta, 0.);

        assert_eq!(input.refract(&normal, eta / eta_prime), expected);
    }

    #[test]
    fn vec3_refract_1_5() {
        let input = Vec3(1., -1., 0.).normalize();
        let normal = Vec3(0., 1., 0.);
        let eta = 1.;
        let eta_prime = 1.5;
        let sin_theta = (eta / eta_prime) * (PI / 4.).sin();
        let cos_theta = (1. - (sin_theta * sin_theta)).sqrt();

        let expected = Vec3(sin_theta, -cos_theta, 0.);

        assert_eq!(input.refract(&normal, eta / eta_prime), expected);
    }

    #[test]
    fn vec3_refract_0_67() {
        let input = Vec3(1., -1., 0.).normalize();
        let normal = Vec3(0., 1., 0.);
        let eta = 1.5;
        let eta_prime = 1.;
        let sin_theta = (eta / eta_prime) * (PI / 4.).sin();
        let cos_theta = (1. - (sin_theta * sin_theta)).abs().sqrt();

        let expected = Vec3(sin_theta, -cos_theta, 0.);

        assert_eq!(input.refract(&normal, eta / eta_prime), expected);
    }
}
