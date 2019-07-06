
use rand::prelude::*;
use std::f32::consts::PI;

mod camera;
use camera::*;
mod image;
use image::*;
mod ray;
use ray::*;
mod sphere;
use sphere::*;
mod vec3;
use vec3::*;

struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3
}

trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

fn hit(world: &[Box<dyn Hittable>], ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
    let mut best = None;
    let mut closest = t_max;

    for s in world {
        if let Some(h) = s.hit(ray, t_min, closest) {
            closest = h.t;
            best = Some(h);
        }
    }

    best
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let p = 2.0 * v - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() <= 1.0 {
            return p;
        }
    }
}

fn colour(ray: Ray, world: &[Box<dyn Hittable>]) -> Vec3 {
    if let Some(h) = hit(world, ray, 0.001, std::f32::MAX) {
        let target = h.p + h.normal + random_point_in_unit_sphere();
        return 0.5 * colour(Ray::new(h.p, target - h.p), world);
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Background linearly interpolates vertically from blue to white.
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    (1.0 - t) * white + t * blue
}

fn render(
    width: usize,
    height: usize,
    num_samples: usize,
    world: &[Box<dyn Hittable>],
    camera: &Camera
) -> Image {
    let mut image = Image::new(width, height);
    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut c = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..num_samples {
                let du: f32 = rng.gen();
                let dv: f32 = rng.gen();

                let u = (x as f32 + du) / width as f32;
                let v = ((height - y - 1) as f32 + dv) / height as f32;

                let r = camera.get_ray(u, v);
                c += colour(r, world);
            }

            c /= num_samples as f32;

            // rough gamma correction
            c = Vec3::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            image.set(x, y, [ir, ig, ib]);
        }
    }
    image
}

fn world() -> Vec<Box<dyn Hittable>> {
    let r = PI / 4.0;
    let s1 = Sphere::new(Vec3::new(-r, 0.0, -1.0), r);
    let s2 = Sphere::new(Vec3::new( r, 0.0, -1.0), r);
    vec![Box::new(s1), Box::new(s2)]
}

fn main() {
    let (width, height, num_samples) = (200, 100, 100);
    let world = world();
    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        width as f32 / height as f32
    );
    let image = render(
        width,
        height,
        num_samples,
        &world,
        &camera
    );
    image.save_to_ppm("hello_world.ppm").unwrap();
}
