#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub fn u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
