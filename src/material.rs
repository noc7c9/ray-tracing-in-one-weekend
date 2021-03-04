use super::hittable::HitRecord;
use super::ray::Ray;
use super::utils;
use super::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub attentuation: Color,
    pub scattered: Ray,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some(ScatterResult {
            scattered: Ray::new(hit.point, scatter_direction),
            attentuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vector().reflect(hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction.dot(hit.normal) > 0. {
            Some(ScatterResult {
                scattered,
                attentuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if hit.front_face {
            1. / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > utils::rand() {
                unit_direction.reflect(hit.normal)
            } else {
                unit_direction.refract(hit.normal, refraction_ratio)
            };

        Some(ScatterResult {
            scattered: Ray::new(hit.point, direction),
            attentuation: Color::WHITE,
        })
    }
}
