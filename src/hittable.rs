use crate::{
    color::Color,
    material::{Lambertian, Material},
    ray, vec3,
};
use std::{
    ops::Range,
    rc::{self, Rc},
};

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: rc::Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: vec3::Point3,
        normal: vec3::Vec3,
        t: f32,
        r: &ray::Ray,
        material: &rc::Rc<dyn Material>,
    ) -> Self {
        let front_face = r.direction.dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        let material = rc::Rc::clone(material);
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_interval: &Range<f32>) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<rc::Rc<dyn Hittable + 'static>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn push(&mut self, value: &rc::Rc<dyn Hittable>) {
        let rc_value: Rc<dyn Hittable> = rc::Rc::clone(value);
        self.objects.push(rc_value);
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
    fn hit(&self, r: &ray::Ray, ray_interval: &Range<f32>) -> Option<HitRecord> {
        let default_mat: Rc<dyn Material> = Rc::new(Lambertian::new(Color::black()));
        let mut hit_record: HitRecord = HitRecord::new(
            vec3::Vec3(0., 0., 0.),
            vec3::Vec3(0., 0., 0.),
            0.,
            r,
            &default_mat,
        );
        let mut closest_so_far = ray_interval.clone();
        let mut hit_something = false;
        for hit_obj_rc in self.objects.iter() {
            let hit_obj = hit_obj_rc.as_ref();
            match hit_obj.hit(r, &closest_so_far) {
                None => continue,
                Some(x) => {
                    hit_something = true;
                    if x.t < closest_so_far.end {
                        closest_so_far.end = x.t;
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
        let item_1: Rc<dyn Hittable> = create_rc_sphere(0., 1., 2., 50.);
        let item_2: Rc<dyn Hittable> = create_rc_sphere(100., 1., 2., 1.);

        list.push(&item_1);
        list.push(&item_2);

        assert_eq!(list.len(), 2);
    }

    /// We must be able to clear the data
    #[test]
    fn clear_list() {
        let mut list = HittableList::new();
        let item_1: Rc<dyn Hittable> = create_rc_sphere(0., 1., 2., 50.);
        let item_2: Rc<dyn Hittable> = create_rc_sphere(100., 1., 2., 1.);

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
        let item_1: Rc<dyn Hittable> = create_rc_sphere(0., 1., 2., 0.25);
        let item_2: Rc<dyn Hittable> = create_rc_sphere(100., 1., 2., 1.);

        let origin = vec3::Vec3(0., 0., 0.);

        let r = Ray::new(vec3::Vec3(0., 0.5, 1.), origin);

        list.push(&item_1);
        list.push(&item_2);

        let hit_record = list.hit(&r, &(0f32..3f32));
        assert!(hit_record.is_some());
    }
}
