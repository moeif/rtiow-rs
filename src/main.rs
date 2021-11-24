#![allow(dead_code)]
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use color::Color;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    if let Some(hit_record) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + Color::one());
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image config
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera config
    let cam = Camera::new();

    // Render
    let mut rng = rand::thread_rng();
    let mut image_file_string = String::new();
    image_file_string.push_str(&format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u = (i as f64 + u_rand) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + v_rand) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }

            image_file_string.push_str(&format!(
                "{}",
                color::get_color_string(pixel_color, SAMPLES_PER_PIXEL)
            ));
        }
    }

    println!("{}", image_file_string);
}
