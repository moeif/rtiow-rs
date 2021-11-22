#![allow(dead_code)]
mod ray;
mod vec3;
use ray::Ray;
use vec3::{Color, Vec3};

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    // 公式中的 （A - C)
    let oc = r.origin - center;

    // 公式中第1项的 b*b
    // let a = Vec3::dot(r.direction, r.direction);
    let a = r.direction.length_squared();

    // 公式中第2项的内容，忽略 t
    // let b = 2.0 * Vec3::dot(oc, r.direction);
    let half_b = Vec3::dot(oc, r.direction);

    // 公式中的 (A - C) * (A - C) - r^2
    // let c = Vec3::dot(oc, oc) - radius * radius;
    let c = oc.length_squared() - radius * radius;

    // 计算出了 a, b, c，判断 b^2 - 4ac 解的个数
    // let result = b * b - 4.0 * a * c;
    let result = half_b * half_b - a * c;

    // -----------------------------------
    if result < 0.0 {
        return -1.0;
    } else {
        // return (-b - result.sqrt()) / (2.0 * a);
        return (-half_b - result.sqrt()) / a;
    }
    // -----------------------------------
}

fn ray_color(r: Ray) -> Color {
    // -----------------------------------
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let unit_normal = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5
            * Color::new(
                unit_normal.x + 1.0,
                unit_normal.y + 1.0,
                unit_normal.z + 1.0,
            );
    }
    // -----------------------------------

    // 将光线的方向标准化，保证其值在 -1 到 1 之间
    let unit_direction = Vec3::unit_vector(r.direction);

    // 为了计算方便，我们将方向的 y 值，从 [-1,1] 映射到 [0, 1]
    let t = 0.5 * (unit_direction.y + 1.0);

    // 做一个蓝白渐变，当 t 为 0 时，就是白色，将 t 为 1 时，就是蓝色
    return (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image config
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    // Camera config
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("{}", format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(origin, direction);
            let pixel_color = ray_color(r);
            println!("{}", pixel_color.get_color_string());
        }
    }
}
