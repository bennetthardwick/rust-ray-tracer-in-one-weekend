use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = (&oc).dot(ray.direction());
        let c = (&oc).dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let d_square = discriminant.sqrt();
            let mut temp = (-b - d_square) / a;

            if temp > t_max || temp < t_min {
                temp = (-b + d_square) / a;
                if temp > t_max || temp < t_min {
                    return None;
                }
            }

            let t = temp;
            let p = ray.point_at_parameter(temp);
            let normal = (p - &self.center) / self.radius;

            return Some(HitRecord {
                t,
                p,
                normal,
                material: Rc::clone(&self.material),
            });
        }

        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

const CENTER: Vec3 = vec3!(4., 0.2, 0.);

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }

    pub fn random(rng: &mut ThreadRng) -> HittableList {
        let mut list = HittableList::new();

        list.add(Box::new(Sphere::new(
            vec3!(0., -1000., 0.),
            1000.,
            Rc::new(Lambertian::new(vec3!(0.5, 0.5, 0.5))),
        )));

        let end: i16 = 11;

        for a in -end..end {
            for b in -end..end {
                let choose_mat: f32 = rng.gen();

                let a_fl = f32::from(a);
                let b_fl = f32::from(b);

                let center = vec3!(
                    a_fl + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b_fl + 0.9 * rng.gen::<f32>()
                );

                if (&center - &CENTER).length() > 0.9 {
                    if choose_mat < 0.8 {
                        list.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(Lambertian::new(vec3!(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>()
                            ))),
                        )));
                    } else if choose_mat < 0.95 {
                        list.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(Metal::new(
                                vec3!(
                                    0.5 * (1. + rng.gen::<f32>()),
                                    0.5 * (1. + rng.gen::<f32>()),
                                    0.5 * (1. + rng.gen::<f32>())
                                ),
                                0.5 * rng.gen::<f32>(),
                            )),
                        )));
                    } else {
                        list.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }

        list.add(Box::new(Sphere::new(
            vec3!(0., 1., 0.),
            1.,
            Rc::new(Dielectric::new(1.5)),
        )));
        list.add(Box::new(Sphere::new(
            vec3!(-4., 1., 0.),
            1.,
            Rc::new(Lambertian::new(vec3!(0.4, 0.2, 0.1))),
        )));
        list.add(Box::new(Sphere::new(
            vec3!(4., 1., 0.),
            1.,
            Rc::new(Metal::new(vec3!(0.7, 0.6, 0.5), 0.)),
        )));

        list
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for item in self.list.iter() {
            if let Some(r) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = r.t;
                record = Some(r);
            }
        }

        return record;
    }
}
