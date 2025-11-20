use crate::{Color, Image};

pub struct TextureClip {
    source: Image,
    hoffset: f32,
    voffset: f32,
    hrepeat: f32,
    vrepeat: f32,
    widthf: f32,
    heightf: f32,
}

pub struct TextureClipCreateInfo {
    pub image: Image,
    pub hoffset: f32,
    pub voffset: f32,
    pub hrepeat: f32,
    pub vrepeat: f32,
    // pub mipmap_levels: u32,
}

impl TextureClip {
    pub fn new(create_info: TextureClipCreateInfo) -> Self {
        let TextureClipCreateInfo {
            image,
            hoffset,
            voffset,
            hrepeat,
            vrepeat,
        } = create_info;

        Self {
            hoffset,
            voffset,
            hrepeat,
            vrepeat,
            widthf: image.width() as f32,
            heightf: image.height() as f32,
            source: image,
        }
    }

    #[inline]
    pub(crate) fn map_nearest(&self, u: f32, v: f32) -> Color {
        let x =
            (self.source.widthf * (self.hrepeat * u + self.hoffset)) as i32 % self.source.width();
        let y =
            (self.source.heightf * (self.vrepeat * v + self.voffset)) as i32 % self.source.height();

        self.source.get(x, y)
    }

    #[inline]
    pub(crate) fn map_bilinear(&self, u: f32, v: f32) -> Color {
        let wf = self.source.widthf - 1.0;
        let hf = self.source.heightf - 1.0;

        let x = wf * (self.hrepeat * u + self.hoffset) % wf;
        let y = hf * (self.vrepeat * v + self.voffset) % hf;

        let x0 = x as i32;
        let y0 = y as i32;

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

impl Clone for TextureClip {
    fn clone(&self) -> Self {
        Self {
            source: self.source.cheap_clone(),
            hoffset: self.hoffset,
            voffset: self.voffset,
            hrepeat: self.hrepeat,
            vrepeat: self.vrepeat,
            widthf: self.widthf,
            heightf: self.heightf,
        }
    }
}
