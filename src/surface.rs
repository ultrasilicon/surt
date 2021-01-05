
use crate::materials::Material;
use crate::vector::Vector3d;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vector3d,
    pub normal: Vector3d,
    pub t: f64,
    pub front_face: bool,
    pub material: Material

}

impl HitRecord {
    pub fn new(point: &Vector3d, t: f64, ray: &Ray, out_normal: Vector3d, material: Material) -> HitRecord {
        let front_face = Vector3d::dot(ray.dir, out_normal) < 0.0;
        return HitRecord{
            point: point.clone(),
            normal: if front_face { out_normal } else {-out_normal},
            // normal: out_normal,
            t: t,
            front_face: front_face,
            material: material
        }
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


