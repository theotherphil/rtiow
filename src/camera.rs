use rand::prelude::*;
use std::f32::consts::PI;
use crate::ray::*;
use crate::vec3::*;

fn random_point_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0 {
            return p;
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(view_up, w));
        let v = cross(w, u);
        let lower_left_corner = look_from
              - half_width * focus_distance * u
              - half_height * focus_distance * v
              - w * focus_distance;

        Camera {
            origin: look_from,
            lower_left_corner: lower_left_corner,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_point_in_unit_disk();
        let offset = self.u * rd.x()+ self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical
                - self.origin
                - offset
        )
    }
}
