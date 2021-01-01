use crate::ray::Ray;
use crate::vector::Vector3d;

pub struct Camera {
    pub origin: Vector3d,
    pub lower_left: Vector3d,
    pub horizontal: Vector3d,
    pub vertical: Vector3d,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vector3d::new(0.0, 0.0, 0.0);
        let horizontal = Vector3d::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3d::new(0.0, viewport_height, 0.0);
        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector3d::new(0.0, 0.0, focal_length),
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        );
    }
}
