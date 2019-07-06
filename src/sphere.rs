
use crate::{Hit, Hittable};
use crate::vec3::*;
use crate::ray::*;

pub struct Sphere {
    centre: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin() - self.centre;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(oc, ray.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(Hit {
                    t,
                    p,
                    normal: (p - self.centre) / self.radius
                });
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(Hit {
                    t,
                    p,
                    normal: (p - self.centre) / self.radius
                });
            }
        }
        None
    }
}
