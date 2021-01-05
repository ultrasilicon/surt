use crate::color::ColorRGB;
use crate::ray::Ray;
use crate::surface::HitRecord;
use crate::vector::Vector3d;

use rand::Rng;



#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: ColorRGB },
    Metal { albedo: ColorRGB, fuzz: f64 },
    Dielectric { ir: f64 },
}

pub fn refract(uv: Vector3d, n: Vector3d, refr_ratio: f64) -> Vector3d  {
    let cos_theta = Vector3d::dot(-uv, n).min(1.0);
    let r_perp = (uv + n * cos_theta) * refr_ratio;
    let r_para = - n* (1.0 - r_perp.len_sqr()).abs().sqrt();
    return r_perp + r_para;
}

pub fn reflect(vec: Vector3d, normal: Vector3d) -> Vector3d {
    return vec - normal * Vector3d::dot(vec, normal) * 2.0;
}

pub fn reflectance(cos: f64, refr_idx: f64) -> f64 {
    let r0 = ((1.0 - refr_idx) / (1.0 + refr_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, ColorRGB)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scattered_dir = rec.normal + Vector3d::random_unit_vec();
                if scattered_dir.near_zero() {
                    scattered_dir = rec.normal;
                }
                return Some((Ray::new(rec.point, scattered_dir), *albedo));
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray.dir.unit_vec(), rec.normal);
                let scattered = Ray::new(
                    rec.point,
                    reflected + Vector3d::random_in_unit_sphere() * *fuzz,
                );
                if Vector3d::dot(scattered.dir, rec.normal) > 0.0 {
                    return Some((scattered, *albedo));
                } else {
                    return None;
                }
            }
            Material::Dielectric {ir} => {
                let refr_ratio: f64 = if rec.front_face { 1.0 / ir} else { *ir };
                let unit_dir = ray.dir.unit_vec();

                let cos_theta = Vector3d::dot(-unit_dir, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                
                let refracted;

                let mut rng = rand::thread_rng();
                let full_reflect = reflectance(cos_theta, refr_ratio) > rng.gen::<f64>();
                let cannor_refract = refr_ratio * sin_theta > 1.0;

                if  cannor_refract || full_reflect {
                    refracted = reflect(unit_dir, rec.normal);
                } else {
                    refracted = refract(unit_dir, rec.normal, refr_ratio);
                }
                
                return Some((Ray::new(rec.point, refracted), ColorRGB::new(1.0, 1.0, 1.0)));
            }
        }
    }
}
