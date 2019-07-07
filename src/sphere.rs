
use crate::{Hit, Hittable};
use crate::vec3::*;
use crate::ray::*;
use crate::Material;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere { centre, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let intersection = sphere_intersection(
                self.centre, self.radius, ray, t_min, t_max
        );
        intersection.and_then(|(t, p)|
            Some(Hit {
                t,
                p,
                normal: (p - self.centre) / self.radius,
                material: &*self.material
            })
        )
    }
}

/// Finds the intersection between a sphere and a ray which is
/// closest to the ray's origin, if one exists.
///
/// Returns (t, p)
fn sphere_intersection(
    centre: Vec3, radius: f32, ray: Ray, t_min: f32, t_max: f32
) -> Option<(f32, Vec3)> {
    let oc = ray.origin() - centre;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(oc, ray.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < t_max && t > t_min {
            let p = ray.point_at_parameter(t);
            return Some((t, p));
        }

        let t = (-b + discriminant.sqrt()) / (2.0 * a);
        if t < t_max && t > t_min {
            let p = ray.point_at_parameter(t);
            return Some((t, p));
        }
    }

    None
}

mod tests {
    use super::*;

    #[test]
    fn test_sphere_intersection() {
        let origin = Vec3::new(0.0, 0.0, 1.0);
        let centre = Vec3::new(3.0, 0.0, -1.0);
        let radius = 1.0;
        let t_min = 0.0;
        let t_max = 100.0;

        // Beyond left of sphere
        let direction = Vec3::new(0.5, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(
            sphere_intersection(centre, radius, ray, t_min, t_max),
            None
        );

        // Leftmost point of sphere
        let direction = Vec3::new(2.0, 0.0, -2.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(
            sphere_intersection(centre, radius, ray, t_min, t_max),
            Some((1.0, Vec3::new(2.0, 0.0, -1.0)))
        );

        // Centre of sphere
        let direction = Vec3::new(3.0, 0.0, -2.0);
        let ray = Ray::new(origin, direction);
        // Defined actual expected result here, but needed approx_eq and
        // didn't bother adding/implementing it.
        assert!(sphere_intersection(centre, radius, ray, t_min, t_max).is_some());

        // Rightmost point of sphere
        let direction = Vec3::new(3.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(
            sphere_intersection(centre, radius, ray, t_min, t_max),
            Some((1.0, Vec3::new(3.0, 0.0, 0.0)))
        );

        // Beyond right of sphere
        let direction = Vec3::new(4.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(
            sphere_intersection(centre, radius, ray, t_min, t_max),
            None
        );
    }
}
