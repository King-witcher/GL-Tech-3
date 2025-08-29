use std::os::raw::c_void;

use sdl2::{
    pixels,
    sys::{
        SDL_CreateRenderer, SDL_CreateTexture, SDL_CreateWindow, SDL_DestroyRenderer,
        SDL_DestroyTexture, SDL_DestroyWindow, SDL_EventType, SDL_PollEvent, SDL_Rect,
        SDL_RenderCopy, SDL_RenderPresent, SDL_Renderer, SDL_RendererFlags, SDL_Texture,
        SDL_UpdateTexture, SDL_WINDOWPOS_UNDEFINED_MASK, SDL_Window, SDL_WindowFlags,
    },
};

use crate::buffer::Image;

pub struct Window {
    buffer: Image,
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
}

impl Window {
    pub fn new(title: &str, w: i32, h: i32, buf_w: i32, buf_h: i32, fullscreen: bool) -> Self {
        unsafe {
            let title = format!("{}\0", title);
            let title = title.as_bytes().as_ptr() as *const i8;

            let mut flags = SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
            if fullscreen {
                flags |= SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
            }

            let buffer = Image::new(buf_w as u32, buf_h as u32);

            let window = SDL_CreateWindow(
                title,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                w,
                h,
                flags,
            );

            let renderer = SDL_CreateRenderer(
                window,
                -1,
                SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            );

            let texture = SDL_CreateTexture(
                renderer,
                pixels::PixelFormatEnum::RGBA32 as u32,
                sdl2::render::TextureAccess::Static as i32,
                buf_w,
                buf_h,
            );

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
                0 as *const SDL_Rect,
                self.buffer.buffer.as_ptr() as *const c_void,
                (self.buffer.width * 4) as i32,
            );
            SDL_RenderCopy(
                self.renderer,
                self.texture,
                0 as *const SDL_Rect,
                0 as *const SDL_Rect,
            );
            SDL_RenderPresent(self.renderer);
        }
    }

    pub fn image(&self) -> &Image {
        &self.buffer
    }

    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.buffer
    }

    pub fn keep_alive(&self) {}

    pub fn events() -> impl Iterator<Item = SDL_EventType> {
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
    type Item = SDL_EventType;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut event = std::mem::zeroed();

            if SDL_PollEvent(&mut event) != 0 {
                Some(std::mem::transmute(event.type_))
            } else {
                None
            }
        }
    }
}
