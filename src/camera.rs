use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, 
        aspect: f32
    ) -> Camera {

        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (&origin - &look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: &origin - half_width * &u - half_height * &v - &w,
            horizontal: 2. * half_width * &u,
            vertical: 2. * half_height * &v,
            origin
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin,
        )
    }
}
