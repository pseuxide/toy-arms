#[doc(hidden)]
pub use winapi::um::winuser::GetAsyncKeyState;

/// detect_keydown! returns true if the passed keys are both pressed. You can specify as many keys as you crave.
/// * `$keycode` - VirtualKeycode you want to detect that's being pressed.
/// #Example
/// ```rust
/// use toy_arms::detect_keydown;
/// if detect_keydown!(VirtualKeyCode::VK_INSERT, VirtualKeyCode::VK_HOME) {
///     println!("INSERT and HOME is both pressed down");
/// }
/// ```
#[macro_export]
macro_rules! detect_keydown {
    ($($keycode:expr),*) => {
            if true $(&& (|keycode| unsafe { $crate::keyboard::GetAsyncKeyState(keycode) })($keycode) & 0x8000 != 0 )* {
                true
            } else {
                false
            }
    };
}

/// detect_keydown returns true when you pressed the specified key.
/// However, the "key press" signal will be emitted several times when you press and hold a key.
/// Make sure you release your finger immediately after pressing a key just in case so that this function catches key press only once.
/// # Example
/// ```rust
/// use toy_arms::keyboard::VirtualKeyCode;
/// if toy_arms::detect_keypress(VirtualKeyCode::VK_HOME) {
///     println!("HOME key is pressed!");
/// }
/// ```
pub fn detect_keypress(code: i32) -> bool {
    unsafe { GetAsyncKeyState(code) & 1 != 0 }
}

/// VirtualKeyCode is a set of virtual key code defined by microsoft.
/// document is [here](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
pub struct VirtualKeyCode;
impl VirtualKeyCode {
    pub const VK_LBUTTON: i32 = 0x01;
    pub const VK_RBUTTON: i32 = 0x02;
    pub const VK_CANCEL: i32 = 0x03;
    pub const VK_MBUTTON: i32 = 0x04;
    pub const VK_XBUTTON1: i32 = 0x05;
    pub const VK_XBUTTON2: i32 = 0x06;
    pub const VK_BACK: i32 = 0x08;
    pub const VK_TAB: i32 = 0x09;
    pub const VK_CLEAR: i32 = 0x0C;
    pub const VK_RETURN: i32 = 0x0D;
    pub const VK_SHIFT: i32 = 0x10;
    pub const VK_CONTROL: i32 = 0x11;
    pub const VK_MENU: i32 = 0x12;
    pub const VK_PAUSE: i32 = 0x13;
    pub const VK_CAPITAL: i32 = 0x14;
    pub const VK_KANA: i32 = 0x15;
    pub const VK_HANGUEL: i32 = 0x15;
    pub const VK_HANGUL: i32 = 0x15;
    pub const VK_IME_ON: i32 = 0x16;
    pub const VK_JUNJA: i32 = 0x17;
    pub const VK_FINAL: i32 = 0x18;
    pub const VK_HANJA: i32 = 0x19;
    pub const VK_KANJI: i32 = 0x19;
    pub const VK_IME_OFF: i32 = 0x1A;
    pub const VK_ESCAPE: i32 = 0x1B;
    pub const VK_CONVERT: i32 = 0x1C;
    pub const VK_NONCONVERT: i32 = 0x1D;
    pub const VK_ACCEPT: i32 = 0x1E;
    pub const VK_MODECHANGE: i32 = 0x1F;
    pub const VK_SPACE: i32 = 0x20;
    pub const VK_PRIOR: i32 = 0x21;
    pub const VK_NEXT: i32 = 0x22;
    pub const VK_END: i32 = 0x23;
    pub const VK_HOME: i32 = 0x24;
    pub const VK_LEFT: i32 = 0x25;
    pub const VK_UP: i32 = 0x26;
    pub const VK_RIGHT: i32 = 0x27;
    pub const VK_DOWN: i32 = 0x28;
    pub const VK_SELECT: i32 = 0x29;
    pub const VK_PRINT: i32 = 0x2A;
    pub const VK_EXECUTE: i32 = 0x2B;
    pub const VK_SNAPSHOT: i32 = 0x2C;
    pub const VK_INSERT: i32 = 0x2D;
    pub const VK_DELETE: i32 = 0x2E;
    pub const VK_HELP: i32 = 0x2F;
    pub const VK_0: i32 = 0x30;
    pub const VK_1: i32 = 0x31;
    pub const VK_2: i32 = 0x32;
    pub const VK_3: i32 = 0x33;
    pub const VK_4: i32 = 0x34;
    pub const VK_5: i32 = 0x35;
    pub const VK_6: i32 = 0x36;
    pub const VK_7: i32 = 0x37;
    pub const VK_8: i32 = 0x38;
    pub const VK_9: i32 = 0x39;
    pub const VK_A: i32 = 0x41;
    pub const VK_B: i32 = 0x42;
    pub const VK_C: i32 = 0x43;
    pub const VK_D: i32 = 0x44;
    pub const VK_E: i32 = 0x45;
    pub const VK_F: i32 = 0x46;
    pub const VK_G: i32 = 0x47;
    pub const VK_H: i32 = 0x48;
    pub const VK_I: i32 = 0x49;
    pub const VK_J: i32 = 0x4A;
    pub const VK_K: i32 = 0x4B;
    pub const VK_L: i32 = 0x4C;
    pub const VK_M: i32 = 0x4D;
    pub const VK_N: i32 = 0x4E;
    pub const VK_O: i32 = 0x4F;
    pub const VK_P: i32 = 0x50;
    pub const VK_Q: i32 = 0x51;
    pub const VK_R: i32 = 0x52;
    pub const VK_S: i32 = 0x53;
    pub const VK_T: i32 = 0x54;
    pub const VK_U: i32 = 0x55;
    pub const VK_V: i32 = 0x56;
    pub const VK_W: i32 = 0x57;
    pub const VK_X: i32 = 0x58;
    pub const VK_Y: i32 = 0x59;
    pub const VK_Z: i32 = 0x5A;
    pub const VK_LWIN: i32 = 0x5B;
    pub const VK_RWIN: i32 = 0x5C;
    pub const VK_APPS: i32 = 0x5D;
    pub const VK_SLEEP: i32 = 0x5F;
    pub const VK_NUMPAD0: i32 = 0x60;
    pub const VK_NUMPAD1: i32 = 0x61;
    pub const VK_NUMPAD2: i32 = 0x62;
    pub const VK_NUMPAD3: i32 = 0x63;
    pub const VK_NUMPAD4: i32 = 0x64;
    pub const VK_NUMPAD5: i32 = 0x65;
    pub const VK_NUMPAD6: i32 = 0x66;
    pub const VK_NUMPAD7: i32 = 0x67;
    pub const VK_NUMPAD8: i32 = 0x68;
    pub const VK_NUMPAD9: i32 = 0x69;
    pub const VK_MULTIPLY: i32 = 0x6A;
    pub const VK_ADD: i32 = 0x6B;
    pub const VK_SEPARATOR: i32 = 0x6C;
    pub const VK_SUBTRACT: i32 = 0x6D;
    pub const VK_DECIMAL: i32 = 0x6E;
    pub const VK_DIVIDE: i32 = 0x6F;
    pub const VK_F1: i32 = 0x70;
    pub const VK_F2: i32 = 0x71;
    pub const VK_F3: i32 = 0x72;
    pub const VK_F4: i32 = 0x73;
    pub const VK_F5: i32 = 0x74;
    pub const VK_F6: i32 = 0x75;
    pub const VK_F7: i32 = 0x76;
    pub const VK_F8: i32 = 0x77;
    pub const VK_F9: i32 = 0x78;
    pub const VK_F10: i32 = 0x79;
    pub const VK_F11: i32 = 0x7A;
    pub const VK_F12: i32 = 0x7B;
    pub const VK_F13: i32 = 0x7C;
    pub const VK_F14: i32 = 0x7D;
    pub const VK_F15: i32 = 0x7E;
    pub const VK_F16: i32 = 0x7F;
    pub const VK_F17: i32 = 0x80;
    pub const VK_F18: i32 = 0x81;
    pub const VK_F19: i32 = 0x82;
    pub const VK_F20: i32 = 0x83;
    pub const VK_F21: i32 = 0x84;
    pub const VK_F22: i32 = 0x85;
    pub const VK_F23: i32 = 0x86;
    pub const VK_F24: i32 = 0x87;
    pub const VK_NUMLOCK: i32 = 0x90;
    pub const VK_SCROLL: i32 = 0x91;
    pub const VK_LSHIFT: i32 = 0xA0;
    pub const VK_RSHIFT: i32 = 0xA1;
    pub const VK_LCONTROL: i32 = 0xA2;
    pub const VK_RCONTROL: i32 = 0xA3;
    pub const VK_LMENU: i32 = 0xA4;
    pub const VK_RMENU: i32 = 0xA5;
    pub const VK_BROWSER_BACK: i32 = 0xA6;
    pub const VK_BROWSER_FORWARD: i32 = 0xA7;
    pub const VK_BROWSER_REFRESH: i32 = 0xA8;
    pub const VK_BROWSER_STOP: i32 = 0xA9;
    pub const VK_BROWSER_SEARCH: i32 = 0xAA;
    pub const VK_BROWSER_FAVORITES: i32 = 0xAB;
    pub const VK_BROWSER_HOME: i32 = 0xAC;
    pub const VK_VOLUME_MUTE: i32 = 0xAD;
    pub const VK_VOLUME_DOWN: i32 = 0xAE;
    pub const VK_VOLUME_UP: i32 = 0xAF;
    pub const VK_MEDIA_NEXT_TRACK: i32 = 0xB0;
    pub const VK_MEDIA_PREV_TRACK: i32 = 0xB1;
    pub const VK_MEDIA_STOP: i32 = 0xB2;
    pub const VK_MEDIA_PLAY_PAUSE: i32 = 0xB3;
    pub const VK_LAUNCH_MAIL: i32 = 0xB4;
    pub const VK_LAUNCH_MEDIA_SELECT: i32 = 0xB5;
    pub const VK_LAUNCH_APP1: i32 = 0xB6;
    pub const VK_LAUNCH_APP2: i32 = 0xB7;
    pub const VK_OEM_1: i32 = 0xBA;
    pub const VK_OEM_PLUS: i32 = 0xBB;
    pub const VK_OEM_COMMA: i32 = 0xBC;
    pub const VK_OEM_MINUS: i32 = 0xBD;
    pub const VK_OEM_PERIOD: i32 = 0xBE;
    pub const VK_OEM_2: i32 = 0xBF;
    pub const VK_OEM_3: i32 = 0xC0;
    pub const VK_OEM_4: i32 = 0xDB;
    pub const VK_OEM_5: i32 = 0xDC;
    pub const VK_OEM_6: i32 = 0xDD;
    pub const VK_OEM_7: i32 = 0xDE;
    pub const VK_OEM_8: i32 = 0xDF;
    pub const VK_OEM_102: i32 = 0xE2;
    pub const VK_PROCESSKEY: i32 = 0xE5;
    pub const VK_PACKET: i32 = 0xE7;
    pub const VK_ATTN: i32 = 0xF6;
    pub const VK_CRSEL: i32 = 0xF7;
    pub const VK_EXSEL: i32 = 0xF8;
    pub const VK_EREOF: i32 = 0xF9;
    pub const VK_PLAY: i32 = 0xFA;
    pub const VK_ZOOM: i32 = 0xFB;
    pub const VK_NONAME: i32 = 0xFC;
    pub const VK_PA1: i32 = 0xFD;
    pub const VK_OEM_CLEAR: i32 = 0xFE;
}
