use std::cell::UnsafeCell;

use shared_vector::{RefCountedVector, SharedVector};

use crate::imaging::Color;

pub struct Image {
    buffer: SharedVector<UnsafeCell<Color>>,
    width: i32,
    height: i32,
    pub(crate) widthf: f32,
    pub(crate) heightf: f32,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        let buffer = RefCountedVector::with_capacity((width * height) as usize);

        Self {
            buffer,
            width,
            height,
            widthf: width as f32,
            heightf: height as f32,
        }
    }

    pub fn cheap_clone(&self) -> Self {
        println!("Cloning image");
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
        self.buffer.as_slice()
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

    pub fn size(&self) -> usize {
        (self.width * self.height * 4) as usize
    }

    #[inline]
    pub(crate) fn u32_buffer(&self) -> *mut i32 {
        self.buffer.as_ref() as *const [Color] as *mut i32
    }

    #[inline]
    pub(crate) unsafe fn mut_ptr(&self) -> &mut [Color] {
        unsafe {
            let ptr = self.buffer.as_ref() as *const [Color] as *mut [Color];
            &mut *ptr
        }
    }

    #[inline]
    pub(crate) fn u8_buffer(&self) -> *mut u8 {
        self.buffer.get().cast::<u8>()
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Color {
        unsafe {
            let index: usize = (x + self.width * y) as usize;
            *self.buffer[index].get()
        }
    }

    #[inline]
    pub(crate) fn set_unsafe(&self, x: i32, y: i32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let reference = &self.buffer[index];
            let mutable = reference.get();
            *mutable = value;
        }
    }

    #[inline]
    pub fn set(&self, x: i32, y: i32, value: Color) {
        let index: usize = (x + self.width * y) as usize;
        unsafe {
            let reference = &self.buffer[index];
            let mutable = reference.get();
            *mutable = value;
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (i32, i32)> {
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
