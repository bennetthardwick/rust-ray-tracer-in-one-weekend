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
        vfov: f32, 
        aspect: f32
    ) -> Camera {

        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        Camera {
            lower_left_corner: vec3!(-half_width, -half_height, -1.),
            horizontal: vec3!(2. * half_width, 0., 0.),
            vertical: vec3!(0., 2. * half_height, 0.),
            origin: vec3!(0.),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
