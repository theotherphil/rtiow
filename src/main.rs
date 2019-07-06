
mod image;
use image::*;
mod vec3;
use vec3::*;
mod ray;
use ray::*;

fn lower_left_corner() -> Vec3 {
    Vec3::new(-2.0, -1.0, -1.0)
}

fn horizontal() -> Vec3 {
    Vec3::new(4.0, 0.0, 0.0)
}

fn vertical() -> Vec3 {
    Vec3::new(0.0, 2.0, 0.0)
}

fn origin() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// Linearly interpolates vertically from blue to white.
fn colour(ray: Ray) -> Vec3 {
    if hits_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let direction = unit_vector(ray.direction());
    let t = 0.5 * (direction.y() + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    (1.0 - t) * white + t * blue
}

fn hits_sphere(centre: Vec3, radius: f32, ray: Ray) -> bool {
    let oc = ray.origin() - centre;
    let d = ray.direction();
    let a = dot(d, d);
    let b = 2.0 * dot(oc, d);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn hello_world_image() -> Image {
    let (width, height) = (200, 100);
    let mut image = Image::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = (height - y - 1) as f32 / height as f32;
            let r = Ray::new(
                origin(),
                lower_left_corner() + u * horizontal() + v * vertical()
            );
            let c = colour(r);
            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            image.set(x, y, [ir, ig, ib]);
        }
    }
    image
}

fn main() {
    let image = hello_world_image();
    image.save_to_ppm("hello_world.ppm").unwrap();
}
