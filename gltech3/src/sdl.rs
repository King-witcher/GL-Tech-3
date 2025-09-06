use sdl2::sys::{SDL_Keycode, SDL_Scancode};

unsafe extern "C" {
    pub unsafe fn SDL_CreateRenderer(window: usize, index: i32, flags: u32) -> usize;
    pub unsafe fn SDL_CreateTexture(
        renderer: usize,
        format: SDLPixelFormat,
        access: SDLTextureAccess,
        w: i32,
        h: i32,
    ) -> usize;
    pub unsafe fn SDL_CreateWindow(
        title: *const u8,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> usize;
    pub unsafe fn SDL_DestroyRenderer(renderer: usize);
    pub unsafe fn SDL_DestroyTexture(texture: usize);
    pub unsafe fn SDL_DestroyWindow(window: usize);
    pub unsafe fn SDL_GetError() -> *const u8;
    pub unsafe fn SDL_GetRelativeMouseState(x: &mut i32, y: &mut i32) -> i32;
    pub unsafe fn SDL_PollEvent(event: &mut SDLEvent) -> bool;
    pub unsafe fn SDL_RenderClear(renderer: usize) -> i32;
    pub unsafe fn SDL_RenderCopy(
        renderer: usize,
        texture: usize,
        srcrect: Option<&SDLRect>,
        dstrect: Option<&SDLRect>,
    ) -> i32;
    pub unsafe fn SDL_RenderPresent(renderer: usize);
    pub unsafe fn SDL_UpdateTexture(
        texture: usize,
        rect: Option<&SDLRect>,
        pixels: *const u32,
        pitch: i32,
    ) -> i32;
}

fn get_string(src: *const u8) -> String {
    unsafe {
        let mut length = 0usize;
        while *src.add(length) != 0 {
            length += 1;
        }

        let layout = std::alloc::Layout::array::<u8>(length).unwrap();
        let dst = std::alloc::alloc(layout);
        std::ptr::copy::<u8>(src, dst, length);
        String::from_raw_parts(dst, length, length)
    }
}

pub fn get_last_error() -> String {
    let error = unsafe { SDL_GetError() };
    get_string(error)
}

pub const SDL_WINDOWPOS_UNDEFINED: i32 = 536805376;
pub const SDL_WINDOWPOS_CENTERED: i32 = 805240832;

#[doc = "fullscreen window"]
pub const SDL_WINDOW_FULLSCREEN: u32 = 1;
#[doc = "window usable with OpenGL context"]
pub const SDL_WINDOW_OPENGL: u32 = 2;
#[doc = "window is visible"]
pub const SDL_WINDOW_SHOWN: u32 = 4;
#[doc = "window is not visible"]
pub const SDL_WINDOW_HIDDEN: u32 = 8;
#[doc = "no window decoration"]
pub const SDL_WINDOW_BORDERLESS: u32 = 16;
#[doc = "window can be resized"]
pub const SDL_WINDOW_RESIZABLE: u32 = 32;
#[doc = "window is minimized"]
pub const SDL_WINDOW_MINIMIZED: u32 = 64;
#[doc = "window is maximized"]
pub const SDL_WINDOW_MAXIMIZED: u32 = 128;
#[doc = "window has grabbed mouse input"]
pub const SDL_WINDOW_MOUSE_GRABBED: u32 = 256;
#[doc = "window has input focus"]
pub const SDL_WINDOW_INPUT_FOCUS: u32 = 512;
#[doc = "window has mouse focus"]
pub const SDL_WINDOW_MOUSE_FOCUS: u32 = 1024;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = 4097;
#[doc = "window not created by SDL"]
pub const SDL_WINDOW_FOREIGN: u32 = 2048;
#[doc = "window should be created in high-DPI mode if supported.\nOn macOS NSHighResolutionCapable must be set true in the\napplication's Info.plist for this tohave any effect."]
pub const SDL_WINDOW_ALLOW_HIGHDPI: u32 = 8192;
#[doc = "window has mouse captured (unrelated to MOUSE_GRABBED)"]
pub const SDL_WINDOW_MOUSE_CAPTURE: u32 = 16384;
#[doc = "window should always be above others"]
pub const SDL_WINDOW_ALWAYS_ON_TOP: u32 = 32768;
#[doc = "window should not be added to the taskbar"]
pub const SDL_WINDOW_SKIP_TASKBAR: u32 = 65536;
#[doc = "window should be treated as a utility window"]
pub const SDL_WINDOW_UTILITY: u32 = 131072;
#[doc = "window should be treated as a tooltip"]
pub const SDL_WINDOW_TOOLTIP: u32 = 262144;
#[doc = "window should be treated as a popup menu"]
pub const SDL_WINDOW_POPUP_MENU: u32 = 524288;
#[doc = "window has grabbed keyboard input"]
pub const SDL_WINDOW_KEYBOARD_GRABBED: u32 = 1048576;
#[doc = "window usable for Vulkan surface"]
pub const SDL_WINDOW_VULKAN: u32 = 268435456;
#[doc = "window usable for Metal view"]
pub const SDL_WINDOW_METAL: u32 = 536870912;
#[doc = "The renderer is a software fallback"]
pub const SDL_RENDERER_SOFTWARE: u32 = 1;
#[doc = "The renderer uses hardware\nacceleration"]
pub const SDL_RENDERER_ACCELERATED: u32 = 2;
#[doc = "Present is synchronized\nwith the refresh rate"]
pub const SDL_RENDERER_PRESENTVSYNC: u32 = 4;
#[doc = "The renderer supports\nrendering to texture"]
pub const SDL_RENDERER_TARGETTEXTURE: u32 = 8;

#[repr(C)]
pub enum SDLPixelFormat {
    Unknown = 0,
    Index1LSB = 286261504,
    Index1MSB = 287310080,
    Index4LSB = 303039488,
    Index4MSB = 304088064,
    Index8 = 318769153,
    RGB332 = 336660481,
    XRGB4444 = 353504258,
    XBGR4444 = 357698562,
    XRGB1555 = 353570562,
    XBGR1555 = 357764866,
    ARGB4444 = 355602434,
    RGBA4444 = 356651010,
    ABGR4444 = 359796738,
    BGRA4444 = 360845314,
    ARGB1555 = 355667970,
    RGBA5551 = 356782082,
    ABGR1555 = 359862274,
    BGRA5551 = 360976386,
    RGB565 = 353701890,
    BGR565 = 357896194,
    RGB24 = 386930691,
    BGR24 = 390076419,
    XRGB8888 = 370546692,
    RGBX8888 = 371595268,
    XBGR8888 = 374740996,
    BGRX8888 = 375789572,
    ARGB8888 = 372645892,
    RGBA8888 = 373694468,
    ABGR8888 = 376840196,
    BGRA8888 = 377888772,
    ARGB2101010 = 372711428,
    YV12 = 842094169,
    IYUV = 1448433993,
    YUY2 = 844715353,
    UYVY = 1498831189,
    YVYU = 1431918169,
    NV12 = 842094158,
    NV21 = 825382478,
    ExternalOes = 542328143,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SDLTextureAccess {
    Static = 0,
    Streaming = 1,
    Target = 2,
    DoNotUse = 0x10000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDLRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(u32)]
pub enum SDLEvent {
    #[doc = "User-requested quit"]
    FirstEvent = 0,
    Quit(SDLQuitEvent) = 256,
    KeyDown(SDLKeyboardEvent) = 768,
    KeyUp(SDLKeyboardEvent) = 769,
}

impl Default for SDLEvent {
    fn default() -> Self {
        Self::FirstEvent
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDLQuitEvent {
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDLKeyboardEvent {
    #[doc = "In milliseconds, populated using SDL_GetTicks()"]
    pub timestamp: u32,
    #[doc = "The window with keyboard focus, if any"]
    pub window_id: u32,
    #[doc = "::SDL_PRESSED or ::SDL_RELEASED"]
    pub state: u8,
    #[doc = "Non-zero if this is a key repeat"]
    pub repeat: u8,
    pub padding2: u8,
    pub padding3: u8,
    #[doc = "The key that was pressed or released"]
    pub keysym: SDLKeysym,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDLKeysym {
    #[doc = "< SDL physical key code - see ::SDL_Scancode for details"]
    pub scancode: SDL_Scancode,
    #[doc = "< SDL virtual key code - see ::SDL_Keycode for details"]
    pub sym: SDL_Keycode,
    #[doc = "< current key modifiers"]
    pub mod_: u16,
    pub unused: u32,
}
