use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn get_color_string(pixel_color: Color, samples_per_pixel: u64) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let r = (256.0 * r.clamp(0.0, 0.999)) as u64;
    let g = (256.0 * g.clamp(0.0, 0.999)) as u64;
    let b = (256.0 * b.clamp(0.0, 0.999)) as u64;

    format!("{} {} {}\n", r, g, b)
}
