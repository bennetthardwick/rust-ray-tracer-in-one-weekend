use rand::rngs::ThreadRng;
use rand::Rng;

use crate::objects::HitRecord;
use crate::ray::Ray;
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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2. * v.dot(n) * n;
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);

    if discriminant > 0. {
        Some(ni_over_nt * (&uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng)
        -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let target = record.p + record.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(record.p, target - record.p);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if f < 1. && f >= 0. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction().unit_vector(), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        let attenuation = self.albedo;

        if scattered.direction().dot(&record.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let outward_normal;
        let reflected = reflect(ray_in.direction(), &record.normal);
        let ni_over_nt;

        let attenuation = vec3!(1.);

        let scattered;
        let cosine;

        if ray_in.direction().dot(&record.normal) > 0. {
            outward_normal = -&record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction().dot(&record.normal) / ray_in.direction().length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1. / self.ref_idx;
            cosine = -(ray_in.direction().dot(&record.normal)) / ray_in.direction().length();
        }

        if let Some(refracted) = refract(ray_in.direction(), &outward_normal, ni_over_nt) {
            let reflect_prob: f32 = rng.gen();
            if reflect_prob < schlick(cosine, self.ref_idx) {
                scattered = Ray::new(record.p, reflected);
            } else {
                scattered = Ray::new(record.p, refracted);
            }
        } else {
            scattered = Ray::new(record.p, reflected);
        }

        Some((attenuation, scattered))
    }
}
