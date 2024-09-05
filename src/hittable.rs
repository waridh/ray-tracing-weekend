use crate::{material::Material, ray, vec3};
use std::ops::Range;

pub struct HitRecord<'a> {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: vec3::Point3,
        normal: vec3::Vec3,
        t: f32,
        r: &ray::Ray,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.direction.dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &ray::Ray, ray_interval: &Range<f32>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push<H: Hittable + 'static>(&mut self, value: H) {
        // let rc_value: Rc<dyn Hittable> = rc::Rc::clone(value);
        let boxed_val: Box<dyn Hittable> = Box::new(value);
        self.objects.push(boxed_val);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    /// If the ray, r, hits a hittable, then return the hit record for the
    /// closest hittable
    fn hit(&self, r: &ray::Ray, ray_interval: &Range<f32>) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_interval.clone();
        for hit_obj_rc in self.objects.iter() {
            let hit_obj = hit_obj_rc.as_ref();
            hit_record = match hit_obj.hit(r, &closest_so_far) {
                Some(x) if x.t < closest_so_far.end => {
                    closest_so_far.end = x.t;
                    Some(x)
                }
                _ => continue,
            }
        }
        hit_record
    }
}

#[cfg(test)]
mod test {
    use crate::material::Lambertian;
    use ray::Ray;

    use super::*;
    use crate::sphere::Sphere;

    fn create_sphere(x: f32, y: f32, z: f32, r: f32) -> Sphere<Lambertian> {
        Sphere::from((x, y, z, r))
    }

    /// Ensures that creation of the data structure does not fail
    #[test]
    fn create_list() {
        let mut list = HittableList::default();
        let item_1 = create_sphere(0., 1., 2., 50.);
        let item_2 = create_sphere(100., 1., 2., 1.);

        list.push(item_1);
        list.push(item_2);

        assert_eq!(list.len(), 2);
    }

    /// We must be able to clear the data
    #[test]
    fn clear_list() {
        let mut list = HittableList::default();
        let item_1 = create_sphere(0., 1., 2., 50.);
        let item_2 = create_sphere(100., 1., 2., 1.);

        list.push(item_1);
        list.push(item_2);

        assert_eq!(list.len(), 2);

        list.clear();
        assert_eq!(list.len(), 0);
    }

    /// We must be able to clear the data
    #[test]
    fn hit_one_thing() {
        let mut list = HittableList::default();
        let item_1 = create_sphere(0., 1., 2., 0.25);
        let item_2 = create_sphere(100., 1., 2., 1.);

        let origin = vec3::Vec3(0., 0., 0.);

        let r = Ray::new(vec3::Vec3(0., 0.5, 1.), origin);

        list.push(item_1);
        list.push(item_2);

        let hit_record = list.hit(&r, &(0f32..3f32));
        assert!(hit_record.is_some());
    }
}
