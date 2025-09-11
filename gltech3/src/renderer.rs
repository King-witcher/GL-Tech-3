use core::f32;
use std::slice;

use sdl2::{EventPump, pixels::PixelFormatEnum, render::Canvas, video::Window};

use crate::{
    imaging::Image,
    scripting::script::UpdateContext,
    vector::Vector,
    world::{Entity, Scene, Spatial},
};

pub struct RendererBuilder {
    fullscreen: bool,
    scene: Scene,
    width: u32,
    height: u32,
    title: String,
}

struct RendererState<'a> {
    canvas: Canvas<Window>,
    image: Image,
    event_pump: EventPump,
    scene: Scene,
    sdl_texture: sdl2::render::Texture<'a>,
}

impl RendererBuilder {
    pub fn new(scene: Scene) -> Self {
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
        let sdl_texture = texture_creator
            .create_texture(
                PixelFormatEnum::RGBA8888,
                sdl2::render::TextureAccess::Static,
                self.width,
                self.height,
            )
            .unwrap();

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

impl RendererState<'_> {
    fn run(mut self) {
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
                let slice = slice::from_raw_parts(
                    self.image.buffer.as_ptr() as *mut u8,
                    (self.image.width * self.image.height * 4) as usize,
                );
                self.sdl_texture
                    .update(None, slice, (self.image.width * 4) as usize)
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
        unsafe {
            std::ptr::write_bytes(
                self.image.buffer.as_ptr(),
                0,
                (self.image.width * self.image.height) as usize,
            );
        }
        let tan = f32::tan(110.0 * 0.5 * f32::consts::PI / 180.0);
        let step0 = 2.0 * tan / self.image.width as f32;
        let col_height_1 = self.image.width as f32 / (2.0 * tan);
        let camera_dir = self.scene.camera.dir();
        let camera_left = Vector(-camera_dir.1, camera_dir.0);

        for col in 0..self.image.width {
            let ray_direction = {
                let delta = self.image.width as i32 / 2 - col as i32;
                let mut dir = camera_dir + camera_left * step0 * delta as f32;
                dir.modularize();
                dir
            };

            let collision = self
                .scene
                .nearest_plane(self.scene.camera.pos(), ray_direction);

            let Some((plane, distance)) = collision else {
                continue;
            };

            // println!("Plane: {}", plane.dir().module());

            let col_h = col_height_1 / (ray_direction.dot_product(&camera_dir) * distance);
            let col_start = (self.image.height as f32 - 1.0 - col_h) * 0.5;
            let col_end = (self.image.height as f32 - 1.0 + col_h) * 0.5;

            let mut draw_col_start =
                self.image.height as i32 - (self.image.height as f32 - col_start) as i32; // Inclusve
            let mut draw_col_end =
                self.image.height as i32 - (self.image.height as f32 - col_end) as i32; // Exclusive

            if draw_col_start < 0 {
                draw_col_start = 0;
            }

            if draw_col_end >= self.image.height as i32 {
                draw_col_end = self.image.height as i32;
            }

            // let i_col_h = 1.0 / col_h;

            for line in draw_col_start..draw_col_end {
                self.image.set(col, line as u32, 0xFFFFFFFF);
            }
        }
    }
}
