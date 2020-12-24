
use crate::vector::Vector3D;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Vector3D,
    pub dir: Vector3D
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3D {
        return self.orig + self.dir * t
    }


}
