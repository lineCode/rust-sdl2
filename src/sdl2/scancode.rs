use num::{ToPrimitive, FromPrimitive};
use std::ffi::{CString, CStr};

use sys::scancode as ll;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ScanCode {
    Unknown            = ll::SDL_SCANCODE_UNKNOWN as isize,
    A                  = ll::SDL_SCANCODE_A as isize,
    B                  = ll::SDL_SCANCODE_B as isize,
    C                  = ll::SDL_SCANCODE_C as isize,
    D                  = ll::SDL_SCANCODE_D as isize,
    E                  = ll::SDL_SCANCODE_E as isize,
    F                  = ll::SDL_SCANCODE_F as isize,
    G                  = ll::SDL_SCANCODE_G as isize,
    H                  = ll::SDL_SCANCODE_H as isize,
    I                  = ll::SDL_SCANCODE_I as isize,
    J                  = ll::SDL_SCANCODE_J as isize,
    K                  = ll::SDL_SCANCODE_K as isize,
    L                  = ll::SDL_SCANCODE_L as isize,
    M                  = ll::SDL_SCANCODE_M as isize,
    N                  = ll::SDL_SCANCODE_N as isize,
    O                  = ll::SDL_SCANCODE_O as isize,
    P                  = ll::SDL_SCANCODE_P as isize,
    Q                  = ll::SDL_SCANCODE_Q as isize,
    R                  = ll::SDL_SCANCODE_R as isize,
    S                  = ll::SDL_SCANCODE_S as isize,
    T                  = ll::SDL_SCANCODE_T as isize,
    U                  = ll::SDL_SCANCODE_U as isize,
    V                  = ll::SDL_SCANCODE_V as isize,
    W                  = ll::SDL_SCANCODE_W as isize,
    X                  = ll::SDL_SCANCODE_X as isize,
    Y                  = ll::SDL_SCANCODE_Y as isize,
    Z                  = ll::SDL_SCANCODE_Z as isize,
    Num1               = ll::SDL_SCANCODE_1 as isize,
    Num2               = ll::SDL_SCANCODE_2 as isize,
    Num3               = ll::SDL_SCANCODE_3 as isize,
    Num4               = ll::SDL_SCANCODE_4 as isize,
    Num5               = ll::SDL_SCANCODE_5 as isize,
    Num6               = ll::SDL_SCANCODE_6 as isize,
    Num7               = ll::SDL_SCANCODE_7 as isize,
    Num8               = ll::SDL_SCANCODE_8 as isize,
    Num9               = ll::SDL_SCANCODE_9 as isize,
    Num0               = ll::SDL_SCANCODE_0 as isize,
    Return             = ll::SDL_SCANCODE_RETURN as isize,
    Escape             = ll::SDL_SCANCODE_ESCAPE as isize,
    Backspace          = ll::SDL_SCANCODE_BACKSPACE as isize,
    Tab                = ll::SDL_SCANCODE_TAB as isize,
    Space              = ll::SDL_SCANCODE_SPACE as isize,
    Minus              = ll::SDL_SCANCODE_MINUS as isize,
    Equals             = ll::SDL_SCANCODE_EQUALS as isize,
    LeftBracket        = ll::SDL_SCANCODE_LEFTBRACKET as isize,
    RightBracket       = ll::SDL_SCANCODE_RIGHTBRACKET as isize,
    Backslash          = ll::SDL_SCANCODE_BACKSLASH as isize,
    NonUsHash          = ll::SDL_SCANCODE_NONUSHASH as isize,
    Semicolon          = ll::SDL_SCANCODE_SEMICOLON as isize,
    Apostrophe         = ll::SDL_SCANCODE_APOSTROPHE as isize,
    Grave              = ll::SDL_SCANCODE_GRAVE as isize,
    Comma              = ll::SDL_SCANCODE_COMMA as isize,
    Period             = ll::SDL_SCANCODE_PERIOD as isize,
    Slash              = ll::SDL_SCANCODE_SLASH as isize,
    CapsLock           = ll::SDL_SCANCODE_CAPSLOCK as isize,
    F1                 = ll::SDL_SCANCODE_F1 as isize,
    F2                 = ll::SDL_SCANCODE_F2 as isize,
    F3                 = ll::SDL_SCANCODE_F3 as isize,
    F4                 = ll::SDL_SCANCODE_F4 as isize,
    F5                 = ll::SDL_SCANCODE_F5 as isize,
    F6                 = ll::SDL_SCANCODE_F6 as isize,
    F7                 = ll::SDL_SCANCODE_F7 as isize,
    F8                 = ll::SDL_SCANCODE_F8 as isize,
    F9                 = ll::SDL_SCANCODE_F9 as isize,
    F10                = ll::SDL_SCANCODE_F10 as isize,
    F11                = ll::SDL_SCANCODE_F11 as isize,
    F12                = ll::SDL_SCANCODE_F12 as isize,
    PrintScreen        = ll::SDL_SCANCODE_PRINTSCREEN as isize,
    ScrollLock         = ll::SDL_SCANCODE_SCROLLLOCK as isize,
    Pause              = ll::SDL_SCANCODE_PAUSE as isize,
    Insert             = ll::SDL_SCANCODE_INSERT as isize,
    Home               = ll::SDL_SCANCODE_HOME as isize,
    PageUp             = ll::SDL_SCANCODE_PAGEUP as isize,
    Delete             = ll::SDL_SCANCODE_DELETE as isize,
    End                = ll::SDL_SCANCODE_END as isize,
    PageDown           = ll::SDL_SCANCODE_PAGEDOWN as isize,
    Right              = ll::SDL_SCANCODE_RIGHT as isize,
    Left               = ll::SDL_SCANCODE_LEFT as isize,
    Down               = ll::SDL_SCANCODE_DOWN as isize,
    Up                 = ll::SDL_SCANCODE_UP as isize,
    NumLockClear       = ll::SDL_SCANCODE_NUMLOCKCLEAR as isize,
    KpDivide           = ll::SDL_SCANCODE_KP_DIVIDE as isize,
    KpMultiply         = ll::SDL_SCANCODE_KP_MULTIPLY as isize,
    KpMinus            = ll::SDL_SCANCODE_KP_MINUS as isize,
    KpPlus             = ll::SDL_SCANCODE_KP_PLUS as isize,
    KpEnter            = ll::SDL_SCANCODE_KP_ENTER as isize,
    Kp1                = ll::SDL_SCANCODE_KP_1 as isize,
    Kp2                = ll::SDL_SCANCODE_KP_2 as isize,
    Kp3                = ll::SDL_SCANCODE_KP_3 as isize,
    Kp4                = ll::SDL_SCANCODE_KP_4 as isize,
    Kp5                = ll::SDL_SCANCODE_KP_5 as isize,
    Kp6                = ll::SDL_SCANCODE_KP_6 as isize,
    Kp7                = ll::SDL_SCANCODE_KP_7 as isize,
    Kp8                = ll::SDL_SCANCODE_KP_8 as isize,
    Kp9                = ll::SDL_SCANCODE_KP_9 as isize,
    Kp0                = ll::SDL_SCANCODE_KP_0 as isize,
    KpPeriod           = ll::SDL_SCANCODE_KP_PERIOD as isize,
    NonUsBackslash     = ll::SDL_SCANCODE_NONUSBACKSLASH as isize,
    Application        = ll::SDL_SCANCODE_APPLICATION as isize,
    Power              = ll::SDL_SCANCODE_POWER as isize,
    KpEquals           = ll::SDL_SCANCODE_KP_EQUALS as isize,
    F13                = ll::SDL_SCANCODE_F13 as isize,
    F14                = ll::SDL_SCANCODE_F14 as isize,
    F15                = ll::SDL_SCANCODE_F15 as isize,
    F16                = ll::SDL_SCANCODE_F16 as isize,
    F17                = ll::SDL_SCANCODE_F17 as isize,
    F18                = ll::SDL_SCANCODE_F18 as isize,
    F19                = ll::SDL_SCANCODE_F19 as isize,
    F20                = ll::SDL_SCANCODE_F20 as isize,
    F21                = ll::SDL_SCANCODE_F21 as isize,
    F22                = ll::SDL_SCANCODE_F22 as isize,
    F23                = ll::SDL_SCANCODE_F23 as isize,
    F24                = ll::SDL_SCANCODE_F24 as isize,
    Execute            = ll::SDL_SCANCODE_EXECUTE as isize,
    Help               = ll::SDL_SCANCODE_HELP as isize,
    Menu               = ll::SDL_SCANCODE_MENU as isize,
    Select             = ll::SDL_SCANCODE_SELECT as isize,
    Stop               = ll::SDL_SCANCODE_STOP as isize,
    Again              = ll::SDL_SCANCODE_AGAIN as isize,
    Undo               = ll::SDL_SCANCODE_UNDO as isize,
    Cut                = ll::SDL_SCANCODE_CUT as isize,
    Copy               = ll::SDL_SCANCODE_COPY as isize,
    Paste              = ll::SDL_SCANCODE_PASTE as isize,
    Find               = ll::SDL_SCANCODE_FIND as isize,
    Mute               = ll::SDL_SCANCODE_MUTE as isize,
    VolumeUp           = ll::SDL_SCANCODE_VOLUMEUP as isize,
    VolumeDown         = ll::SDL_SCANCODE_VOLUMEDOWN as isize,
    KpComma            = ll::SDL_SCANCODE_KP_COMMA as isize,
    KpEqualsAS400      = ll::SDL_SCANCODE_KP_EQUALSAS400 as isize,
    International1     = ll::SDL_SCANCODE_INTERNATIONAL1 as isize,
    International2     = ll::SDL_SCANCODE_INTERNATIONAL2 as isize,
    International3     = ll::SDL_SCANCODE_INTERNATIONAL3 as isize,
    International4     = ll::SDL_SCANCODE_INTERNATIONAL4 as isize,
    International5     = ll::SDL_SCANCODE_INTERNATIONAL5 as isize,
    International6     = ll::SDL_SCANCODE_INTERNATIONAL6 as isize,
    International7     = ll::SDL_SCANCODE_INTERNATIONAL7 as isize,
    International8     = ll::SDL_SCANCODE_INTERNATIONAL8 as isize,
    International9     = ll::SDL_SCANCODE_INTERNATIONAL9 as isize,
    Lang1              = ll::SDL_SCANCODE_LANG1 as isize,
    Lang2              = ll::SDL_SCANCODE_LANG2 as isize,
    Lang3              = ll::SDL_SCANCODE_LANG3 as isize,
    Lang4              = ll::SDL_SCANCODE_LANG4 as isize,
    Lang5              = ll::SDL_SCANCODE_LANG5 as isize,
    Lang6              = ll::SDL_SCANCODE_LANG6 as isize,
    Lang7              = ll::SDL_SCANCODE_LANG7 as isize,
    Lang8              = ll::SDL_SCANCODE_LANG8 as isize,
    Lang9              = ll::SDL_SCANCODE_LANG9 as isize,
    AltErase           = ll::SDL_SCANCODE_ALTERASE as isize,
    SysReq             = ll::SDL_SCANCODE_SYSREQ as isize,
    Cancel             = ll::SDL_SCANCODE_CANCEL as isize,
    Clear              = ll::SDL_SCANCODE_CLEAR as isize,
    Prior              = ll::SDL_SCANCODE_PRIOR as isize,
    Return2            = ll::SDL_SCANCODE_RETURN2 as isize,
    Separator          = ll::SDL_SCANCODE_SEPARATOR as isize,
    Out                = ll::SDL_SCANCODE_OUT as isize,
    Oper               = ll::SDL_SCANCODE_OPER as isize,
    ClearAgain         = ll::SDL_SCANCODE_CLEARAGAIN as isize,
    Crse               = ll::SDL_SCANCODE_CRSEL as isize,
    ExseL              = ll::SDL_SCANCODE_EXSEL as isize,
    Kp00               = ll::SDL_SCANCODE_KP_00 as isize,
    Kp000              = ll::SDL_SCANCODE_KP_000 as isize,
    ThousandsSeparator = ll::SDL_SCANCODE_THOUSANDSSEPARATOR as isize,
    DecimalSeparator   = ll::SDL_SCANCODE_DECIMALSEPARATOR as isize,
    CurrencyUnit       = ll::SDL_SCANCODE_CURRENCYUNIT as isize,
    CurrencySubUnit    = ll::SDL_SCANCODE_CURRENCYSUBUNIT as isize,
    KpLeftParen        = ll::SDL_SCANCODE_KP_LEFTPAREN as isize,
    KpRightParen       = ll::SDL_SCANCODE_KP_RIGHTPAREN as isize,
    KpLeftBrace        = ll::SDL_SCANCODE_KP_LEFTBRACE as isize,
    KpRightBrace       = ll::SDL_SCANCODE_KP_RIGHTBRACE as isize,
    KpTab              = ll::SDL_SCANCODE_KP_TAB as isize,
    KpBackspace        = ll::SDL_SCANCODE_KP_BACKSPACE as isize,
    KpA                = ll::SDL_SCANCODE_KP_A as isize,
    KpB                = ll::SDL_SCANCODE_KP_B as isize,
    KpC                = ll::SDL_SCANCODE_KP_C as isize,
    KpD                = ll::SDL_SCANCODE_KP_D as isize,
    KpE                = ll::SDL_SCANCODE_KP_E as isize,
    KpF                = ll::SDL_SCANCODE_KP_F as isize,
    KpXor              = ll::SDL_SCANCODE_KP_XOR as isize,
    KpPower            = ll::SDL_SCANCODE_KP_POWER as isize,
    KpPercent          = ll::SDL_SCANCODE_KP_PERCENT as isize,
    KpLess             = ll::SDL_SCANCODE_KP_LESS as isize,
    KpGreater          = ll::SDL_SCANCODE_KP_GREATER as isize,
    KpAmpersand        = ll::SDL_SCANCODE_KP_AMPERSAND as isize,
    KpDblAmpersand     = ll::SDL_SCANCODE_KP_DBLAMPERSAND as isize,
    KpVerticalBar      = ll::SDL_SCANCODE_KP_VERTICALBAR as isize,
    KpDblVerticalBar   = ll::SDL_SCANCODE_KP_DBLVERTICALBAR as isize,
    KpColon            = ll::SDL_SCANCODE_KP_COLON as isize,
    KpHash             = ll::SDL_SCANCODE_KP_HASH as isize,
    KpSpace            = ll::SDL_SCANCODE_KP_SPACE as isize,
    KpAt               = ll::SDL_SCANCODE_KP_AT as isize,
    KpExclam           = ll::SDL_SCANCODE_KP_EXCLAM as isize,
    KpMemStore         = ll::SDL_SCANCODE_KP_MEMSTORE as isize,
    KpMemRecall        = ll::SDL_SCANCODE_KP_MEMRECALL as isize,
    KpMemClear         = ll::SDL_SCANCODE_KP_MEMCLEAR as isize,
    KpMemAdd           = ll::SDL_SCANCODE_KP_MEMADD as isize,
    KpMemSubtract      = ll::SDL_SCANCODE_KP_MEMSUBTRACT as isize,
    KpMemMultiply      = ll::SDL_SCANCODE_KP_MEMMULTIPLY as isize,
    KpMemDivide        = ll::SDL_SCANCODE_KP_MEMDIVIDE as isize,
    KpPlusMinus        = ll::SDL_SCANCODE_KP_PLUSMINUS as isize,
    KpClear            = ll::SDL_SCANCODE_KP_CLEAR as isize,
    KpClearEntry       = ll::SDL_SCANCODE_KP_CLEARENTRY as isize,
    KpBinary           = ll::SDL_SCANCODE_KP_BINARY as isize,
    KpOoctal           = ll::SDL_SCANCODE_KP_OCTAL as isize,
    KpDecimal          = ll::SDL_SCANCODE_KP_DECIMAL as isize,
    KpHexadecimal      = ll::SDL_SCANCODE_KP_HEXADECIMAL as isize,
    LCtrl              = ll::SDL_SCANCODE_LCTRL as isize,
    LShift             = ll::SDL_SCANCODE_LSHIFT as isize,
    LAlt               = ll::SDL_SCANCODE_LALT as isize,
    LGui               = ll::SDL_SCANCODE_LGUI as isize,
    RCtrl              = ll::SDL_SCANCODE_RCTRL as isize,
    RShift             = ll::SDL_SCANCODE_RSHIFT as isize,
    RAlt               = ll::SDL_SCANCODE_RALT as isize,
    RGui               = ll::SDL_SCANCODE_RGUI as isize,
    Mode               = ll::SDL_SCANCODE_MODE as isize,
    AudioNext          = ll::SDL_SCANCODE_AUDIONEXT as isize,
    AudioPrev          = ll::SDL_SCANCODE_AUDIOPREV as isize,
    AudioStop          = ll::SDL_SCANCODE_AUDIOSTOP as isize,
    AudioPlay          = ll::SDL_SCANCODE_AUDIOPLAY as isize,
    AudioMute          = ll::SDL_SCANCODE_AUDIOMUTE as isize,
    MediaSelect        = ll::SDL_SCANCODE_MEDIASELECT as isize,
    Www                = ll::SDL_SCANCODE_WWW as isize,
    Mail               = ll::SDL_SCANCODE_MAIL as isize,
    Calculator         = ll::SDL_SCANCODE_CALCULATOR as isize,
    Computer           = ll::SDL_SCANCODE_COMPUTER as isize,
    AcSearch           = ll::SDL_SCANCODE_AC_SEARCH as isize,
    AcHome             = ll::SDL_SCANCODE_AC_HOME as isize,
    AcBack             = ll::SDL_SCANCODE_AC_BACK as isize,
    AcForward          = ll::SDL_SCANCODE_AC_FORWARD as isize,
    AcStop             = ll::SDL_SCANCODE_AC_STOP as isize,
    AcRefresh          = ll::SDL_SCANCODE_AC_REFRESH as isize,
    AcBookmarks        = ll::SDL_SCANCODE_AC_BOOKMARKS as isize,
    BrightnessDown     = ll::SDL_SCANCODE_BRIGHTNESSDOWN as isize,
    BrightnessUp       = ll::SDL_SCANCODE_BRIGHTNESSUP as isize,
    DisplaySwitch      = ll::SDL_SCANCODE_DISPLAYSWITCH as isize,
    KbdIllumToggle     = ll::SDL_SCANCODE_KBDILLUMTOGGLE as isize,
    KbdIllumDown       = ll::SDL_SCANCODE_KBDILLUMDOWN as isize,
    KbdIllumUp         = ll::SDL_SCANCODE_KBDILLUMUP as isize,
    Eject              = ll::SDL_SCANCODE_EJECT as isize,
    Sleep              = ll::SDL_SCANCODE_SLEEP as isize,
    App1               = ll::SDL_SCANCODE_APP1 as isize,
    App2               = ll::SDL_SCANCODE_APP2 as isize,
    Num                = ll::SDL_NUM_SCANCODES as isize,
}

impl ToPrimitive for ScanCode {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }

    #[inline]
    fn to_isize(&self) -> Option<isize> {
        Some(*self as isize)
    }
}

impl FromPrimitive for ScanCode {
    fn from_i64(n: i64) -> Option<ScanCode> {
        use self::ScanCode::*;

        Some( match n as ll::SDL_Scancode {
            ll::SDL_SCANCODE_UNKNOWN            => Unknown,
            ll::SDL_SCANCODE_A                  => A,
            ll::SDL_SCANCODE_B                  => B,
            ll::SDL_SCANCODE_C                  => C,
            ll::SDL_SCANCODE_D                  => D,
            ll::SDL_SCANCODE_E                  => E,
            ll::SDL_SCANCODE_F                  => F,
            ll::SDL_SCANCODE_G                  => G,
            ll::SDL_SCANCODE_H                  => H,
            ll::SDL_SCANCODE_I                  => I,
            ll::SDL_SCANCODE_J                  => J,
            ll::SDL_SCANCODE_K                  => K,
            ll::SDL_SCANCODE_L                  => L,
            ll::SDL_SCANCODE_M                  => M,
            ll::SDL_SCANCODE_N                  => N,
            ll::SDL_SCANCODE_O                  => O,
            ll::SDL_SCANCODE_P                  => P,
            ll::SDL_SCANCODE_Q                  => Q,
            ll::SDL_SCANCODE_R                  => R,
            ll::SDL_SCANCODE_S                  => S,
            ll::SDL_SCANCODE_T                  => T,
            ll::SDL_SCANCODE_U                  => U,
            ll::SDL_SCANCODE_V                  => V,
            ll::SDL_SCANCODE_W                  => W,
            ll::SDL_SCANCODE_X                  => X,
            ll::SDL_SCANCODE_Y                  => Y,
            ll::SDL_SCANCODE_Z                  => Z,
            ll::SDL_SCANCODE_1                  => Num1,
            ll::SDL_SCANCODE_2                  => Num2,
            ll::SDL_SCANCODE_3                  => Num3,
            ll::SDL_SCANCODE_4                  => Num4,
            ll::SDL_SCANCODE_5                  => Num5,
            ll::SDL_SCANCODE_6                  => Num6,
            ll::SDL_SCANCODE_7                  => Num7,
            ll::SDL_SCANCODE_8                  => Num8,
            ll::SDL_SCANCODE_9                  => Num9,
            ll::SDL_SCANCODE_0                  => Num0,
            ll::SDL_SCANCODE_RETURN             => Return,
            ll::SDL_SCANCODE_ESCAPE             => Escape,
            ll::SDL_SCANCODE_BACKSPACE          => Backspace,
            ll::SDL_SCANCODE_TAB                => Tab,
            ll::SDL_SCANCODE_SPACE              => Space,
            ll::SDL_SCANCODE_MINUS              => Minus,
            ll::SDL_SCANCODE_EQUALS             => Equals,
            ll::SDL_SCANCODE_LEFTBRACKET        => LeftBracket,
            ll::SDL_SCANCODE_RIGHTBRACKET       => RightBracket,
            ll::SDL_SCANCODE_BACKSLASH          => Backslash,
            ll::SDL_SCANCODE_NONUSHASH          => NonUsHash,
            ll::SDL_SCANCODE_SEMICOLON          => Semicolon,
            ll::SDL_SCANCODE_APOSTROPHE         => Apostrophe,
            ll::SDL_SCANCODE_GRAVE              => Grave,
            ll::SDL_SCANCODE_COMMA              => Comma,
            ll::SDL_SCANCODE_PERIOD             => Period,
            ll::SDL_SCANCODE_SLASH              => Slash,
            ll::SDL_SCANCODE_CAPSLOCK           => CapsLock,
            ll::SDL_SCANCODE_F1                 => F1,
            ll::SDL_SCANCODE_F2                 => F2,
            ll::SDL_SCANCODE_F3                 => F3,
            ll::SDL_SCANCODE_F4                 => F4,
            ll::SDL_SCANCODE_F5                 => F5,
            ll::SDL_SCANCODE_F6                 => F6,
            ll::SDL_SCANCODE_F7                 => F7,
            ll::SDL_SCANCODE_F8                 => F8,
            ll::SDL_SCANCODE_F9                 => F9,
            ll::SDL_SCANCODE_F10                => F10,
            ll::SDL_SCANCODE_F11                => F11,
            ll::SDL_SCANCODE_F12                => F12,
            ll::SDL_SCANCODE_PRINTSCREEN        => PrintScreen,
            ll::SDL_SCANCODE_SCROLLLOCK         => ScrollLock,
            ll::SDL_SCANCODE_PAUSE              => Pause,
            ll::SDL_SCANCODE_INSERT             => Insert,
            ll::SDL_SCANCODE_HOME               => Home,
            ll::SDL_SCANCODE_PAGEUP             => PageUp,
            ll::SDL_SCANCODE_DELETE             => Delete,
            ll::SDL_SCANCODE_END                => End,
            ll::SDL_SCANCODE_PAGEDOWN           => PageDown,
            ll::SDL_SCANCODE_RIGHT              => Right,
            ll::SDL_SCANCODE_LEFT               => Left,
            ll::SDL_SCANCODE_DOWN               => Down,
            ll::SDL_SCANCODE_UP                 => Up,
            ll::SDL_SCANCODE_NUMLOCKCLEAR       => NumLockClear,
            ll::SDL_SCANCODE_KP_DIVIDE          => KpDivide,
            ll::SDL_SCANCODE_KP_MULTIPLY        => KpMultiply,
            ll::SDL_SCANCODE_KP_MINUS           => KpMinus,
            ll::SDL_SCANCODE_KP_PLUS            => KpPlus,
            ll::SDL_SCANCODE_KP_ENTER           => KpEnter,
            ll::SDL_SCANCODE_KP_1               => Kp1,
            ll::SDL_SCANCODE_KP_2               => Kp2,
            ll::SDL_SCANCODE_KP_3               => Kp3,
            ll::SDL_SCANCODE_KP_4               => Kp4,
            ll::SDL_SCANCODE_KP_5               => Kp5,
            ll::SDL_SCANCODE_KP_6               => Kp6,
            ll::SDL_SCANCODE_KP_7               => Kp7,
            ll::SDL_SCANCODE_KP_8               => Kp8,
            ll::SDL_SCANCODE_KP_9               => Kp9,
            ll::SDL_SCANCODE_KP_0               => Kp0,
            ll::SDL_SCANCODE_KP_PERIOD          => KpPeriod,
            ll::SDL_SCANCODE_NONUSBACKSLASH     => NonUsBackslash,
            ll::SDL_SCANCODE_APPLICATION        => Application,
            ll::SDL_SCANCODE_POWER              => Power,
            ll::SDL_SCANCODE_KP_EQUALS          => KpEquals,
            ll::SDL_SCANCODE_F13                => F13,
            ll::SDL_SCANCODE_F14                => F14,
            ll::SDL_SCANCODE_F15                => F15,
            ll::SDL_SCANCODE_F16                => F16,
            ll::SDL_SCANCODE_F17                => F17,
            ll::SDL_SCANCODE_F18                => F18,
            ll::SDL_SCANCODE_F19                => F19,
            ll::SDL_SCANCODE_F20                => F20,
            ll::SDL_SCANCODE_F21                => F21,
            ll::SDL_SCANCODE_F22                => F22,
            ll::SDL_SCANCODE_F23                => F23,
            ll::SDL_SCANCODE_F24                => F24,
            ll::SDL_SCANCODE_EXECUTE            => Execute,
            ll::SDL_SCANCODE_HELP               => Help,
            ll::SDL_SCANCODE_MENU               => Menu,
            ll::SDL_SCANCODE_SELECT             => Select,
            ll::SDL_SCANCODE_STOP               => Stop,
            ll::SDL_SCANCODE_AGAIN              => Again,
            ll::SDL_SCANCODE_UNDO               => Undo,
            ll::SDL_SCANCODE_CUT                => Cut,
            ll::SDL_SCANCODE_COPY               => Copy,
            ll::SDL_SCANCODE_PASTE              => Paste,
            ll::SDL_SCANCODE_FIND               => Find,
            ll::SDL_SCANCODE_MUTE               => Mute,
            ll::SDL_SCANCODE_VOLUMEUP           => VolumeUp,
            ll::SDL_SCANCODE_VOLUMEDOWN         => VolumeDown,
            ll::SDL_SCANCODE_KP_COMMA           => KpComma,
            ll::SDL_SCANCODE_KP_EQUALSAS400     => KpEqualsAS400,
            ll::SDL_SCANCODE_INTERNATIONAL1     => International1,
            ll::SDL_SCANCODE_INTERNATIONAL2     => International2,
            ll::SDL_SCANCODE_INTERNATIONAL3     => International3,
            ll::SDL_SCANCODE_INTERNATIONAL4     => International4,
            ll::SDL_SCANCODE_INTERNATIONAL5     => International5,
            ll::SDL_SCANCODE_INTERNATIONAL6     => International6,
            ll::SDL_SCANCODE_INTERNATIONAL7     => International7,
            ll::SDL_SCANCODE_INTERNATIONAL8     => International8,
            ll::SDL_SCANCODE_INTERNATIONAL9     => International9,
            ll::SDL_SCANCODE_LANG1              => Lang1,
            ll::SDL_SCANCODE_LANG2              => Lang2,
            ll::SDL_SCANCODE_LANG3              => Lang3,
            ll::SDL_SCANCODE_LANG4              => Lang4,
            ll::SDL_SCANCODE_LANG5              => Lang5,
            ll::SDL_SCANCODE_LANG6              => Lang6,
            ll::SDL_SCANCODE_LANG7              => Lang7,
            ll::SDL_SCANCODE_LANG8              => Lang8,
            ll::SDL_SCANCODE_LANG9              => Lang9,
            ll::SDL_SCANCODE_ALTERASE           => AltErase,
            ll::SDL_SCANCODE_SYSREQ             => SysReq,
            ll::SDL_SCANCODE_CANCEL             => Cancel,
            ll::SDL_SCANCODE_CLEAR              => Clear,
            ll::SDL_SCANCODE_PRIOR              => Prior,
            ll::SDL_SCANCODE_RETURN2            => Return2,
            ll::SDL_SCANCODE_SEPARATOR          => Separator,
            ll::SDL_SCANCODE_OUT                => Out,
            ll::SDL_SCANCODE_OPER               => Oper,
            ll::SDL_SCANCODE_CLEARAGAIN         => ClearAgain,
            ll::SDL_SCANCODE_CRSEL              => Crse,
            ll::SDL_SCANCODE_EXSEL              => ExseL,
            ll::SDL_SCANCODE_KP_00              => Kp00,
            ll::SDL_SCANCODE_KP_000             => Kp000,
            ll::SDL_SCANCODE_THOUSANDSSEPARATOR => ThousandsSeparator,
            ll::SDL_SCANCODE_DECIMALSEPARATOR   => DecimalSeparator,
            ll::SDL_SCANCODE_CURRENCYUNIT       => CurrencyUnit,
            ll::SDL_SCANCODE_CURRENCYSUBUNIT    => CurrencySubUnit,
            ll::SDL_SCANCODE_KP_LEFTPAREN       => KpLeftParen,
            ll::SDL_SCANCODE_KP_RIGHTPAREN      => KpRightParen,
            ll::SDL_SCANCODE_KP_LEFTBRACE       => KpLeftBrace,
            ll::SDL_SCANCODE_KP_RIGHTBRACE      => KpRightBrace,
            ll::SDL_SCANCODE_KP_TAB             => KpTab,
            ll::SDL_SCANCODE_KP_BACKSPACE       => KpBackspace,
            ll::SDL_SCANCODE_KP_A               => KpA,
            ll::SDL_SCANCODE_KP_B               => KpB,
            ll::SDL_SCANCODE_KP_C               => KpC,
            ll::SDL_SCANCODE_KP_D               => KpD,
            ll::SDL_SCANCODE_KP_E               => KpE,
            ll::SDL_SCANCODE_KP_F               => KpF,
            ll::SDL_SCANCODE_KP_XOR             => KpXor,
            ll::SDL_SCANCODE_KP_POWER           => KpPower,
            ll::SDL_SCANCODE_KP_PERCENT         => KpPercent,
            ll::SDL_SCANCODE_KP_LESS            => KpLess,
            ll::SDL_SCANCODE_KP_GREATER         => KpGreater,
            ll::SDL_SCANCODE_KP_AMPERSAND       => KpAmpersand,
            ll::SDL_SCANCODE_KP_DBLAMPERSAND    => KpDblAmpersand,
            ll::SDL_SCANCODE_KP_VERTICALBAR     => KpVerticalBar,
            ll::SDL_SCANCODE_KP_DBLVERTICALBAR  => KpDblVerticalBar,
            ll::SDL_SCANCODE_KP_COLON           => KpColon,
            ll::SDL_SCANCODE_KP_HASH            => KpHash,
            ll::SDL_SCANCODE_KP_SPACE           => KpSpace,
            ll::SDL_SCANCODE_KP_AT              => KpAt,
            ll::SDL_SCANCODE_KP_EXCLAM          => KpExclam,
            ll::SDL_SCANCODE_KP_MEMSTORE        => KpMemStore,
            ll::SDL_SCANCODE_KP_MEMRECALL       => KpMemRecall,
            ll::SDL_SCANCODE_KP_MEMCLEAR        => KpMemClear,
            ll::SDL_SCANCODE_KP_MEMADD          => KpMemAdd,
            ll::SDL_SCANCODE_KP_MEMSUBTRACT     => KpMemSubtract,
            ll::SDL_SCANCODE_KP_MEMMULTIPLY     => KpMemMultiply,
            ll::SDL_SCANCODE_KP_MEMDIVIDE       => KpMemDivide,
            ll::SDL_SCANCODE_KP_PLUSMINUS       => KpPlusMinus,
            ll::SDL_SCANCODE_KP_CLEAR           => KpClear,
            ll::SDL_SCANCODE_KP_CLEARENTRY      => KpClearEntry,
            ll::SDL_SCANCODE_KP_BINARY          => KpBinary,
            ll::SDL_SCANCODE_KP_OCTAL           => KpOoctal,
            ll::SDL_SCANCODE_KP_DECIMAL         => KpDecimal,
            ll::SDL_SCANCODE_KP_HEXADECIMAL     => KpHexadecimal,
            ll::SDL_SCANCODE_LCTRL              => LCtrl,
            ll::SDL_SCANCODE_LSHIFT             => LShift,
            ll::SDL_SCANCODE_LALT               => LAlt,
            ll::SDL_SCANCODE_LGUI               => LGui,
            ll::SDL_SCANCODE_RCTRL              => RCtrl,
            ll::SDL_SCANCODE_RSHIFT             => RShift,
            ll::SDL_SCANCODE_RALT               => RAlt,
            ll::SDL_SCANCODE_RGUI               => RGui,
            ll::SDL_SCANCODE_MODE               => Mode,
            ll::SDL_SCANCODE_AUDIONEXT          => AudioNext,
            ll::SDL_SCANCODE_AUDIOPREV          => AudioPrev,
            ll::SDL_SCANCODE_AUDIOSTOP          => AudioStop,
            ll::SDL_SCANCODE_AUDIOPLAY          => AudioPlay,
            ll::SDL_SCANCODE_AUDIOMUTE          => AudioMute,
            ll::SDL_SCANCODE_MEDIASELECT        => MediaSelect,
            ll::SDL_SCANCODE_WWW                => Www,
            ll::SDL_SCANCODE_MAIL               => Mail,
            ll::SDL_SCANCODE_CALCULATOR         => Calculator,
            ll::SDL_SCANCODE_COMPUTER           => Computer,
            ll::SDL_SCANCODE_AC_SEARCH          => AcSearch,
            ll::SDL_SCANCODE_AC_HOME            => AcHome,
            ll::SDL_SCANCODE_AC_BACK            => AcBack,
            ll::SDL_SCANCODE_AC_FORWARD         => AcForward,
            ll::SDL_SCANCODE_AC_STOP            => AcStop,
            ll::SDL_SCANCODE_AC_REFRESH         => AcRefresh,
            ll::SDL_SCANCODE_AC_BOOKMARKS       => AcBookmarks,
            ll::SDL_SCANCODE_BRIGHTNESSDOWN     => BrightnessDown,
            ll::SDL_SCANCODE_BRIGHTNESSUP       => BrightnessUp,
            ll::SDL_SCANCODE_DISPLAYSWITCH      => DisplaySwitch,
            ll::SDL_SCANCODE_KBDILLUMTOGGLE     => KbdIllumToggle,
            ll::SDL_SCANCODE_KBDILLUMDOWN       => KbdIllumDown,
            ll::SDL_SCANCODE_KBDILLUMUP         => KbdIllumUp,
            ll::SDL_SCANCODE_EJECT              => Eject,
            ll::SDL_SCANCODE_SLEEP              => Sleep,
            ll::SDL_SCANCODE_APP1               => App1,
            ll::SDL_SCANCODE_APP2               => App2,
            ll::SDL_NUM_SCANCODES               => Num,
            _                                   => return None,
        })
    }

    fn from_u64(n: u64) -> Option<ScanCode> { FromPrimitive::from_i64(n as i64) }
}

use std::fmt;

impl fmt::Display for ScanCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

use keycode::KeyCode;

impl ScanCode {
    /// Gets the scancode from a virtual key. Returns None if there is no corresponding scancode.
    pub fn from_keycode(keycode: KeyCode) -> Option<ScanCode> {
        unsafe {
            match ::sys::keyboard::SDL_GetScancodeFromKey(keycode as i32) {
                ll::SDL_SCANCODE_UNKNOWN => None,
                scancode_id => FromPrimitive::from_isize(scancode_id as isize)
            }
        }
    }

    pub fn from_name(name: &str) -> Option<ScanCode> {
        unsafe {
            match CString::new(name) {
                Ok(name) => match ::sys::keyboard::SDL_GetScancodeFromName(name.as_ptr()) {
                    ll::SDL_SCANCODE_UNKNOWN => None,
                    scancode_id => Some(FromPrimitive::from_isize(scancode_id as isize).unwrap())
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None
            }
        }
    }

    pub fn name(self) -> &'static str {
        // The name string pointer lives in static, read-only memory.
        // Knowing this, we can always return a string slice.
        unsafe {
            let buf = ::sys::keyboard::SDL_GetScancodeName(self as u32);
            ::std::str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap()
        }
    }
}
