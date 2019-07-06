
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

/// Background linearly interpolates vertically from blue to white.
fn colour(ray: Ray, world: &[Box<dyn Hittable>]) -> Vec3 {
    if let Some(h) = hit(world, ray, 0.0, std::f32::MAX) {
        return 0.5 * Vec3::new(
            h.normal.x() + 1.0,
            h.normal.y() + 1.0,
            h.normal.z() + 1.0
        );
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    (1.0 - t) * white + t * blue
}

fn render(world: &[Box<dyn Hittable>], camera: &Camera) -> Image {
    let (width, height) = (200, 100);
    let mut image = Image::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = (height - y - 1) as f32 / height as f32;
            let r = camera.get_ray(u, v);
            let c = colour(r, world);
            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            image.set(x, y, [ir, ig, ib]);
        }
    }
    image
}

fn world() -> Vec<Box<dyn Hittable>> {
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    vec![Box::new(s1), Box::new(s2)]
}

fn main() {
    let world = world();
    let camera = Camera::new();
    let image = render(&world, &camera);
    image.save_to_ppm("hello_world.ppm").unwrap();
}
