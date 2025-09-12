use std::ffi::CString;

use crate::imaging::Image;
use crate::sdl::*;

pub struct Window {
    buffer: Image,
    window: usize,
    renderer: usize,
    texture: usize,
}

impl Window {
    pub fn new(title: &str, w: i32, h: i32, buf_w: i32, buf_h: i32, fullscreen: bool) -> Self {
        unsafe {
            let buffer = Image::new(buf_w as u32, buf_h as u32);

            let title = CString::new(title.as_bytes()).unwrap();
            let mut flags = SDL_WINDOW_VULKAN;
            if fullscreen {
                flags |= SDL_WINDOW_FULLSCREEN;
            }

            let window = SDL_CreateWindow(
                title.as_ptr() as *const u8,
                SDL_WINDOWPOS_UNDEFINED,
                SDL_WINDOWPOS_UNDEFINED,
                w,
                h,
                flags,
            );

            let renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

            let texture = SDL_CreateTexture(
                renderer,
                SDLPixelFormat::RGBX8888,
                SDLTextureAccess::Static,
                buf_w,
                buf_h,
            );

            println!("{}", get_last_error());

            Self {
                buffer,
                window,
                renderer,
                texture,
            }
        }
    }

    pub fn update(&self) {
        unsafe {
            SDL_UpdateTexture(
                self.texture,
                None,
                self.buffer.u32_buffer(),
                (self.buffer.width() * 4) as i32,
            );

            SDL_RenderCopy(self.renderer, self.texture, None, None);
            SDL_RenderPresent(self.renderer);
        }
    }

    pub fn image(&self) -> &Image {
        &self.buffer
    }

    pub fn get_mouse_shift() -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            SDL_GetRelativeMouseState(&mut x, &mut y);
        }
        (x, y)
    }

    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.buffer
    }

    pub fn keep_alive(&self) {}

    pub fn events() -> impl Iterator<Item = SDLEvent> {
        EventIterator
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.texture);
            SDL_DestroyRenderer(self.renderer);
            SDL_DestroyWindow(self.window);
        }
    }
}

struct EventIterator;

impl Iterator for EventIterator {
    type Item = SDLEvent;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut event = Default::default();

            if SDL_PollEvent(&mut event) {
                Some(event)
            } else {
                None
            }
        }
    }
}
