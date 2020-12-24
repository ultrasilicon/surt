use std::ops;

#[derive(Copy, Clone)]
pub struct Vector3D{
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

impl Vector3D {
    pub fn len_sqr(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn len(&self) -> f64 {
        return self.len_sqr();
    }

    pub fn unit_vec(self) -> Vector3D {
        return self / self.len();
    }

    pub fn dot(lhs: Vector3D, rhs: Vector3D) -> f64 {
        return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Vector3D {
        return Vector3D{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        };
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: Vector3D) -> Vector3D {
        return Vector3D{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f64) -> Vector3D {
        return Vector3D{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        };
    }
}

impl ops::Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Vector3D {
        return Vector3D{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        };
    }
}