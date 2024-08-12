use crate::vec3;
use std::{ops::Deref, rc};

pub struct Ray {
    pub direction: vec3::Vec3,
    pub origin: rc::Rc<vec3::Point3>,
}

impl Ray {
    pub fn new(direction: vec3::Vec3, origin: rc::Rc<vec3::Vec3>) -> Self {
        Ray { direction, origin }
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f32) -> vec3::Point3 {
        self.origin.deref() + (t * self.direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construction_1() {
        let origin = rc::Rc::new(vec3::Point3::from((0., 0., 0.)));
        let direction = vec3::Vec3(1., 1., 1.);
        let arg = 50f32;
        let expected = (50f32, 50f32, 50f32);

        let input = Ray { direction, origin };

        assert_eq!(input.at(arg), vec3::Point3::from(expected));
    }
}
