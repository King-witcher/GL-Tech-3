use std::{alloc::Layout, ptr::NonNull};

use crate::imaging::Color;

pub struct Image {
    buffer: NonNull<Color>,
    width: u32,
    height: u32,
    pub(crate) widthf: f32,
    pub(crate) heightf: f32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let layout = Layout::array::<Color>((width * height) as usize).unwrap();

        let buffer = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut Color;
            NonNull::new_unchecked(ptr)
        };

        Self {
            buffer,
            width,
            height,
            widthf: width as f32,
            heightf: height as f32,
        }
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

    #[inline]
    pub(crate) fn u32_buffer(&self) -> *mut u32 {
        self.buffer.as_ptr() as *mut u32
    }

    #[inline]
    pub(crate) fn u8_buffer(&self) -> *mut u8 {
        self.buffer.as_ptr() as *mut u8
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Color {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer.as_ptr();
            buffer = buffer.add(index);
            *buffer
        }
    }

    #[inline]
    pub fn set(&self, x: u32, y: u32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer.as_ptr();
            buffer = buffer.add(index);
            *buffer = value;
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        let layout = Layout::array::<Color>((self.width * self.height) as usize).unwrap();
        unsafe {
            std::alloc::dealloc(self.buffer.as_ptr() as *mut u8, layout);
        }
    }
}
