use std::sync::Arc;

use crate::Color;

pub struct Image {
    buffer: Arc<[Color]>,
    width: i32,
    height: i32,
    pub(crate) widthf: f32,
    pub(crate) heightf: f32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Image dimensions must be positive");

        let size = width * height;
        let vec: Vec<Color> = vec![Default::default(); size as usize];
        let buffer: Arc<[Color]> = vec.into();

        Self {
            buffer,
            width,
            height,
            widthf: width as f32,
            heightf: height as f32,
        }
    }

    pub fn cheap_clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            width: self.width,
            height: self.height,
            widthf: self.widthf,
            heightf: self.heightf,
        }
    }

    #[inline]
    pub fn byte_slice(&self) -> &[u8] {
        let size = self.size() * size_of::<Color>();
        let ptr = self.buffer.as_ptr().cast();

        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    #[inline]
    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Get the size of the image buffer in bytes
    #[inline]
    pub fn size(&self) -> usize {
        (self.width * self.height * 4) as usize
    }

    // #[inline]
    // pub(crate) fn u32_buffer(&self) -> *mut i32 {
    //     self.buffer.as_ptr().cast()
    // }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Color {
        unsafe {
            let offset = (x + self.width * y) as isize;
            let ptr = self.buffer.as_ptr();
            *ptr.offset(offset)
        }
    }

    #[inline]
    pub(crate) fn set_unsafe(&self, x: i32, y: i32, value: Color) {
        let offset = (x + self.width * y) as isize;
        unsafe {
            let ptr = self.buffer.as_ptr().cast_mut();
            *ptr.offset(offset) = value;
        }
    }

    #[inline]
    pub fn set(&self, x: i32, y: i32, value: Color) {
        let offset = (x + self.width * y) as isize;
        unsafe {
            let ptr = self.buffer.as_ptr().cast_mut();
            *ptr.offset(offset) = value;
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (i32, i32)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }
}
