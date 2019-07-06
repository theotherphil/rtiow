
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

struct Hit<'a> {
    // This hit is the intersection of an object
    // with a ray of the form A + t * B. This is the
    // t in that expression.
    //
    // It's only used in the hit function taking world as
    // its first argument, where it's used to determine which
    // object hit is closest to the origin of the input ray.
    t: f32,
    // The coordinates of the intersection point.
    p: Vec3,
    // The normal to the object at the point of the intersection.
    normal: Vec3,
    material: &'a dyn Material
}

// This design is a bit wonky - would probably be neater to say that
// an object contains a Shape and a Material, both of which are enums

trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

trait Material {
    /// Returns (attenuation, scattered ray)
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    fn new(r: f32, g: f32, b: f32) -> Lambertian {
        Lambertian { albedo: Vec3::new(r, g, b) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_point_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((self.albedo, scattered))
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}

struct Metal {
    albedo: Vec3
}

impl Metal {
    fn new(r: f32, g: f32, b: f32) -> Metal {
        Metal { albedo: Vec3::new(r, g, b) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = reflect(unit_vector(ray.direction()), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        if dot(scattered.direction(), hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
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

fn colour(ray: Ray, world: &[Box<dyn Hittable>], depth: usize) -> Vec3 {
    if let Some(h) = hit(world, ray, 0.001, std::f32::MAX) {
        if depth >= 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let Some(p) = h.material.scatter(ray, &h) {
            return p.0 * colour(p.1, world, depth + 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
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
                c += colour(r, world, 0);
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
    let s1 = Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Box::new(Lambertian::new(1.0, 0.5, 0.5))
    );
    let s2 = Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Box::new(Lambertian::new(0.5, 0.5, 1.0))
    );
    let s3 = Sphere::new(
        Vec3::new(3.0 * r, 0.0, -1.0),
        r,
        Box::new(Lambertian::new(0.5, 1.0, 0.5))
    );
    let s4 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Metal::new(0.7, 0.7, 0.7))
    );
    vec![Box::new(s1), Box::new(s2), Box::new(s3), Box::new(s4)]
}

fn main() {
    let (width, height, num_samples) = (200, 100, 100);
    let world = world();
    let look_from = Vec3::new(0.0, 0.2, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 1.0;
    let aperture = 0.01;
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        120.0,
        width as f32 / height as f32,
        aperture,
        focus_distance
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
