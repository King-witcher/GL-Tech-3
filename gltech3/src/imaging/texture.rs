use crate::imaging::{Color, Image};

pub struct Texture<'a> {
    source: &'a Image,
    hoffset: f32,
    voffset: f32,
    hrepeat: f32,
    vrepeat: f32,
}

impl<'a> Texture<'a> {
    pub fn new(source: &'a Image) -> Self {
        Self {
            source,
            hoffset: 0.0,
            voffset: 0.0,
            hrepeat: 1.0,
            vrepeat: 1.0,
        }
    }

    #[inline]
    pub fn source(&self) -> &Image {
        self.source
    }

    #[inline]
    pub fn hoffset(&self) -> f32 {
        self.hoffset
    }

    #[inline]
    pub fn voffset(&self) -> f32 {
        self.voffset
    }

    #[inline]
    pub fn hrepeat(&self) -> f32 {
        self.hrepeat
    }

    #[inline]
    pub fn vrepeat(&self) -> f32 {
        self.vrepeat
    }

    #[inline]
    pub fn map_pixel(&self, x: f32, y: f32) -> Color {
        let x = self.source.widthf * (self.hrepeat * x + self.hoffset) % self.source.widthf;
        let y = self.source.heightf * (self.vrepeat * y + self.voffset) % self.source.heightf;
        let x = x as u32;
        let y = y as u32;
        self.source.get(x, y)
    }
}
