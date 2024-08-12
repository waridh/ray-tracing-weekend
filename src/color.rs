use crate::vec3;
use std::{fmt, ops};

/// The Color newtype is a vec3 that represents a color.
/// Invariant:
///     The fields of this struct must be a float between 0 and 1.
pub struct Color(vec3::Vec3);

impl Color {
    pub fn from_args(a: f32, b: f32, c: f32) -> Self {
        Color(vec3::Vec3(a, b, c))
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

impl fmt::Display for Color {
    /// Color has a different byte representation in ppm than vec3.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.0[0];
        let g = self.0[1];
        let b = self.0[2];

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl ops::Index<usize> for Color {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Mul<f32> for Color {
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

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        let new_val = (self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]);
        Color::from(new_val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_print_struct() {
        let input = Color::from_args(0.5, 0.25, 0.125);

        let expected = format!("{} {} {}", 127, 63, 31);
        assert_eq!(expected, format!("{}", input));
    }

    #[test]
    fn color_usize_index() {
        let input = Color::from_args(0.5, 0.25, 0.125);

        assert_eq!(0.5, input[0]);
        assert_eq!(0.25, input[1]);
        assert_eq!(0.125, input[2]);
    }
}
