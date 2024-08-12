use std::{fmt, ops};

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub type Point3 = Vec3;
impl Vec3 {
    pub fn new<A>(args: A) -> Self
    where
        A: Into<Vec3>,
    {
        args.into()
    }

    /// Tested
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    /// Tested
    pub fn squared_length(&self) -> f32 {
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
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
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

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
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
impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
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

#[cfg(test)]
mod tests {
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
        let input = Vec3::new(1);
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
    fn vec3_addition() {
        let left = Vec3(1., 2., 3.);
        let right = Vec3(4., 5., 6.);
        let res = left + right;
        let expected = Vec3(5., 7., 9.);

        assert_eq!(res, expected);

        assert_eq!(left, Vec3(1., 2., 3.));
    }

    #[test]
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
    fn squared_length() {
        let input = Vec3(3., 6., 9.);
        let expected = 126.;

        assert_eq!(input.squared_length(), expected);
    }

    #[test]
    fn vec3_length() {
        let input = Vec3(3., 6., 9.);
        let expected = 3. * (14f32.sqrt());

        assert_delta!(input.length(), expected, 0.000001);
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
        let res = input.unit_vector();
        let expected = Vec3(3. / scalar, 6. / scalar, 9. / scalar);

        assert_delta!(res.0, expected.0, 0.0000001);
        assert_delta!(res.1, expected.1, 0.0000001);
        assert_delta!(res.2, expected.2, 0.0000001);
        assert_eq!(res.length(), 1.);
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
}
