
use std::ops;

use crate::vector::Vector3d;
use crate::utils::clamp;

#[derive(Copy, Clone)]
pub struct ColorRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorRGB {
    pub const WHITE: ColorRGB = ColorRGB{r: 1.0, g: 1.0, b: 1.0};
    pub const BLACK: ColorRGB = ColorRGB{r: 0.0, g: 0.0, b: 0.0};

    pub fn new(r: f64, g: f64, b: f64) -> ColorRGB {
        return ColorRGB{r: r, g: g, b: b}
    }

    pub fn to_ppm(&self) -> String {
        let ppm = format!( 
            "{} {} {}\n", 
            (256.0 * clamp(self.r.sqrt(), 0.0, 0.999)) as i64, 
            (256.0 * clamp(self.g.sqrt(), 0.0, 0.999)) as i64,
            (256.0 * clamp(self.b.sqrt(), 0.0, 0.999)) as i64
        );
        return ppm;
    }


}

impl ops::Sub<ColorRGB> for ColorRGB {
    type Output = ColorRGB;

    fn sub(self, rhs: ColorRGB) -> ColorRGB {
        return ColorRGB{
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        };
    }
}

impl ops::Add<ColorRGB> for ColorRGB {
    type Output = ColorRGB;
    fn add(self, rhs: ColorRGB) -> ColorRGB {
        return ColorRGB{
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        };
    }
}

impl ops::Add<Vector3d> for ColorRGB {
    type Output = ColorRGB;
    fn add(self, rhs: Vector3d) -> ColorRGB {
        return ColorRGB{
            r: self.r + rhs.x,
            g: self.g + rhs.y,
            b: self.b + rhs.z
        };
    }
}

impl ops::AddAssign for ColorRGB {
    fn add_assign(&mut self, rhs: ColorRGB) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::SubAssign for ColorRGB {
    fn sub_assign(&mut self, rhs: ColorRGB) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl ops::Mul<f64> for ColorRGB {
    type Output = ColorRGB;
    fn mul(self, rhs: f64) -> ColorRGB {
        return ColorRGB{
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        };
    }
}

impl ops::Div<f64> for ColorRGB {
    type Output = ColorRGB;
    fn div(self, rhs: f64) -> ColorRGB {
        return ColorRGB{
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        };
    }
}