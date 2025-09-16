use std::{alloc::Layout, ptr::NonNull};

use crate::imaging::Color;

pub struct Image {
    buffer: NonNull<Color>,
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

    pub(crate) fn from_raw_parts(buffer: *const u32, width: u32, height: u32) -> Self {
        unsafe {
            Self {
                buffer: NonNull::new(buffer as *mut Color).unwrap_unchecked(),
                width,
                height,
                widthf: width as f32,
                heightf: height as f32,
            }
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

    pub fn size(&self) -> usize {
        (self.width * self.height * 4) as usize
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
    pub(crate) fn set_unsafe(&self, x: u32, y: u32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer.as_ptr();
            buffer = buffer.add(index);
            *buffer = value;
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

    pub fn coordinates(&self) -> impl Iterator<Item = (u32, u32)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
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
