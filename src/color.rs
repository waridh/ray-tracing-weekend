use crate::vec3;
use std::{fmt, iter, ops};

pub fn linear_to_gamma(linear: f32) -> f32 {
    match linear {
        x if x > 0. => x.sqrt(),
        _ => 0.,
    }
}
/// The Color newtype is a vec3 that represents a color.
/// Invariant:
///     The fields of this struct must be a float between 0 and 1.
#[derive(Debug, PartialEq, Clone)]
pub struct Color(vec3::Vec3);

impl Color {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Color(vec3::Vec3(a, b, c))
    }

    pub fn black() -> Self {
        Color::new(0., 0., 0.)
    }
}

impl From<vec3::Vec3> for Color {
    fn from(value: vec3::Vec3) -> Self {
        Color(value)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Color(vec3::Vec3::from(value))
    }
}

impl iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0., 0., 0.), |a, e| a + e)
    }
}

/// Used to clamp the color output
fn clamp(legal_range: &ops::Range<f32>, val: f32) -> f32 {
    match val {
        x if x > legal_range.end => legal_range.end,
        x if x < legal_range.start => legal_range.start,
        x => x,
    }
}

impl fmt::Display for Color {
    /// Color has a different byte representation in ppm than vec3.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = linear_to_gamma(self.0[0]);
        let g = linear_to_gamma(self.0[1]);
        let b = linear_to_gamma(self.0[2]);

        let legal_range = 0f32..0.999;

        let ir = (255.999 * clamp(&legal_range, r)) as u8;
        let ig = (255.999 * clamp(&legal_range, g)) as u8;
        let ib = (255.999 * clamp(&legal_range, b)) as u8;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl ops::Index<usize> for Color {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        let new_val = (self[0] * rhs, self[1] * rhs, self[2] * rhs);
        Color::from(new_val)
    }
}

impl ops::Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        let new_val = (self[0] * rhs, self[1] * rhs, self[2] * rhs);
        Color::from(new_val)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<&Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::from(self.0 * rhs.0)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        let new_val = (self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]);
        Color::from(new_val)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        for i in 0..3 {
            self[i] += rhs[i]
        }
    }
}

/// Tested
impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}

/// Tested
impl ops::DivAssign<usize> for Color {
    fn div_assign(&mut self, rhs: usize) {
        self.0 /= rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_print_struct() {
        let input = Color::new(0.5, 0.25, 0.125);

        let expected = format!("{} {} {}", 181, 127, 90);
        assert_eq!(expected, format!("{}", input));
    }

    #[test]
    fn color_usize_index() {
        let input = Color::new(0.5, 0.25, 0.125);

        assert_eq!(0.5, input[0]);
        assert_eq!(0.25, input[1]);
        assert_eq!(0.125, input[2]);
    }

    #[test]
    fn color_assign_scalar_division() {
        let mut left = Color::new(3., 6., 9.);
        left /= 2.;
        let expected = Color::new(1.5, 3., 4.5);

        assert_eq!(left, expected);
    }

    #[test]
    fn color_assign_scalar_division_usize() {
        let mut left = Color::new(3., 6., 9.);
        left /= 2;
        let expected = Color::new(1.5, 3., 4.5);

        assert_eq!(left, expected);
    }
}
