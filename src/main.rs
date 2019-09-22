#[macro_use]
mod vec3;
mod camera;
mod material;
mod objects;
mod ray;

use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use objects::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

const MAX_DEPTH: i32 = 50;

fn color(ray: Ray, world: &Box<dyn Hittable>, rng: &mut ThreadRng) -> Vec3 {
    let mut ray = ray;
    let mut mult: Vec3 = vec3!(1.);
    let mut depth = 0;

    loop {
        if depth > MAX_DEPTH {
            return vec3!(0.);
        }

        if let Some(record) = world.hit(&ray, 0.001, std::f32::MAX) {
            if let Some((attenuation, scattered)) = record.material.scatter(&ray, &record, rng) {
                ray = scattered;
                mult *= &attenuation;
            } else {
                return vec3!(0.);
            }
        } else {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.);
            return mult * ((1. - t) * vec3!(1.) + t * vec3!(0.5, 0.7, 1.0));
        }

        depth += 1;
    }
}

fn main() {
    let width: i16 = 200;
    let height: i16 = 100;
    let samples: i16 = 100;

    let mut rng = ThreadRng::default();

    let camera = Camera::new(
        vec3!(-2., 2., 1.),
        vec3!(0., 0., -1.),
        vec3!(0., 1., 0.),
        90.,
        f32::from(width) / f32::from(height),
    );

    print!("P3\n{} {}\n255\n", width, height);

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        vec3!(0., 0., -1.),
        0.5,
        Rc::new(Lambertian::new(vec3!(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        vec3!(0., -100.5, -1.),
        100.,
        Rc::new(Lambertian::new(vec3!(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        vec3!(1., 0., -1.),
        0.5,
        Rc::new(Metal::new(vec3!(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        vec3!(-1., 0., -1.),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        vec3!(-1., 0., -1.),
        -0.45,
        Rc::new(Dielectric::new(1.5)),
    )));

    let world: Box<dyn Hittable> = Box::new(world);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = vec3!(0.);

            for _ in 0..samples {
                let a: f32 = rng.gen();
                let b: f32 = rng.gen();

                let u = (f32::from(i) + a) / f32::from(width);
                let v = (f32::from(j) + b) / f32::from(height);

                let ray = camera.get_ray(u, v);
                col += color(ray, &world, &mut rng);
            }

            col /= f32::from(samples);
            col = col.map(|x| x.sqrt());

            let ir = (255.99 * col.r()).floor() as i32;
            let ig = (255.99 * col.g()).floor() as i32;
            let ib = (255.99 * col.b()).floor() as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
