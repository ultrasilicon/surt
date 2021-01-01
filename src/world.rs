
use std::vec::Vec;

use crate::surface::Surface;
use crate::surface::HitRecord;
use crate::ray::Ray;

#[derive(Default)]
pub struct World {
    surface_list: Vec<Box<dyn Surface>>
}

impl World {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut closest_rec: Option<HitRecord> = None;
        let mut closest: f64 = t_max;

        for s in &self.surface_list {
            let result = s.hit(ray, t_min, closest);
            match result {
                Some(r) => {
                    closest = r.t;
                    closest_rec = Some(r);
                },
                None => {}
            }
        }

        return closest_rec
    }

    pub fn add(&mut self, surface: Box<dyn Surface>) {
        self.surface_list.push(surface);
    }
}