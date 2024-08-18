use crate::vec3;

pub struct Ray {
    pub direction: vec3::Vec3,
    pub origin: vec3::Point3,
}

impl Ray {
    pub fn new(direction: vec3::Vec3, origin: vec3::Vec3) -> Self {
        Ray { direction, origin }
    }

    pub fn at(&self, t: f32) -> vec3::Point3 {
        self.origin + (t * self.direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construction_1() {
        let origin = vec3::Point3::from((0., 0., 0.));
        let direction = vec3::Vec3(1., 1., 1.);
        let arg = 50f32;
        let expected = (50f32, 50f32, 50f32);

        let input = Ray { direction, origin };

        assert_eq!(input.at(arg), vec3::Point3::from(expected));
    }
}
