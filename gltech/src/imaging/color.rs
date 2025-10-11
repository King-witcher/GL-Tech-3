#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub const unsafe fn u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        self.0 as u8
    }

    // #[inline]
    // pub const fn a(self) -> u8 {
    //     (self.0 & 0xFF) as u8
    // }

    #[inline]
    pub const fn luma(self) -> u8 {
        let r = self.r() as f32;
        let g = self.g() as f32;
        let b = self.b() as f32;
        (0.299 * r + 0.587 * g + 0.114 * b) as u8
    }

    #[inline]
    pub const fn lerp(self, other: Color, t: f32) -> Color {
        let r = self.r() as f32 * (1.0 - t) + other.r() as f32 * t;
        let g = self.g() as f32 * (1.0 - t) + other.g() as f32 * t;
        let b = self.b() as f32 * (1.0 - t) + other.b() as f32 * t;
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        Color::rgb(r, g, b)
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const GRAY: Color = Color::rgb(128, 128, 128);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
}

impl From<u32> for Color {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
