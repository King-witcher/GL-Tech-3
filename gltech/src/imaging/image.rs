use std::{alloc::Layout, sync::Arc};

use crate::imaging::Color;

pub struct Image {
    buffer: Arc<Color>,
    width: u32,
    height: u32,
    pub(crate) widthf: f32,
    pub(crate) heightf: f32,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let layout = Layout::array::<Color>((width * height) as usize).unwrap();

        let buffer = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut Color;
            Arc::from_raw(ptr)
        };

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
    pub(crate) fn byte_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.u8_buffer(), self.size()) }
    }

    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> usize {
        (self.width * self.height * 4) as usize
    }

    #[inline]
    pub(crate) fn u32_buffer(&self) -> *mut u32 {
        self.buffer.as_ref() as *const Color as *mut u32
    }

    #[inline]
    pub(crate) unsafe fn buffer(&self) -> *mut Color {
        self.buffer.as_ref() as *const Color as *mut Color
    }

    #[inline]
    pub(crate) fn u8_buffer(&self) -> *mut u8 {
        self.buffer.as_ref() as *const Color as *mut u8
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Color {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer();
            buffer = buffer.add(index);
            *buffer
        }
    }

    #[inline]
    pub(crate) fn set_unsafe(&self, x: u32, y: u32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer();
            buffer = buffer.add(index);
            *buffer = value;
        }
    }

    #[inline]
    pub fn set(&self, x: u32, y: u32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer();
            buffer = buffer.add(index);
            *buffer = value;
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (u32, u32)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }
}

// impl Drop for Image {
//     fn drop(&mut self) {
//         let layout = Layout::array::<Color>((self.width * self.height) as usize).unwrap();
//         unsafe {
//             std::alloc::dealloc(self.u8_buffer(), layout);
//         }
//     }
// }
