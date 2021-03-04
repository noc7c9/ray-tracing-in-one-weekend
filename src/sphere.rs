use std::rc::Rc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        #[allow(clippy::suspicious_operation_groupings)]
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord::new(
            ray,
            root,
            point,
            (point - self.center) / self.radius,
            Rc::clone(&self.material),
        ))
    }
}
