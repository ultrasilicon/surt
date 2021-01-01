
use crate::surface::Surface;
use crate::surface::HitRecord;
use crate::vector::Vector3d;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3d,
    pub radius: f64
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vector3d = ray.orig - self.center;
        let a = ray.dir.len_sqr();
        let b_half = Vector3d::dot(oc, ray.dir);
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root: f64 = (- b_half - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (- b_half + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let rec = HitRecord::new(
            &ray.at(root),
            root,
            ray,
            (ray.at(root) - self.center) / self.radius
        );
        return Some(rec);
    }
}
