use std::rc::Rc;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use hittable::{Hittable, HittableVector};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}   ", j);
        for i in 0..image_width {
            let mut color = Color::BLACK;

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utils::rand()) / (image_width - 1) as f64;
                let v = (j as f64 + utils::rand()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(ray, &world, max_depth);
            }
            print_color(color, samples_per_pixel);
        }
    }

    eprintln!();
    eprintln!("Done!");
}

fn ray_color(ray: Ray, world: &impl Hittable, depth: u16) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return scatter.attentuation * ray_color(scatter.scattered, world, depth - 1);
        }
        return Color::BLACK;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::WHITE + (t) * Color::new(0.5, 0.7, 1.0)
}

fn print_color(color: Color, samples_per_pixel: u16) {
    let (mut r, mut g, mut b) = color.rgb();

    let scale = 1. / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Write each color component after translating to [0,255] range
    let r = (256. * r.clamp(0., 0.999)) as u16;
    let g = (256. * g.clamp(0., 0.999)) as u16;
    let b = (256. * b.clamp(0., 0.999)) as u16;
    println!("{} {} {}", r, g, b);
}

fn material(material: impl Material + 'static) -> Rc<dyn Material> {
    Rc::new(material)
}

fn sphere((a, b, c): (f64, f64, f64), radius: f64, material: &Rc<dyn Material>) -> Box<Sphere> {
    let point = Point3::new(a, b, c);
    let material = Rc::clone(material);
    Box::new(Sphere::new(point, radius, material))
}

fn random_scene() -> HittableVector {
    let mut world = HittableVector::new();

    let ground_material = material(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(sphere((0., -1000., 0.), 1000., &ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::rand();
            let center = Point3::new(
                a as f64 + 0.9 * utils::rand(),
                0.2,
                b as f64 + 0.9 * utils::rand(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                // diffuse
                let sphere_material = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    material(Lambertian::new(albedo))
                }
                // metal
                else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = utils::rand_range(0., 0.5);
                    material(Metal::new(albedo, fuzz))
                }
                // glass
                else {
                    material(Dielectric::new(1.5))
                };

                world.push(sphere(center.xyz(), 0.2, &sphere_material));
            }
        }
    }

    let material1 = material(Dielectric::new(1.5));
    world.push(sphere((0., 1., 0.), 1.0, &material1));

    let material2 = material(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(sphere((-4., 1., 0.), 1.0, &material2));

    let material3 = material(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(sphere((4., 1., 0.), 1.0, &material3));

    world
}

fn test_scene() -> HittableVector {
    let mut world = HittableVector::new();

    let material_ground = material(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = material(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = material(Dielectric::new(1.5));
    let material_right = material(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.push(sphere((0., -100.5, -1.), 100., &material_ground));
    world.push(sphere((0., 0., -1.), 0.5, &material_center));
    world.push(sphere((-1., 0., -1.), 0.5, &material_left));
    world.push(sphere((-1., 0., -1.), -0.45, &material_left));
    world.push(sphere((1., 0., -1.), 0.5, &material_right));

    world
}
