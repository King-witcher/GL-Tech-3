use std::ffi::CString;

use super::native::*;
use crate::imaging::Image;

pub struct Window {
    buffer: Image,
    window: SDLWindow,
    renderer: SDLRenderer,
    texture: SDLTexture,
}

impl Window {
    pub fn new(title: &str, w: i32, h: i32, buf_w: i32, buf_h: i32, fullscreen: bool) -> Self {
        let buffer = Image::new(buf_w as u32, buf_h as u32);

        let title = CString::new(title.as_bytes()).unwrap();
        let mut flags = SDL_WINDOW_VULKAN;
        if fullscreen {
            flags |= SDL_WINDOW_FULLSCREEN;
        }

        let window = SDL_CreateWindow(
            title,
            SDL_WINDOWPOS_UNDEFINED,
            SDL_WINDOWPOS_UNDEFINED,
            w,
            h,
            flags,
        );

        let renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

        let texture = SDL_CreateTexture(
            renderer,
            SDLPixelFormat::XRGB8888,
            SDLTextureAccess::Static,
            buf_w,
            buf_h,
        );

        // println!("{:?}", SDL_GetError());

        Self {
            buffer,
            window,
            renderer,
            texture,
        }
    }

    #[inline]
    pub fn update(&self) {
        let before_drawing = std::time::Instant::now();
        SDL_UpdateTexture(
            self.texture,
            None,
            self.buffer.u32_buffer(),
            (self.buffer.width() * 4) as i32,
        );
        let time = before_drawing.elapsed();
        println!("Draw time: {:?}", time.as_micros());

        SDL_RenderCopy(self.renderer, self.texture, None, None);
        SDL_RenderPresent(self.renderer);
    }

    pub fn image(&self) -> Image {
        self.buffer.cheap_clone()
    }

    pub fn get_mouse_shift() -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        SDL_GetRelativeMouseState(&mut x, &mut y);
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
        SDL_DestroyTexture(self.texture);
        SDL_DestroyRenderer(self.renderer);
        SDL_DestroyWindow(self.window);
    }
}

struct EventIterator;

impl Iterator for EventIterator {
    type Item = SDLEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let mut event = Default::default();

        if SDL_PollEvent(&mut event) {
            Some(event)
        } else {
            None
        }
    }
}
