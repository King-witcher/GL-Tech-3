use std::slice;

use sdl2::pixels::PixelFormatEnum;

use crate::{imaging::Image, window::Window, world::Scene};

pub fn run(scene: &mut Scene) {
    // let window = Window::new("GLTech 3", 800, 450, 800, 450, false);
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("GLTech 3", 800, 450)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::RGBA8888,
            sdl2::render::TextureAccess::Static,
            800,
            450,
        )
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut image = Image::new(800, 450);
    let mut frame = 0;

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

        for i in 0..800 {
            for j in 0..450 {
                if (frame / 60 % 2) == 0 {
                    image.set(i, j, 0xff000000);
                } else {
                    image.set(i, j, 0xffffffff);
                }
            }
        }

        // Rendering code would go here

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
