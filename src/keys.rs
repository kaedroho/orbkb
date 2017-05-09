#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    /// Keys that represent alphabetic characters
    Alphabetic,
    /// Keys that represent numerals
    Numeric,
    /// Keys that represent punctuation
    Punctuation,
    /// The numeric and '.' keys on the numpad (symbols, num lock, and enter keys not included)
    Numpad,
    /// Keys that represent controls such as enter, escape, backspace, and functions
    Control,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key(u8);

pub const KEY_ESC: Key = Key(1);
pub const KEY_1: Key = Key(2);
pub const KEY_2: Key = Key(3);
pub const KEY_3: Key = Key(4);
pub const KEY_4: Key = Key(5);
pub const KEY_5: Key = Key(6);
pub const KEY_6: Key = Key(7);
pub const KEY_7: Key = Key(8);
pub const KEY_8: Key = Key(9);
pub const KEY_9: Key = Key(10);
pub const KEY_0: Key = Key(11);
pub const KEY_HYPHEN: Key = Key(12);
pub const KEY_EQUALS: Key = Key(13);
pub const KEY_BACKSPACE: Key = Key(14);
pub const KEY_TAB: Key = Key(15);
pub const KEY_Q: Key = Key(16);
pub const KEY_W: Key = Key(17);
pub const KEY_E: Key = Key(18);
pub const KEY_R: Key = Key(19);
pub const KEY_T: Key = Key(20);
pub const KEY_Y: Key = Key(21);
pub const KEY_U: Key = Key(22);
pub const KEY_I: Key = Key(23);
pub const KEY_O: Key = Key(24);
pub const KEY_P: Key = Key(25);
pub const KEY_OP_BRACKET: Key = Key(26);
pub const KEY_CL_BRACKET: Key = Key(27);
pub const KEY_ENTER: Key = Key(28);
pub const KEY_L_CTRL: Key = Key(29);
pub const KEY_A: Key = Key(30);
pub const KEY_S: Key = Key(31);
pub const KEY_D: Key = Key(32);
pub const KEY_F: Key = Key(33);
pub const KEY_G: Key = Key(34);
pub const KEY_H: Key = Key(35);
pub const KEY_J: Key = Key(36);
pub const KEY_K: Key = Key(37);
pub const KEY_L: Key = Key(38);
pub const KEY_COLON: Key = Key(39);
pub const KEY_QUOTE: Key = Key(40);
pub const KEY_BACKTICK: Key = Key(41);
pub const KEY_L_SHIFT: Key = Key(42);
pub const KEY_HASH: Key = Key(43);
pub const KEY_Z: Key = Key(44);
pub const KEY_X: Key = Key(45);
pub const KEY_C: Key = Key(46);
pub const KEY_V: Key = Key(47);
pub const KEY_B: Key = Key(48);
pub const KEY_N: Key = Key(49);
pub const KEY_M: Key = Key(50);
pub const KEY_COMMA: Key = Key(51);
pub const KEY_PERIOD: Key = Key(52);
pub const KEY_F_SLASH: Key = Key(53);
pub const KEY_R_SHIFT: Key = Key(54);
pub const KEY_NUM_MUL: Key = Key(55);
pub const KEY_ALT: Key = Key(56);
pub const KEY_SPACE: Key = Key(57);
pub const KEY_CAPS_LOCK: Key = Key(58);
pub const KEY_F1: Key = Key(59);
pub const KEY_F2: Key = Key(60);
pub const KEY_F3: Key = Key(61);
pub const KEY_F4: Key = Key(62);
pub const KEY_F5: Key = Key(63);
pub const KEY_F6: Key = Key(64);
pub const KEY_F7: Key = Key(65);
pub const KEY_F8: Key = Key(66);
pub const KEY_F9: Key = Key(67);
pub const KEY_F10: Key = Key(68);
pub const KEY_NUM_LOCK: Key = Key(69);
pub const KEY_SCROLL_LOCK: Key = Key(70);
pub const KEY_NUM_7: Key = Key(71);
pub const KEY_NUM_8: Key = Key(72);
pub const KEY_NUM_9: Key = Key(73);
pub const KEY_NUM_SUB: Key = Key(74);
pub const KEY_NUM_4: Key = Key(75);
pub const KEY_NUM_5: Key = Key(76);
pub const KEY_NUM_6: Key = Key(77);
pub const KEY_NUM_ADD: Key = Key(78);
pub const KEY_NUM_1: Key = Key(79);
pub const KEY_NUM_2: Key = Key(80);
pub const KEY_NUM_3: Key = Key(81);
pub const KEY_NUM_0: Key = Key(82);
pub const KEY_NUM_DECIMAL: Key = Key(83);
pub const KEY_F11: Key = Key(87);
pub const KEY_F12: Key = Key(88);
pub const KEY_NUM_ENTER: Key = Key(96);
pub const KEY_R_CTRL: Key = Key(97);
pub const KEY_NUM_DIV: Key = Key(98);
pub const KEY_PRT_SCR: Key = Key(99);
pub const KEY_ALT_GR: Key = Key(100);
pub const KEY_HOME: Key = Key(102);
pub const KEY_UP: Key = Key(103);
pub const KEY_PG_UP: Key = Key(104);
pub const KEY_LEFT: Key = Key(105);
pub const KEY_RIGHT: Key = Key(106);
pub const KEY_END: Key = Key(107);
pub const KEY_DOWN: Key = Key(108);
pub const KEY_PG_DOWN: Key = Key(109);
pub const KEY_INS: Key = Key(110);
pub const KEY_DEL: Key = Key(111);
pub const KEY_PAUSE: Key = Key(119);
pub const KEY_L_SUPER: Key = Key(125);
pub const KEY_R_SUPER: Key = Key(126);
pub const KEY_MENU: Key = Key(127);

impl Key {
    pub fn from_keycode(keycode: u8) -> Key {
        Key(keycode)
    }

    /// Returns the keycode for this key
    pub fn keycode(&self) -> u8 {
        self.0
    }

    pub fn qwerty_char(&self) -> char {
        match *self {
            KEY_1 => '1',
            KEY_2 => '2',
            KEY_3 => '3',
            KEY_4 => '4',
            KEY_5 => '5',
            KEY_6 => '6',
            KEY_7 => '7',
            KEY_8 => '8',
            KEY_9 => '9',
            KEY_0 => '0',
            KEY_HYPHEN => '-',
            KEY_EQUALS => '=',
            KEY_TAB => '\t',
            KEY_Q => 'q',
            KEY_W => 'w',
            KEY_E => 'e',
            KEY_R => 'r',
            KEY_T => 't',
            KEY_Y => 'y',
            KEY_U => 'u',
            KEY_I => 'i',
            KEY_O => 'o',
            KEY_P => 'p',
            KEY_OP_BRACKET => '[',
            KEY_CL_BRACKET => ']',
            KEY_ENTER => '\n',
            KEY_A => 'a',
            KEY_S => 's',
            KEY_D => 'd',
            KEY_F => 'f',
            KEY_G => 'g',
            KEY_H => 'h',
            KEY_J => 'j',
            KEY_K => 'k',
            KEY_L => 'l',
            KEY_COLON => ';',
            KEY_QUOTE => '\'',
            KEY_BACKTICK => '`',
            KEY_HASH => '#',
            KEY_Z => 'z',
            KEY_X => 'x',
            KEY_C => 'c',
            KEY_V => 'v',
            KEY_B => 'b',
            KEY_N => 'n',
            KEY_M => 'm',
            KEY_COMMA => ',',
            KEY_PERIOD => '.',
            KEY_F_SLASH => '/',
            KEY_NUM_MUL => '*',
            KEY_NUM_SUB => '-',
            KEY_NUM_ADD => '+',
            KEY_NUM_DECIMAL => '.',
            KEY_NUM_DIV => '/',
            _ => '\0'
        }
    }

    /// Returns the name of the key for debugging and reference
    ///
    /// Do not use this for finding the symbol of the key as it does not take into account the keyboard layout or modifiers
    pub fn name(&self) -> &'static str {
        match *self {
            KEY_ESC => "ESC",
            KEY_1 => "1",
            KEY_2 => "2",
            KEY_3 => "3",
            KEY_4 => "4",
            KEY_5 => "5",
            KEY_6 => "6",
            KEY_7 => "7",
            KEY_8 => "8",
            KEY_9 => "9",
            KEY_0 => "0",
            KEY_HYPHEN => "-",
            KEY_EQUALS => "=",
            KEY_BACKSPACE => "BACKSPACE",
            KEY_TAB => "TAB",
            KEY_Q => "Q",
            KEY_W => "W",
            KEY_E => "E",
            KEY_R => "R",
            KEY_T => "T",
            KEY_Y => "Y",
            KEY_U => "U",
            KEY_I => "I",
            KEY_O => "O",
            KEY_P => "P",
            KEY_OP_BRACKET => "[",
            KEY_CL_BRACKET => "]",
            KEY_ENTER => "ENTER",
            KEY_L_CTRL => "L CTRL",
            KEY_A => "A",
            KEY_S => "S",
            KEY_D => "D",
            KEY_F => "F",
            KEY_G => "G",
            KEY_H => "H",
            KEY_J => "J",
            KEY_K => "K",
            KEY_L => "L",
            KEY_COLON => ";",
            KEY_QUOTE => "'",
            KEY_BACKTICK => "`",
            KEY_L_SHIFT => "L SHIFT",
            KEY_HASH => "#",
            KEY_Z => "Z",
            KEY_X => "X",
            KEY_C => "C",
            KEY_V => "V",
            KEY_B => "B",
            KEY_N => "N",
            KEY_M => "M",
            KEY_COMMA => ",",
            KEY_PERIOD => ".",
            KEY_F_SLASH => "/",
            KEY_R_SHIFT => "R SHIFT",
            KEY_NUM_MUL => "NUM *",
            KEY_ALT => "ALT",
            KEY_SPACE => "SPACE",
            KEY_CAPS_LOCK => "CAPS LOCK",
            KEY_F1 => "F1",
            KEY_F2 => "F2",
            KEY_F3 => "F3",
            KEY_F4 => "F4",
            KEY_F5 => "F5",
            KEY_F6 => "F6",
            KEY_F7 => "F7",
            KEY_F8 => "F8",
            KEY_F9 => "F9",
            KEY_F10 => "F10",
            KEY_NUM_LOCK => "NUM LOCK",
            KEY_SCROLL_LOCK => "SCROLL LOCK",
            KEY_NUM_7 => "NUM 7",
            KEY_NUM_8 => "NUM 8",
            KEY_NUM_9 => "NUM 9",
            KEY_NUM_SUB => "NUM -",
            KEY_NUM_4 => "NUM 4",
            KEY_NUM_5 => "NUM 5",
            KEY_NUM_6 => "NUM 6",
            KEY_NUM_ADD => "NUM +",
            KEY_NUM_1 => "NUM 1",
            KEY_NUM_2 => "NUM 2",
            KEY_NUM_3 => "NUM 3",
            KEY_NUM_0 => "NUM 0",
            KEY_NUM_DECIMAL => "NUM .",
            KEY_F11 => "F11",
            KEY_F12 => "F12",
            KEY_NUM_ENTER => "NUM ENTER",
            KEY_R_CTRL => "R CTRL",
            KEY_NUM_DIV => "NUM /",
            KEY_PRT_SCR => "PRINT SCREEN",
            KEY_ALT_GR => "ALT GR",
            KEY_HOME => "HOME",
            KEY_UP => "UP",
            KEY_PG_UP => "PAGE UP",
            KEY_LEFT => "LEFT",
            KEY_RIGHT => "RIGHT",
            KEY_END => "END",
            KEY_DOWN => "DOWN",
            KEY_PG_DOWN => "PAGE DOWN",
            KEY_INS => "INSERT",
            KEY_DEL => "DELETE",
            KEY_PAUSE => "PAUSE",
            KEY_L_SUPER => "L SUPER",
            KEY_R_SUPER => "R SUPER",
            KEY_MENU => "MENU",
            _ => "UNNAMED",
        }
    }

    /// Returns true if the key represents an alphabetic character
    pub fn is_alphabetic(&self) -> bool {
        self.key_type() == Some(KeyType::Alphabetic)
    }

    /// Returns true if the key represents a numeric character
    ///
    /// Note: this does not include the numpad keys.
    pub fn is_numeric(&self) -> bool {
        self.key_type() == Some(KeyType::Numeric)
    }

    /// Returns true if the key represents a punctuation character
    pub fn is_punctuation(&self) -> bool {
        self.key_type() == Some(KeyType::Punctuation)
    }

    /// Returns true if the key represents a control key
    ///
    /// For example: Enter, Caps lock, Page up, etc
    pub fn is_control(&self) -> bool {
        self.key_type() == Some(KeyType::Control)
    }

    /// Returns true if the key represents a numeric character on the numpad
    ///
    /// This includes all the numbers and the '.' key but not enter or any of the symbols
    pub fn is_numpad(&self) -> bool {
        self.key_type() == Some(KeyType::Numpad)
    }

    /// Returns the type of the key which used to determine which modifers can alter the keys level
    pub fn key_type(&self) -> Option<KeyType> {
        match self.keycode() {
            16 ... 25 | 30 ... 38 | 44 ... 50 => Some(KeyType::Alphabetic),
            2 ... 11 => Some(KeyType::Numeric),
            12 | 13 | 26 | 27 | 39 | 40 | 41 | 43 | 51 | 52 | 53 | 55 | 98 => Some(KeyType::Punctuation),
            1 | 14 | 15 | 28 | 29 | 42 | 54 | 56 ... 70 | 74 | 78 | 87 | 88 | 96 | 97 | 99 | 100 | 102 ... 111 | 119 | 125 | 126 | 127  => Some(KeyType::Control),
            71 | 72 | 73 | 75 | 76 | 77 | 79 ... 83 => Some(KeyType::Numpad),
            _ => None,
        }
    }
}
