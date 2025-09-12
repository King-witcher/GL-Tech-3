use core::f32;
use std::slice;

use crate::{
    imaging::Image,
    scripting::script::UpdateContext,
    world::{Entity, Scene},
};
use crate::{prelude::*, sdl};
use sdl2::{EventPump, pixels::PixelFormatEnum, render::Canvas, video::Window};

pub struct RendererBuilder<'a> {
    fullscreen: bool,
    scene: Scene<'a>,
    width: u32,
    height: u32,
    title: String,
}

struct RendererState<'a, 'b> {
    canvas: Canvas<Window>,
    image: Image,
    event_pump: EventPump,
    scene: Scene<'a>,
    sdl_texture: sdl2::render::Texture<'b>,
}

impl<'a> RendererBuilder<'a> {
    pub fn new(scene: Scene<'a>) -> Self {
        Self {
            fullscreen: false,
            scene,
            width: 800,
            height: 450,
            title: "GLTech 3".into(),
        }
    }

    pub fn fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn start(self) {
        // let window = Window::new("GLTech 3", 800, 450, 800, 450, false);
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let sdl_texture = texture_creator.create_texture(
            PixelFormatEnum::RGBA8888,
            sdl2::render::TextureAccess::Static,
            self.width,
            self.height,
        );
        let sdl_texture = sdl_texture.unwrap();

        let event_pump = sdl.event_pump().unwrap();

        let image = Image::new(self.width, self.height);

        let renderer_state = RendererState {
            canvas,
            image,
            event_pump,
            scene: self.scene,
            sdl_texture,
        };

        renderer_state.run();
    }
}

impl RendererState<'_, '_> {
    fn run(mut self) {
        let (width, height) = self.image.dimensions();
        'main_loop: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main_loop,
                    sdl2::event::Event::KeyDown { keycode, .. } => {
                        if let Some(sdl2::keyboard::Keycode::Escape) = keycode {
                            break 'main_loop;
                        }
                    }
                    _ => {}
                }
            }

            self.draw();
            self.update();

            unsafe {
                let slice =
                    slice::from_raw_parts(self.image.u8_buffer(), (width * height * 4) as usize);
                self.sdl_texture
                    .update(None, slice, (width * 4) as usize)
                    .unwrap();
                self.canvas.copy(&self.sdl_texture, None, None).unwrap();
                self.canvas.present();
            }
            self.canvas.present();
        }
    }

    fn update(&mut self) {
        for entity in &mut self.scene.entities {
            let ptr = entity as *mut Entity;
            for script in &mut entity.scripts {
                unsafe {
                    script.update(&mut UpdateContext { entity: &mut *ptr });
                }
            }
        }
    }

    fn draw(&mut self) {
        let (width, height) = self.image.dimensions();
        unsafe {
            std::ptr::write_bytes(self.image.u32_buffer(), 0, (width * height) as usize);
        }
        let tan = f32::tan(110.0 * 0.5 * f32::consts::PI / 180.0);
        let step0 = 2.0 * tan / width as f32;
        let col_height_1 = width as f32 / (2.0 * tan);
        let camera_dir = self.scene.camera.dir();
        let camera_left = Vector(-camera_dir.1, camera_dir.0);

        for col in 0..width {
            let ray = {
                let delta = width as i32 / 2 - col as i32;
                let dir = camera_dir + camera_left * step0 * delta as f32;
                Ray::new(self.scene.camera.pos(), dir)
            };

            let collision = self.scene.raycast(ray);

            let Some((plane, (distance, split))) = collision else {
                continue;
            };

            let col_h = col_height_1 / (ray.dir.dot_product(camera_dir) * distance);
            let col_start = (self.image.heightf - 1.0 - col_h) * 0.5;
            let col_end = (self.image.heightf - 1.0 + col_h) * 0.5;

            let mut draw_col_start = height as i32 - (height as f32 - col_start) as i32; // Inclusive
            let mut draw_col_end = height as i32 - (height as f32 - col_end) as i32; // Exclusive

            if draw_col_start < 0 {
                draw_col_start = 0;
            }

            if draw_col_end >= height as i32 {
                draw_col_end = height as i32;
            }

            let i_col_h = 1.0 / col_h;
            for line in draw_col_start..draw_col_end {
                let vratio = (line as f32 - col_start) * i_col_h;
                let color = plane.texture.map_pixel(split, vratio);
                self.image.set(col, line as u32, color);
            }
        }
    }
}
