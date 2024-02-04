use std::ops::{Add, Mul, Sub, AddAssign, SubAssign, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r, g, b, a
        }
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0, 1.0)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::black()
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
            (self.r - other.r).abs() < 1.0e-3 &&
            (self.g - other.g).abs() < 1.0e-3 &&
            (self.b - other.b).abs() < 1.0e-3
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl MulAssign for Color {

    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

impl DivAssign for Color {

    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_color() {
        let c = Color::new(-0.5, 0.4, 1.7, 1.0);
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75, 1.0);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.0);
        let c3 = c1 + c2;
        assert_eq!(c3, Color::new(1.6, 0.7, 1.0, 2.0));
    }

    #[test]
    fn add_assign_colors() {
        let mut c1 = Color::new(0.9, 0.6, 0.75, 1.0);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.0);
        c1 += c2;
        assert_eq!(c1, Color::new(1.6, 0.7, 1.0, 2.0));
    }

    #[test]
    fn substracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75, 1.0);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.0);
        let c3 = c1 - c2;
        assert_eq!(c3, Color::new(0.2, 0.5, 0.5, 0.0));
    }

    #[test]
    fn sub_assign_colors() {
        let mut c1 = Color::new(0.9, 0.6, 0.75, 1.0);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.0);
        c1 -= c2;
        assert_eq!(c1, Color::new(0.2, 0.5, 0.5, 0.0));
    }

    #[test]
    fn multiplying_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4, 1.0);
        let c2 = c * 2.0;
        assert_eq!(c2, Color::new(0.4, 0.6, 0.8, 2.0));
    }

    #[test]
    fn mul_assign_color_by_a_scalar() {
        let mut c = Color::new(0.2, 0.3, 0.4, 1.0);
        c *= 2.0;
        assert_eq!(c, Color::new(0.4, 0.6, 0.8, 2.0));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4, 1.0);
        let c2 = Color::new(0.9, 1.0, 0.1, 1.0);
        let c3 = c1 * c2;
        assert_eq!(c3, Color::new(0.9, 0.2, 0.04, 1.0));
    }

    #[test]
    fn mul_assign_colors() {
        let mut c1 = Color::new(1.0, 0.2, 0.4, 1.0);
        let c2 = Color::new(0.9, 1.0, 0.1, 1.0);
        c1 *= c2;
        assert_eq!(c1, Color::new(0.9, 0.2, 0.04, 1.0));
    }
}