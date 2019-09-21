use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
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
                t, p, normal, material: Rc::clone(&self.material)
            });
        }

        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list: vec![]
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
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
