use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64) -> Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let result = half_b * half_b - a * c;
        if result < 0.0 {
            return None;
        }

        let sqrtd = result.sqrt();

        // 找到光线打到球面最近的点，有可能光线会穿透球体，与两个点相交，
        // 但是远的点会被近的面遮住，我们看不到，所以这里使用近的点就可以了
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let hit_rec = HitRecord::new(p, t, outward_normal, *r);
        return Some(hit_rec);
    }
}
