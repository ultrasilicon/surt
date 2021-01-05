use crate::ray::Ray;
use crate::vector::Vector3d;

pub struct Camera {
    pub origin: Vector3d,
    pub lower_left: Vector3d,
    pub horizontal: Vector3d,
    pub vertical: Vector3d,
}

impl Camera {
    pub fn new(look_from: Vector3d, look_at: Vector3d, view_up: Vector3d, vfov_deg: f64, aspect_ratio: f64) -> Camera {
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vec();
        let u = Vector3d::cross(view_up, w).unit_vec();
        let v = Vector3d::cross(w, u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        return Camera {
            origin: look_from,
            horizontal: horizontal,
            vertical: vertical,
            lower_left: look_from
                - horizontal / 2.0
                - vertical / 2.0
                - w,
        };
    }


    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        );
    }
}
