use crate::{ray, vec3};
use std::rc;

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, normal: vec3::Vec3, t: f32, r: &ray::Ray) -> Self {
        let front_face = r.direction.dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<rc::Rc<dyn Hittable + 'static>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn push<T>(&mut self, value: &rc::Rc<T>)
    where
        T: Hittable + 'static,
    {
        let rc_value = rc::Rc::clone(value);
        self.objects.push(rc_value);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut hit_record: HitRecord =
            HitRecord::new(vec3::Vec3(0., 0., 0.), vec3::Vec3(0., 0., 0.), 0., r);
        let mut closest_so_far = ray_tmax;
        let mut hit_something = false;
        for hit_obj_rc in self.objects.iter() {
            let hit_obj = hit_obj_rc.as_ref();
            match hit_obj.hit(r, ray_tmin, closest_so_far) {
                None => continue,
                Some(x) => {
                    hit_something = true;
                    if x.t < closest_so_far {
                        closest_so_far = x.t;
                        hit_record = x;
                    }
                }
            }
        }
        if !hit_something {
            None
        } else {
            Some(hit_record)
        }
    }
}

#[cfg(test)]
mod test {
    use ray::Ray;

    use super::*;
    use crate::sphere::Sphere;

    fn create_rc_sphere(x: f32, y: f32, z: f32, r: f32) -> rc::Rc<Sphere> {
        let sphere = Sphere::from((x, y, z, r));
        rc::Rc::new(sphere)
    }

    /// Ensures that creation of the data structure does not fail
    #[test]
    fn create_list() {
        let mut list = HittableList::new();
        let item_1 = create_rc_sphere(0., 1., 2., 50.);
        let item_2 = create_rc_sphere(100., 1., 2., 1.);

        list.push(&item_1);
        list.push(&item_2);

        assert_eq!(list.len(), 2);
    }

    /// We must be able to clear the data
    #[test]
    fn clear_list() {
        let mut list = HittableList::new();
        let item_1 = create_rc_sphere(0., 1., 2., 50.);
        let item_2 = create_rc_sphere(100., 1., 2., 1.);

        list.push(&item_1);
        list.push(&item_2);

        assert_eq!(list.len(), 2);

        list.clear();
        assert_eq!(list.len(), 0);
    }

    /// We must be able to clear the data
    #[test]
    fn hit_one_thing() {
        let mut list = HittableList::new();
        let item_1 = create_rc_sphere(0., 1., 2., 0.25);
        let item_2 = create_rc_sphere(100., 1., 2., 1.);

        let origin = rc::Rc::new(vec3::Vec3(0., 0., 0.));

        let r = Ray::new(vec3::Vec3(0., 0.5, 1.), &origin);

        list.push(&item_1);
        list.push(&item_2);

        let hit_record = list.hit(&r, 0., 3.);
        assert!(hit_record.is_some());
    }
}
