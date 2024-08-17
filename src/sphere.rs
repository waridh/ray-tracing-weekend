use crate::{hittable, vec3};
pub struct Sphere {
    center: vec3::Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl From<(f32, f32, f32, f32)> for Sphere {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        let (x, y, z, r) = value;
        Sphere::new(vec3::Vec3(x, y, z), r)
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f32,
        ray_tmax: f32,
    ) -> Option<hittable::HitRecord> {
        let oc = self.center - r.origin.as_ref();
        let a = r.direction.dot(&r.direction);
        let b = r.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0. {
            return None;
        }

        let rootd = discriminant.sqrt();
        let t = (b - rootd) / (a);
        let t = if t >= ray_tmax && t < ray_tmin {
            let t = (b + rootd) / (a);
            if t < ray_tmax && t >= ray_tmin {
                t
            } else {
                return None;
            }
        } else {
            t
        };
        let p = r.at(t);
        Some(hittable::HitRecord::new(
            p,
            (p - self.center) / self.radius,
            t,
            r,
        ))
    }
}

#[cfg(test)]
mod test {
    use hittable::Hittable;

    use super::*;
    use crate::ray::Ray;
    use std::rc::Rc;

    #[test]
    fn test_basic_hit() {
        let origin = Rc::new(vec3::Vec3(0., 0., 0.));
        let ray = Ray::new(vec3::Vec3(1., 1., 1.), &origin);

        let sphere = Sphere::from((5., 5., 5., 0.5));

        assert!(sphere.hit(&ray, 0., 100.).is_some());
    }
}
