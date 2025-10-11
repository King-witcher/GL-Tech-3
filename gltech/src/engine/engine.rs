use std::time::Instant;

use super::renderer;
use crate::{Image, Input, Scene, SysRequest, SystemContext};
use sdl2::{pixels::PixelFormatEnum, render::TextureCreator};

pub struct GLTechContext {
    borderless: bool,
    fullscreen: bool,
    resolution: Option<(u32, u32)>,
    sdl: sdl2::Sdl,
    title: String,
    video: sdl2::VideoSubsystem,
    vsync: bool,
}

pub fn init() -> Result<GLTechContext, String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    Ok(GLTechContext {
        borderless: false,
        fullscreen: false,
        resolution: None,
        sdl,
        title: "GLTech 3".into(),
        video,
        vsync: false,
    })
}

impl GLTechContext {
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
        let start_time = Instant::now();

        // Run start functions even before creating the window
        let mut system_context = SystemContext::new();
        scene.start(&mut system_context);
        if system_context.exit {
            return Ok(());
        }

        // Spawn the window and create the screen texture and gltech surface
        let mut canvas = self.spawn_window()?;
        let texture_creator = canvas.texture_creator();
        let mut screen_texture = self.get_screen_texture(&texture_creator)?;
        let (width, height) = self.get_resolution()?;
        let mut gltech_surface = crate::Image::new(width, height);

        // Get an event pump and start the main loop
        let mut event_pump = self.sdl.event_pump()?;

        // Main loop
        let mut frame_time = Instant::now();
        let mut input_handler = Input::new();
        loop {
            // Process any requests from the last frame, such as changing resolution or fullscreen
            self.process_requests(&mut system_context);

            // Render the scene to the surface
            let planes: Vec<&crate::Plane> = scene.planes().collect();
            renderer::draw_planes(&scene.camera, planes, &mut gltech_surface);

            // Present the surface on the screen
            Self::present(
                &mut canvas,
                &mut screen_texture,
                gltech_surface.cheap_clone(),
            )?;

            // Update input and check for exit event (usually window close)
            input_handler.update(event_pump.poll_iter());
            if input_handler.exit {
                break;
            }

            // Update the scene with input and time data
            let delta_time = frame_time.elapsed();
            frame_time = Instant::now();
            scene.update(
                input_handler.clone(),
                &mut system_context,
                start_time.elapsed(),
                delta_time,
            );

            // Check if any script requested exit
            if system_context.exit {
                break;
            }
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

    fn process_requests(&self, system_context: &mut SystemContext) {
        for request in system_context.take_requests() {
            match request {
                SysRequest::SetResolution(_, _) => todo!(),
                SysRequest::SetFullscreen(_) => todo!(),
                SysRequest::SetCaptureMouse(capture) => {
                    self.sdl.mouse().set_relative_mouse_mode(capture);
                }
                SysRequest::SetTitle(_) => todo!(),
                SysRequest::SetVSync(_) => todo!(),
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

    fn spawn_window(&self) -> Result<sdl2::render::Canvas<sdl2::video::Window>, String> {
        let window = self.create_window()?;
        let mut builder = window.into_canvas().accelerated();
        if self.vsync {
            builder = builder.present_vsync();
        }
        let canvas = builder.build().map_err(|e| e.to_string())?;
        Ok(canvas)
    }

    fn get_screen_texture<'r>(
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
