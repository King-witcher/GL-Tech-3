use sdl2::{pixels::PixelFormatEnum, render::TextureCreator};

use crate::{Image, Ray, Scene, renderer};

pub struct Engine {
    borderless: bool,
    fullscreen: bool,
    resolution: Option<(u32, u32)>,
    sdl: sdl2::Sdl,
    title: String,
    video: sdl2::VideoSubsystem,
    vsync: bool,
}

pub fn init() -> Result<Engine, String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    Ok(Engine {
        borderless: false,
        fullscreen: false,
        resolution: None,
        sdl,
        title: "GLTech 3".into(),
        video,
        vsync: false,
    })
}

impl Engine {
    pub fn borderless(&mut self, borderless: bool) -> &mut Self {
        self.borderless = borderless;
        self
    }

    pub fn fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn resolution(&mut self, width: u32, height: u32) -> &mut Self {
        self.resolution = Some((width, height));
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn vsync(&mut self, vsync: bool) -> &mut Self {
        self.vsync = vsync;
        self
    }

    pub fn launch(self, mut scene: Scene) -> Result<(), String> {
        let window = self.create_window()?;
        let mut canvas = self.create_canvas(window)?;
        let texture_creator = canvas.texture_creator();
        let mut screen_texture = self.create_texture(&texture_creator)?;
        let mut event_pump = self.sdl.event_pump()?;
        let (width, height) = self.get_resolution()?;
        let mut gltech_image = crate::Image::new(width, height);

        let first_frame = std::time::Instant::now();
        let mut last_frame = first_frame;
        loop {
            let quit = self.handle_events(&mut event_pump);
            if quit {
                break;
            }

            let camera = Ray {
                start: scene.camera.pos(),
                dir: scene.camera.dir(),
            };

            let planes: Vec<&crate::Plane> = scene.planes().collect();

            renderer::draw_planes(camera, scene.camera.z(), planes, &mut gltech_image);
            Self::present(&mut canvas, &mut screen_texture, gltech_image.cheap_clone())?;
            let total_time = first_frame.elapsed();
            let delta_time = last_frame.elapsed();
            last_frame = std::time::Instant::now();
            scene.update(delta_time, total_time);
        }

        Ok(())
    }

    #[inline]
    fn present(
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture: &mut sdl2::render::Texture,
        image: Image,
    ) -> Result<(), String> {
        let slice = image.byte_slice();
        texture
            .update(None, slice, (image.width() * 4) as usize)
            .map_err(|e| e.to_string())?;
        canvas.copy(texture, None, None)?;
        canvas.present();
        Ok(())
    }

    fn handle_events(&self, event_pump: &mut sdl2::EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return true,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    if let Some(sdl2::keyboard::Keycode::Escape) = keycode {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn get_resolution(&self) -> Result<(u32, u32), String> {
        if let Some(res) = self.resolution {
            return Ok(res);
        } else {
            if self.fullscreen {
                let display_mode = self.video.current_display_mode(0)?;
                Ok((display_mode.w as u32, display_mode.h as u32))
            } else {
                Ok((1600, 900))
            }
        }
    }

    fn create_window(&self) -> Result<sdl2::video::Window, String> {
        let (width, height) = self.get_resolution()?;

        let mut window_builder = self.video.window(&self.title, width, height);

        if self.fullscreen {
            window_builder.fullscreen_desktop();
        }

        if self.borderless {
            window_builder.borderless();
        }

        let window = window_builder.build().map_err(|e| e.to_string())?;
        Ok(window)
    }

    fn create_canvas(
        &self,
        window: sdl2::video::Window,
    ) -> Result<sdl2::render::Canvas<sdl2::video::Window>, String> {
        let mut builder = window.into_canvas().accelerated();
        if self.vsync {
            builder = builder.present_vsync();
        }
        let canvas = builder.build().map_err(|e| e.to_string())?;
        Ok(canvas)
    }

    fn create_texture<'r>(
        &self,
        texture_creator: &'r TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<sdl2::render::Texture<'r>, String> {
        let (width, height) = self.get_resolution()?;

        let mut texture = texture_creator
            .create_texture_static(PixelFormatEnum::ARGB8888, width, height)
            .map_err(|e| e.to_string())?;

        texture.set_scale_mode(sdl2::render::ScaleMode::Best);
        Ok(texture)
    }
}
