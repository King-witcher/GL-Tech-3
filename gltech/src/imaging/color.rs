#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF)
    }

    #[inline]
    pub fn u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn r(self) -> u8 {
        (self.0 >> 24) as u8
    }

    #[inline]
    pub fn g(self) -> u8 {
        (self.0 >> 16) as u8
    }

    #[inline]
    pub fn b(self) -> u8 {
        (self.0 >> 8) as u8
    }

    #[inline]
    pub fn a(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    #[inline]
    pub fn luma(self) -> u8 {
        let r = self.r() as f32;
        let g = self.g() as f32;
        let b = self.b() as f32;
        (0.299 * r + 0.587 * g + 0.114 * b) as u8
    }

    #[inline]
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let r = self.r() as f32 * (1.0 - t) + other.r() as f32 * t;
        let g = self.g() as f32 * (1.0 - t) + other.g() as f32 * t;
        let b = self.b() as f32 * (1.0 - t) + other.b() as f32 * t;
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        Color::rgb(r, g, b)
    }

    pub const BLACK: Color = Color(0);
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const RED: Color = Color(0xFF0000FF);
    pub const GREEN: Color = Color(0x00FF00FF);
    pub const BLUE: Color = Color(0x0000FFFF);
    pub const YELLOW: Color = Color(0xFFFF00FF);
    pub const CYAN: Color = Color(0x00FFFFFF);
    pub const MAGENTA: Color = Color(0xFF00FFFF);
}

impl From<u32> for Color {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
