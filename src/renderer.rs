use core::f32;
use std::{slice, time::Instant};

use sdl2::{pixels::PixelFormatEnum, render::Canvas, video::Window};

use crate::{imaging::Image, vector::Vector, world::Scene};

pub struct RendererContext {
    scene: Scene,
    width: u32,
    height: u32,
    title: String,
}

impl RendererContext {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            width: 800,
            height: 450,
            title: "GLTech 3".into(),
        }
    }

    pub fn run(self) {
        // let window = Window::new("GLTech 3", 800, 450, 800, 450, false);
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .opengl()
            .fullscreen()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture(
                PixelFormatEnum::RGBA8888,
                sdl2::render::TextureAccess::Static,
                self.width,
                self.height,
            )
            .unwrap();

        let mut event_pump = sdl.event_pump().unwrap();

        let mut image = Image::new(self.width, self.height);
        let mut frame = 0;
        let mut last_instant = Instant::now();

        'main_loop: loop {
            for event in event_pump.poll_iter() {
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

            // Rendering code would go here
            self.draw(&self.scene, &mut image);
            println!("{}", last_instant.elapsed().as_millis());
            last_instant = Instant::now();

            canvas.clear();
            unsafe {
                let slice = slice::from_raw_parts(
                    image.buffer.as_ptr() as *mut u8,
                    (image.width * image.height * 4) as usize,
                );
                texture
                    .update(None, slice, (image.width * 4) as usize)
                    .unwrap();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();
            }
            canvas.present();
            frame += 1;
        }
    }

    fn draw(&self, scene: &Scene, image: &mut Image) {
        let tan = f32::tan(90.0 * 0.5 * f32::consts::PI / 180.0);
        let step0 = 2.0 * tan / image.width as f32;
        let col_height_1 = image.width as f32 / (2.0 * tan);
        let left_pixel = tan - step0 - 0.5;
        for col in 0..self.width {
            let relative_angle =
                f32::atan(col as f32 * step0 - left_pixel) * 180.0 / f32::consts::PI;
            let ray_cos = f32::cos(relative_angle * f32::consts::PI / 180.0);
            let ray_angle = scene.camera.rotation + relative_angle;
            let ray_direction = Vector::from_angle(ray_angle);

            let collision = self
                .scene
                .nearest_plane(scene.camera.position, ray_direction);

            let Some((plane, distance)) = collision else {
                continue;
            };

            let col_h = col_height_1 / (ray_cos * distance);
            let col_start = (image.height as f32 - 1.0 - col_h) * 0.5;
            let col_end = (image.height as f32 - 1.0 + col_h) * 0.5;

            let draw_col_start = image.height - (image.height as f32 - col_start) as u32; // Inclusve
            let mut draw_col_end = image.height - (image.height as f32 - col_end) as u32; // Exclusive

            if draw_col_end >= image.height {
                draw_col_end = image.height;
            }

            // let i_col_h = 1.0 / col_h;

            for line in draw_col_start..draw_col_end {
                // let vratio = (line as f32 - col_start) * i_col_h;
                image.set(col, line, 0xFFFFFFFF);
            }
        }
    }
}
