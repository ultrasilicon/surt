use crate::ray::Ray;
use crate::vector::Vector3d;

const VIEW_UP: Vector3d = Vector3d{x: 0.0, y: 1.0, z: 0.0};

pub struct Camera {
    pub origin: Vector3d,
    pub lower_left: Vector3d,
    pub horizontal: Vector3d,
    pub vertical: Vector3d,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera {
    pub fn new(origin: Vector3d, look_at: Vector3d, vfov_deg: f64, aspect_ratio: f64) -> Camera {
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).unit_vec();
        let u = Vector3d::cross(VIEW_UP, w).unit_vec();
        let v = Vector3d::cross(w, u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left: origin - (horizontal + vertical) / 2.0 - w,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
        };
    }

    pub fn update_angle(&mut self, look_at: Vector3d) {
        let w = (self.origin - look_at).unit_vec();
        let u = Vector3d::cross(VIEW_UP, w).unit_vec();
        let v = Vector3d::cross(w, u);

        self.horizontal = u * self.viewport_width;
        self.vertical = v * self.viewport_height;
        self.lower_left = self.origin - (self.horizontal + self.vertical) / 2.0 - w;
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        );
    }
}
