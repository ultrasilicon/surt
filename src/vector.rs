
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector3d{
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

impl Vector3d {
    pub fn new(x: f64, y:f64, z:f64) -> Vector3d{
        return Vector3d{
            x: x,
            y: y,
            z: z
        }
    }

    pub fn len_sqr(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn len(&self) -> f64 {
        return self.len_sqr().sqrt();
    }

    pub fn unit_vec(self) -> Vector3d {
        return self / self.len();
    }

    pub fn dot(lhs: Vector3d, rhs: Vector3d) -> f64 {
        return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
    }
}

impl ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        return Vector3d{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        };
    }
}

impl ops::Neg for Vector3d {
    type Output = Vector3d;

    fn neg(self) -> Vector3d {
        return Vector3d{
            x: - self.x,
            y: - self.y,
            z: - self.z
        };
    }
}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn add(self, rhs: Vector3d) -> Vector3d {
        return Vector3d{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl ops::Mul<f64> for Vector3d {
    type Output = Vector3d;
    fn mul(self, rhs: f64) -> Vector3d {
        return Vector3d{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        };
    }
}

impl ops::Div<f64> for Vector3d {
    type Output = Vector3d;
    fn div(self, rhs: f64) -> Vector3d {
        return Vector3d{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        };
    }
}