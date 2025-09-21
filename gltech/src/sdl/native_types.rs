use sdl2::sys::SDL_Keycode;

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
pub struct SDLRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
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
    pub scancode: ScanCode,
    #[doc = "< SDL virtual key code - see ::SDL_Keycode for details"]
    pub sym: SDL_Keycode,
    #[doc = "< current key modifiers"]
    pub mod_: u16,
    pub unused: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum ScanCode {
    SDL_SCANCODE_UNKNOWN = 0,
    SDL_SCANCODE_A = 4,
    SDL_SCANCODE_B = 5,
    SDL_SCANCODE_C = 6,
    SDL_SCANCODE_D = 7,
    SDL_SCANCODE_E = 8,
    SDL_SCANCODE_F = 9,
    SDL_SCANCODE_G = 10,
    SDL_SCANCODE_H = 11,
    SDL_SCANCODE_I = 12,
    SDL_SCANCODE_J = 13,
    SDL_SCANCODE_K = 14,
    SDL_SCANCODE_L = 15,
    SDL_SCANCODE_M = 16,
    SDL_SCANCODE_N = 17,
    SDL_SCANCODE_O = 18,
    SDL_SCANCODE_P = 19,
    SDL_SCANCODE_Q = 20,
    SDL_SCANCODE_R = 21,
    SDL_SCANCODE_S = 22,
    SDL_SCANCODE_T = 23,
    SDL_SCANCODE_U = 24,
    SDL_SCANCODE_V = 25,
    SDL_SCANCODE_W = 26,
    SDL_SCANCODE_X = 27,
    SDL_SCANCODE_Y = 28,
    SDL_SCANCODE_Z = 29,
    SDL_SCANCODE_1 = 30,
    SDL_SCANCODE_2 = 31,
    SDL_SCANCODE_3 = 32,
    SDL_SCANCODE_4 = 33,
    SDL_SCANCODE_5 = 34,
    SDL_SCANCODE_6 = 35,
    SDL_SCANCODE_7 = 36,
    SDL_SCANCODE_8 = 37,
    SDL_SCANCODE_9 = 38,
    SDL_SCANCODE_0 = 39,
    SDL_SCANCODE_RETURN = 40,
    SDL_SCANCODE_ESCAPE = 41,
    SDL_SCANCODE_BACKSPACE = 42,
    SDL_SCANCODE_TAB = 43,
    SDL_SCANCODE_SPACE = 44,
    SDL_SCANCODE_MINUS = 45,
    SDL_SCANCODE_EQUALS = 46,
    SDL_SCANCODE_LEFTBRACKET = 47,
    SDL_SCANCODE_RIGHTBRACKET = 48,
    #[doc = "< Located at the lower left of the return\n   key on ISO keyboards and at the right end\n   of the QWERTY row on ANSI keyboards.\n   Produces REVERSE SOLIDUS (backslash) and\n   VERTICAL LINE in a US layout, REVERSE\n   SOLIDUS and VERTICAL LINE in a UK Mac\n   layout, NUMBER SIGN and TILDE in a UK\n   Windows layout, DOLLAR SIGN and POUND SIGN\n   in a Swiss German layout, NUMBER SIGN and\n   APOSTROPHE in a German layout, GRAVE\n   ACCENT and POUND SIGN in a French Mac\n   layout, and ASTERISK and MICRO SIGN in a\n   French Windows layout."]
    SDL_SCANCODE_BACKSLASH = 49,
    #[doc = "< ISO USB keyboards actually use this code\n   instead of 49 for the same key, but all\n   OSes I've seen treat the two codes\n   identically. So, as an implementor, unless\n   your keyboard generates both of those\n   codes and your OS treats them differently,\n   you should generate SDL_SCANCODE_BACKSLASH\n   instead of this code. As a user, you\n   should not rely on this code because SDL\n   will never generate it with most (all?)\n   keyboards."]
    SDL_SCANCODE_NONUSHASH = 50,
    SDL_SCANCODE_SEMICOLON = 51,
    SDL_SCANCODE_APOSTROPHE = 52,
    #[doc = "< Located in the top left corner (on both ANSI\n   and ISO keyboards). Produces GRAVE ACCENT and\n   TILDE in a US Windows layout and in US and UK\n   Mac layouts on ANSI keyboards, GRAVE ACCENT\n   and NOT SIGN in a UK Windows layout, SECTION\n   SIGN and PLUS-MINUS SIGN in US and UK Mac\n   layouts on ISO keyboards, SECTION SIGN and\n   DEGREE SIGN in a Swiss German layout (Mac:\n   only on ISO keyboards), CIRCUMFLEX ACCENT and\n   DEGREE SIGN in a German layout (Mac: only on\n   ISO keyboards), SUPERSCRIPT TWO and TILDE in a\n   French Windows layout, COMMERCIAL AT and\n   NUMBER SIGN in a French Mac layout on ISO\n   keyboards, and LESS-THAN SIGN and GREATER-THAN\n   SIGN in a Swiss German, German, or French Mac\n   layout on ANSI keyboards."]
    SDL_SCANCODE_GRAVE = 53,
    SDL_SCANCODE_COMMA = 54,
    SDL_SCANCODE_PERIOD = 55,
    SDL_SCANCODE_SLASH = 56,
    SDL_SCANCODE_CAPSLOCK = 57,
    SDL_SCANCODE_F1 = 58,
    SDL_SCANCODE_F2 = 59,
    SDL_SCANCODE_F3 = 60,
    SDL_SCANCODE_F4 = 61,
    SDL_SCANCODE_F5 = 62,
    SDL_SCANCODE_F6 = 63,
    SDL_SCANCODE_F7 = 64,
    SDL_SCANCODE_F8 = 65,
    SDL_SCANCODE_F9 = 66,
    SDL_SCANCODE_F10 = 67,
    SDL_SCANCODE_F11 = 68,
    SDL_SCANCODE_F12 = 69,
    SDL_SCANCODE_PRINTSCREEN = 70,
    SDL_SCANCODE_SCROLLLOCK = 71,
    SDL_SCANCODE_PAUSE = 72,
    #[doc = "< insert on PC, help on some Mac keyboards (but\ndoes send code 73, not 117)"]
    SDL_SCANCODE_INSERT = 73,
    SDL_SCANCODE_HOME = 74,
    SDL_SCANCODE_PAGEUP = 75,
    SDL_SCANCODE_DELETE = 76,
    SDL_SCANCODE_END = 77,
    SDL_SCANCODE_PAGEDOWN = 78,
    SDL_SCANCODE_RIGHT = 79,
    SDL_SCANCODE_LEFT = 80,
    SDL_SCANCODE_DOWN = 81,
    SDL_SCANCODE_UP = 82,
    #[doc = "< num lock on PC, clear on Mac keyboards"]
    SDL_SCANCODE_NUMLOCKCLEAR = 83,
    SDL_SCANCODE_KP_DIVIDE = 84,
    SDL_SCANCODE_KP_MULTIPLY = 85,
    SDL_SCANCODE_KP_MINUS = 86,
    SDL_SCANCODE_KP_PLUS = 87,
    SDL_SCANCODE_KP_ENTER = 88,
    SDL_SCANCODE_KP_1 = 89,
    SDL_SCANCODE_KP_2 = 90,
    SDL_SCANCODE_KP_3 = 91,
    SDL_SCANCODE_KP_4 = 92,
    SDL_SCANCODE_KP_5 = 93,
    SDL_SCANCODE_KP_6 = 94,
    SDL_SCANCODE_KP_7 = 95,
    SDL_SCANCODE_KP_8 = 96,
    SDL_SCANCODE_KP_9 = 97,
    SDL_SCANCODE_KP_0 = 98,
    SDL_SCANCODE_KP_PERIOD = 99,
    #[doc = "< This is the additional key that ISO\n   keyboards have over ANSI ones,\n   located between left shift and Y.\n   Produces GRAVE ACCENT and TILDE in a\n   US or UK Mac layout, REVERSE SOLIDUS\n   (backslash) and VERTICAL LINE in a\n   US or UK Windows layout, and\n   LESS-THAN SIGN and GREATER-THAN SIGN\n   in a Swiss German, German, or French\n   layout."]
    SDL_SCANCODE_NONUSBACKSLASH = 100,
    #[doc = "< windows contextual menu, compose"]
    SDL_SCANCODE_APPLICATION = 101,
    #[doc = "< The USB document says this is a status flag,\n   not a physical key - but some Mac keyboards\n   do have a power key."]
    SDL_SCANCODE_POWER = 102,
    SDL_SCANCODE_KP_EQUALS = 103,
    SDL_SCANCODE_F13 = 104,
    SDL_SCANCODE_F14 = 105,
    SDL_SCANCODE_F15 = 106,
    SDL_SCANCODE_F16 = 107,
    SDL_SCANCODE_F17 = 108,
    SDL_SCANCODE_F18 = 109,
    SDL_SCANCODE_F19 = 110,
    SDL_SCANCODE_F20 = 111,
    SDL_SCANCODE_F21 = 112,
    SDL_SCANCODE_F22 = 113,
    SDL_SCANCODE_F23 = 114,
    SDL_SCANCODE_F24 = 115,
    SDL_SCANCODE_EXECUTE = 116,
    #[doc = "< AL Integrated Help Center"]
    SDL_SCANCODE_HELP = 117,
    #[doc = "< Menu (show menu)"]
    SDL_SCANCODE_MENU = 118,
    SDL_SCANCODE_SELECT = 119,
    #[doc = "< AC Stop"]
    SDL_SCANCODE_STOP = 120,
    #[doc = "< AC Redo/Repeat"]
    SDL_SCANCODE_AGAIN = 121,
    #[doc = "< AC Undo"]
    SDL_SCANCODE_UNDO = 122,
    #[doc = "< AC Cut"]
    SDL_SCANCODE_CUT = 123,
    #[doc = "< AC Copy"]
    SDL_SCANCODE_COPY = 124,
    #[doc = "< AC Paste"]
    SDL_SCANCODE_PASTE = 125,
    #[doc = "< AC Find"]
    SDL_SCANCODE_FIND = 126,
    SDL_SCANCODE_MUTE = 127,
    SDL_SCANCODE_VOLUMEUP = 128,
    SDL_SCANCODE_VOLUMEDOWN = 129,
    SDL_SCANCODE_KP_COMMA = 133,
    SDL_SCANCODE_KP_EQUALSAS400 = 134,
    #[doc = "< used on Asian keyboards, see\nfootnotes in USB doc"]
    SDL_SCANCODE_INTERNATIONAL1 = 135,
    SDL_SCANCODE_INTERNATIONAL2 = 136,
    #[doc = "< Yen"]
    SDL_SCANCODE_INTERNATIONAL3 = 137,
    SDL_SCANCODE_INTERNATIONAL4 = 138,
    SDL_SCANCODE_INTERNATIONAL5 = 139,
    SDL_SCANCODE_INTERNATIONAL6 = 140,
    SDL_SCANCODE_INTERNATIONAL7 = 141,
    SDL_SCANCODE_INTERNATIONAL8 = 142,
    SDL_SCANCODE_INTERNATIONAL9 = 143,
    #[doc = "< Hangul/English toggle"]
    SDL_SCANCODE_LANG1 = 144,
    #[doc = "< Hanja conversion"]
    SDL_SCANCODE_LANG2 = 145,
    #[doc = "< Katakana"]
    SDL_SCANCODE_LANG3 = 146,
    #[doc = "< Hiragana"]
    SDL_SCANCODE_LANG4 = 147,
    #[doc = "< Zenkaku/Hankaku"]
    SDL_SCANCODE_LANG5 = 148,
    #[doc = "< reserved"]
    SDL_SCANCODE_LANG6 = 149,
    #[doc = "< reserved"]
    SDL_SCANCODE_LANG7 = 150,
    #[doc = "< reserved"]
    SDL_SCANCODE_LANG8 = 151,
    #[doc = "< reserved"]
    SDL_SCANCODE_LANG9 = 152,
    #[doc = "< Erase-Eaze"]
    SDL_SCANCODE_ALTERASE = 153,
    SDL_SCANCODE_SYSREQ = 154,
    #[doc = "< AC Cancel"]
    SDL_SCANCODE_CANCEL = 155,
    SDL_SCANCODE_CLEAR = 156,
    SDL_SCANCODE_PRIOR = 157,
    SDL_SCANCODE_RETURN2 = 158,
    SDL_SCANCODE_SEPARATOR = 159,
    SDL_SCANCODE_OUT = 160,
    SDL_SCANCODE_OPER = 161,
    SDL_SCANCODE_CLEARAGAIN = 162,
    SDL_SCANCODE_CRSEL = 163,
    SDL_SCANCODE_EXSEL = 164,
    SDL_SCANCODE_KP_00 = 176,
    SDL_SCANCODE_KP_000 = 177,
    SDL_SCANCODE_THOUSANDSSEPARATOR = 178,
    SDL_SCANCODE_DECIMALSEPARATOR = 179,
    SDL_SCANCODE_CURRENCYUNIT = 180,
    SDL_SCANCODE_CURRENCYSUBUNIT = 181,
    SDL_SCANCODE_KP_LEFTPAREN = 182,
    SDL_SCANCODE_KP_RIGHTPAREN = 183,
    SDL_SCANCODE_KP_LEFTBRACE = 184,
    SDL_SCANCODE_KP_RIGHTBRACE = 185,
    SDL_SCANCODE_KP_TAB = 186,
    SDL_SCANCODE_KP_BACKSPACE = 187,
    SDL_SCANCODE_KP_A = 188,
    SDL_SCANCODE_KP_B = 189,
    SDL_SCANCODE_KP_C = 190,
    SDL_SCANCODE_KP_D = 191,
    SDL_SCANCODE_KP_E = 192,
    SDL_SCANCODE_KP_F = 193,
    SDL_SCANCODE_KP_XOR = 194,
    SDL_SCANCODE_KP_POWER = 195,
    SDL_SCANCODE_KP_PERCENT = 196,
    SDL_SCANCODE_KP_LESS = 197,
    SDL_SCANCODE_KP_GREATER = 198,
    SDL_SCANCODE_KP_AMPERSAND = 199,
    SDL_SCANCODE_KP_DBLAMPERSAND = 200,
    SDL_SCANCODE_KP_VERTICALBAR = 201,
    SDL_SCANCODE_KP_DBLVERTICALBAR = 202,
    SDL_SCANCODE_KP_COLON = 203,
    SDL_SCANCODE_KP_HASH = 204,
    SDL_SCANCODE_KP_SPACE = 205,
    SDL_SCANCODE_KP_AT = 206,
    SDL_SCANCODE_KP_EXCLAM = 207,
    SDL_SCANCODE_KP_MEMSTORE = 208,
    SDL_SCANCODE_KP_MEMRECALL = 209,
    SDL_SCANCODE_KP_MEMCLEAR = 210,
    SDL_SCANCODE_KP_MEMADD = 211,
    SDL_SCANCODE_KP_MEMSUBTRACT = 212,
    SDL_SCANCODE_KP_MEMMULTIPLY = 213,
    SDL_SCANCODE_KP_MEMDIVIDE = 214,
    SDL_SCANCODE_KP_PLUSMINUS = 215,
    SDL_SCANCODE_KP_CLEAR = 216,
    SDL_SCANCODE_KP_CLEARENTRY = 217,
    SDL_SCANCODE_KP_BINARY = 218,
    SDL_SCANCODE_KP_OCTAL = 219,
    SDL_SCANCODE_KP_DECIMAL = 220,
    SDL_SCANCODE_KP_HEXADECIMAL = 221,
    SDL_SCANCODE_LCTRL = 224,
    SDL_SCANCODE_LSHIFT = 225,
    #[doc = "< alt, option"]
    SDL_SCANCODE_LALT = 226,
    #[doc = "< windows, command (apple), meta"]
    SDL_SCANCODE_LGUI = 227,
    SDL_SCANCODE_RCTRL = 228,
    SDL_SCANCODE_RSHIFT = 229,
    #[doc = "< alt gr, option"]
    SDL_SCANCODE_RALT = 230,
    #[doc = "< windows, command (apple), meta"]
    SDL_SCANCODE_RGUI = 231,
    #[doc = "< I'm not sure if this is really not covered\n   by any of the above, but since there's a\n   special KMOD_MODE for it I'm adding it here"]
    SDL_SCANCODE_MODE = 257,
    SDL_SCANCODE_AUDIONEXT = 258,
    SDL_SCANCODE_AUDIOPREV = 259,
    SDL_SCANCODE_AUDIOSTOP = 260,
    SDL_SCANCODE_AUDIOPLAY = 261,
    SDL_SCANCODE_AUDIOMUTE = 262,
    SDL_SCANCODE_MEDIASELECT = 263,
    #[doc = "< AL Internet Browser"]
    SDL_SCANCODE_WWW = 264,
    SDL_SCANCODE_MAIL = 265,
    #[doc = "< AL Calculator"]
    SDL_SCANCODE_CALCULATOR = 266,
    SDL_SCANCODE_COMPUTER = 267,
    #[doc = "< AC Search"]
    SDL_SCANCODE_AC_SEARCH = 268,
    #[doc = "< AC Home"]
    SDL_SCANCODE_AC_HOME = 269,
    #[doc = "< AC Back"]
    SDL_SCANCODE_AC_BACK = 270,
    #[doc = "< AC Forward"]
    SDL_SCANCODE_AC_FORWARD = 271,
    #[doc = "< AC Stop"]
    SDL_SCANCODE_AC_STOP = 272,
    #[doc = "< AC Refresh"]
    SDL_SCANCODE_AC_REFRESH = 273,
    #[doc = "< AC Bookmarks"]
    SDL_SCANCODE_AC_BOOKMARKS = 274,
    SDL_SCANCODE_BRIGHTNESSDOWN = 275,
    SDL_SCANCODE_BRIGHTNESSUP = 276,
    #[doc = "< display mirroring/dual display\nswitch, video mode switch"]
    SDL_SCANCODE_DISPLAYSWITCH = 277,
    SDL_SCANCODE_KBDILLUMTOGGLE = 278,
    SDL_SCANCODE_KBDILLUMDOWN = 279,
    SDL_SCANCODE_KBDILLUMUP = 280,
    SDL_SCANCODE_EJECT = 281,
    #[doc = "< SC System Sleep"]
    SDL_SCANCODE_SLEEP = 282,
    SDL_SCANCODE_APP1 = 283,
    SDL_SCANCODE_APP2 = 284,
    SDL_SCANCODE_AUDIOREWIND = 285,
    SDL_SCANCODE_AUDIOFASTFORWARD = 286,
    #[doc = "< Usually situated below the display on phones and\nused as a multi-function feature key for selecting\na software defined function shown on the bottom left\nof the display."]
    SDL_SCANCODE_SOFTLEFT = 287,
    #[doc = "< Usually situated below the display on phones and\nused as a multi-function feature key for selecting\na software defined function shown on the bottom right\nof the display."]
    SDL_SCANCODE_SOFTRIGHT = 288,
    #[doc = "< Used for accepting phone calls."]
    SDL_SCANCODE_CALL = 289,
    #[doc = "< Used for rejecting phone calls."]
    SDL_SCANCODE_ENDCALL = 290,
    #[doc = "< not a key, just marks the number of scancodes\nfor array bounds"]
    SDL_NUM_SCANCODES = 512,
}
