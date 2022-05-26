use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color (pub f64, pub f64, pub f64, pub f64);

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color (self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, self.3 * rhs.3)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color (self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, 1.0)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, scalar: f64) -> Self::Output {
        Color (self.0 * scalar, self.1 * scalar, self.2 * scalar, 1.0)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Color {
    pub fn to_bytes(&self) -> u32 {
        let r = ((self.0.sqrt() * 255.0).round().min(255.0) as u32) << 24;
        let g = ((self.1.sqrt() * 255.0).round().min(255.0) as u32) << 16;
        let b = ((self.2.sqrt() * 255.0).round().min(255.0) as u32) << 8;
        let a = ((self.3.sqrt() * 255.0).round().min(255.0) as u32) << 0;
        r + g + b + a
    }

    pub fn from_bytes(byte: u32) -> Self where Self: Sized {
        let r = ((byte & 0xff000000) >> 24) as f64 / 255.0;
        let g = ((byte & 0x00ff0000) >> 16) as f64 / 255.0;
        let b = ((byte & 0x0000ff00) >> 8) as f64 / 255.0;
        let a = ((byte & 0x000000ff) >> 0) as f64 / 255.0;
        Color(r, g, b, a) 
    }

    pub fn normalise(&self) -> Color {
        Color (self.0.min(1.0), self.1.min(1.0), self.2.min(1.0), self.3.min(1.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_bytes() {
        let c = Color (1.0, 1.0, 1.0, 1.0);
        let bytes = c.to_bytes();
        assert_eq!(bytes, 0xffffffff)
    }

    #[test]
    fn from_bytes() {
        let bytes = 0xffffffff;
        let c = Color::from_bytes(bytes);
        assert_eq!(c, Color (1.0, 1.0, 1.0, 1.0))
    }
}
