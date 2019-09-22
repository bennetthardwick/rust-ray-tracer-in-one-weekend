use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f32::consts::PI;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let mut p;

    loop {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        p = 2. * vec3!(x, y, 0.) - vec3!(1.,1.,0.);

        if p.dot(&p) < 1. {
            break;
        }
    }

    p
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, 
        aspect: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {

        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (&origin - &look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = 
            &origin
            - half_width * focus_dist * &u
            - half_height * focus_dist  * &v
            - focus_dist * &w;

        let horizontal = 2. * half_width * focus_dist * &u;
        let vertical = 2. * half_height * focus_dist * &v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            lens_radius: aperture / 2.
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - offset,
        )
    }
}
