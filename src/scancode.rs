use keys::{self, Key};

pub fn scancode_to_key(escaped: bool, scancode: u8) -> Option<Key> {
    match (escaped, scancode) {
        (false, 0x01) => Some(keys::KEY_ESC),
        (false, 0x02) => Some(keys::KEY_1),
        (false, 0x03) => Some(keys::KEY_2),
        (false, 0x04) => Some(keys::KEY_3),
        (false, 0x05) => Some(keys::KEY_4),
        (false, 0x06) => Some(keys::KEY_5),
        (false, 0x07) => Some(keys::KEY_6),
        (false, 0x08) => Some(keys::KEY_7),
        (false, 0x09) => Some(keys::KEY_8),
        (false, 0x0A) => Some(keys::KEY_9),
        (false, 0x0B) => Some(keys::KEY_0),
        (false, 0x0C) => Some(keys::KEY_HYPHEN),
        (false, 0x0D) => Some(keys::KEY_EQUALS),
        (false, 0x0E) => Some(keys::KEY_BACKSPACE),
        (false, 0x0F) => Some(keys::KEY_TAB),
        (false, 0x10) => Some(keys::KEY_Q),
        (false, 0x11) => Some(keys::KEY_W),
        (false, 0x12) => Some(keys::KEY_E),
        (false, 0x13) => Some(keys::KEY_R),
        (false, 0x14) => Some(keys::KEY_T),
        (false, 0x15) => Some(keys::KEY_Y),
        (false, 0x16) => Some(keys::KEY_U),
        (false, 0x17) => Some(keys::KEY_I),
        (false, 0x18) => Some(keys::KEY_O),
        (false, 0x19) => Some(keys::KEY_P),
        (false, 0x1A) => Some(keys::KEY_OP_BRACKET),
        (false, 0x1B) => Some(keys::KEY_CL_BRACKET),
        (false, 0x1C) => Some(keys::KEY_ENTER),
        (true, 0x1C) => Some(keys::KEY_NUM_ENTER),
        (false, 0x1D) => Some(keys::KEY_L_CTRL),
        (true, 0x1D) => Some(keys::KEY_R_CTRL),
        (false, 0x1E) => Some(keys::KEY_A),
        (false, 0x1F) => Some(keys::KEY_S),
        (false, 0x20) => Some(keys::KEY_D),
        (false, 0x21) => Some(keys::KEY_F),
        (false, 0x22) => Some(keys::KEY_G),
        (false, 0x23) => Some(keys::KEY_H),
        (false, 0x24) => Some(keys::KEY_J),
        (false, 0x25) => Some(keys::KEY_K),
        (false, 0x26) => Some(keys::KEY_L),
        (false, 0x27) => Some(keys::KEY_COLON),
        (false, 0x28) => Some(keys::KEY_QUOTE),
        (false, 0x29) => Some(keys::KEY_HASH), // CHECKME
        (false, 0x2A) => Some(keys::KEY_L_SHIFT),
        (false, 0x2B) => Some(keys::KEY_BACKTICK), // CHECKME
        (false, 0x2C) => Some(keys::KEY_Z),
        (false, 0x2D) => Some(keys::KEY_X),
        (false, 0x2E) => Some(keys::KEY_C),
        (false, 0x2F) => Some(keys::KEY_V),
        (false, 0x30) => Some(keys::KEY_B),
        (false, 0x31) => Some(keys::KEY_N),
        (false, 0x32) => Some(keys::KEY_M),
        (false, 0x33) => Some(keys::KEY_COMMA),
        (false, 0x34) => Some(keys::KEY_PERIOD),
        (false, 0x35) => Some(keys::KEY_F_SLASH),
        (false, 0x36) => Some(keys::KEY_R_SHIFT),
        (false, 0x37) => Some(keys::KEY_PRT_SCR),
        (false, 0x38) => Some(keys::KEY_ALT),
        (true, 0x38) => Some(keys::KEY_ALT_GR),
        (false, 0x39) => Some(keys::KEY_SPACE),
        (false, 0x3A) => Some(keys::KEY_CAPS_LOCK),
        (false, 0x3B) => Some(keys::KEY_F1),
        (false, 0x3C) => Some(keys::KEY_F2),
        (false, 0x3D) => Some(keys::KEY_F3),
        (false, 0x3E) => Some(keys::KEY_F4),
        (false, 0x3F) => Some(keys::KEY_F5),
        (false, 0x40) => Some(keys::KEY_F6),
        (false, 0x41) => Some(keys::KEY_F7),
        (false, 0x42) => Some(keys::KEY_F8),
        (false, 0x43) => Some(keys::KEY_F9),
        (false, 0x44) => Some(keys::KEY_F10),
        (false, 0x45) => Some(keys::KEY_NUM_LOCK),
        (false, 0x46) => Some(keys::KEY_SCROLL_LOCK),
        (false, 0x47) => Some(keys::KEY_NUM_7),
        (true, 0x47) => Some(keys::KEY_HOME),
        (false, 0x48) => Some(keys::KEY_NUM_8),
        (true, 0x48) => Some(keys::KEY_UP),
        (false, 0x49) => Some(keys::KEY_NUM_9),
        (true, 0x49) => Some(keys::KEY_PG_UP),
        (false, 0x4A) => Some(keys::KEY_NUM_SUB),
        (false, 0x4B) => Some(keys::KEY_NUM_4),
        (true, 0x4B) => Some(keys::KEY_LEFT),
        (false, 0x4C) => Some(keys::KEY_NUM_5),
        (false, 0x4D) => Some(keys::KEY_NUM_6),
        (true, 0x4D) => Some(keys::KEY_RIGHT),
        (false, 0x4E) => Some(keys::KEY_NUM_ADD),
        (false, 0x4F) => Some(keys::KEY_NUM_1),
        (true, 0x4F) => Some(keys::KEY_END),
        (false, 0x50) => Some(keys::KEY_NUM_2),
        (true, 0x50) => Some(keys::KEY_DOWN),
        (false, 0x51) => Some(keys::KEY_NUM_3),
        (true, 0x51) => Some(keys::KEY_PG_DOWN),
        (false, 0x52) => Some(keys::KEY_NUM_0),
        (true, 0x52) => Some(keys::KEY_INS),
        (false, 0x53) => Some(keys::KEY_NUM_DECIMAL),
        (true, 0x53) => Some(keys::KEY_DEL),
        (false, 0x54) => None, // KEY_SYS_RQ
        (false, 0x56) => None, // KEY_B_SLASH (GB only)
        (false, 0x57) => Some(keys::KEY_F11),
        (false, 0x58) => Some(keys::KEY_F12),
        (true, 0x5B) => Some(keys::KEY_L_SUPER),  // PC only
        (true, 0x5C) => Some(keys::KEY_R_SUPER),  // PC only
        (true, 0x5D) => Some(keys::KEY_MENU),  // PC only
        _ => None,
    }
}

#[derive(Debug)]
pub struct ScancodeReader {
    escaped: bool,
}

impl ScancodeReader {
    pub fn new() -> ScancodeReader {
        ScancodeReader {
            escaped: false,
        }
    }

    pub fn feed_scancode(&mut self, mut scancode: u8) -> Option<(Key, bool)> {
        if scancode == 0xE0 {
            self.escaped = true;
            return None;
        }

        let released = if scancode >= 0x80 {
            scancode -= 0x80;
            true
        } else {
            false
        };

        let key = scancode_to_key(self.escaped, scancode);

        self.escaped = false;

        if let Some(key) = key {
            Some((key, released))
        } else {
            // TODO Warning

            None
        }
    }
}
