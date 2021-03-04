use std::rc::Rc;

use super::material::Material;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        t: f64,
        point: Point3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableVector(Vec<Box<dyn Hittable>>);

impl HittableVector {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl Hittable for HittableVector {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for object in self.0.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
