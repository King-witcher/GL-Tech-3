use std::rc::Rc;

use crate::imaging::{Color, Image};

#[derive(Clone)]
pub struct Texture {
    source: Rc<Image>,
    hoffset: f32,
    voffset: f32,
    hrepeat: f32,
    vrepeat: f32,
}

impl Texture {
    pub fn new(source: Rc<Image>) -> Self {
        Self {
            source,
            hoffset: 0.0,
            voffset: 0.0,
            hrepeat: 1.0,
            vrepeat: 1.0,
        }
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
    pub fn map_flat(&self, u: f32, v: f32) -> Color {
        let x = self.source.widthf * (self.hrepeat * u + self.hoffset) % self.source.widthf;
        let y = self.source.heightf * (self.vrepeat * v + self.voffset) % self.source.heightf;
        let x = x as u32;
        let y = y as u32;
        self.source.get(x, y)
    }

    #[inline]
    pub fn map_bilinear(&self, u: f32, v: f32) -> Color {
        let wf = self.source.widthf - 1.0;
        let hf = self.source.heightf - 1.0;

        let x = wf * (self.hrepeat * u + self.hoffset) % wf;
        let y = hf * (self.vrepeat * v + self.voffset) % hf;

        let x0 = x as u32;
        let y0 = y as u32;

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let q11 = self.source.get(x0, y0);
        let q21 = self.source.get(x0 + 1, y0);
        let q12 = self.source.get(x0, y0 + 1);
        let q22 = self.source.get(x0 + 1, y0 + 1);

        let top = q11.lerp(q21, tx);
        let bottom = q12.lerp(q22, tx);

        return top.lerp(bottom, ty);
    }
}
