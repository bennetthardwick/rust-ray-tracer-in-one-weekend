use rand::rngs::ThreadRng;
use rand::Rng;

use crate::ray::Ray;
use crate::objects::HitRecord;
use crate::vec3::Vec3;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p;
    loop {
        p = 2. * vec3!(rng.gen(), rng.gen(), rng.gen()) - vec3!(1.);
        if p.squared_length() < 1. {
            break;
        }
    }

    p
}


pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng) -> Option<(
        Vec3,
        Ray
    )>;
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2. * v.dot(n) * n;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }

    pub fn default() -> Lambertian {
        Lambertian::new(vec3!(0.5))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng) -> Option<(
        Vec3,
        Ray
    )> {

        let target = record.p + record.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(record.p, target - record.p);
        let attenuation = self.albedo;

            Some(( attenuation, scattered ))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if f < 1. && f >= 0. { f } else { 1. }
        }
    }

    pub fn default() -> Metal {
        Metal::new(vec3!(0.5), 0.)
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng) -> Option<(
        Vec3,
        Ray
    )> {

        let reflected = reflect(&ray_in.direction().unit_vector(), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        let attenuation = self.albedo;

        if scattered.direction().dot(&record.normal) > 0. {
            Some(( attenuation, scattered ))
        } else {
            None
        }


    }
}
