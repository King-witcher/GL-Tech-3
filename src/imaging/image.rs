use core::alloc;
use std::{alloc::Layout, ptr::NonNull};

use sdl2::libc::{self, *};

pub struct Image {
    pub(crate) buffer: NonNull<u32>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) widthf: f32,
    pub(crate) heightf: f32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let layout = Layout::array::<u32>((width * height) as usize).unwrap();
        let size = width * height * 4;

        let buffer = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut u32;
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> u32 {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let mut buffer = self.buffer.as_ptr();
            buffer = buffer.add(index);
            *buffer
        }
    }

    pub fn set(&self, x: u32, y: u32, value: u32) {
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
        let layout = Layout::array::<u32>((self.width * self.height) as usize).unwrap();
        unsafe {
            std::alloc::dealloc(self.buffer.as_ptr() as *mut u8, layout);
        }
    }
}
