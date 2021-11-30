use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use std::vec::Vec;

pub struct HittableList {
    pub hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.hittable_list.push(object);
    }

    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result_hit_rec: Option<HitRecord> = None;
        let mut closet_so_far = t_max;

        for obj in self.hittable_list.iter() {
            if let Some(hit_rec) = obj.hit(r, t_min, closet_so_far) {
                closet_so_far = hit_rec.t;
                result_hit_rec = Some(hit_rec);
            }
        }

        return result_hit_rec;
    }
}
