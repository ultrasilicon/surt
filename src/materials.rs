
use crate::color::ColorRGB;
use crate::surface::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector3d;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian{ albedo: ColorRGB },
    Metal{ albedo: ColorRGB },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, ColorRGB)> {
        match self {
            Material::Lambertian{ albedo } => {
                let mut scattered_dir = rec.normal + Vector3d::random_unit_vec();
                if scattered_dir.near_zero() {
                    scattered_dir = rec.normal;
                }
                return Some((Ray::new(rec.point, scattered_dir), *albedo));
            },
            Material::Metal{ albedo } => {
                let reflected = Vector3d::reflect(ray.dir.unit_vec(), rec.normal);
                let scattered = Ray::new(rec.point, reflected);
                if Vector3d::dot(scattered.dir, rec.normal) > 0.0 {
                    return Some((scattered, *albedo));
                } else {
                    return None;
                }
            }
        }

    }
}