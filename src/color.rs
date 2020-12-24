
use std::ops;

// pub trait Exportable {
//     fn to_ppm(&self) -> String;
// }

#[derive(Copy, Clone)]
pub struct ColorRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorRGB {
    pub fn to_ppm(&self) -> String {
        let ppm = format!( 
            "{} {} {}\n", 
            (255.999 * self.r) as i64, 
            (255.999 * self.g) as i64,
            (255.999 * self.b) as i64
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