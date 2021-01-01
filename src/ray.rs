
use crate::vector::Vector3d;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vector3d,
    pub dir: Vector3d
}

impl Ray {
    pub fn new(origin: Vector3d, dir: Vector3d) -> Ray {
        return Ray{
            orig: origin,
            dir: dir
        }
    }

    pub fn at(&self, t: f64) -> Vector3d {
        return self.orig + self.dir * t
    }
}
