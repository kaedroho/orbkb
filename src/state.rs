use fixedbitset::FixedBitSet;

use keys::{self, KeyType, Key};
use layout::Layout;
use event::{KeyboardEvent, KeyboardAction};

bitflags! {
    pub flags Modifiers: u8 {
        const SHIFT = 1,
        const CTRL = 1 << 1,
        const ALT = 1 << 2,
        const ALT_GR = 1 << 3,
        const SUPER = 1 << 4,
    }
}

impl Modifiers {
    pub fn to_bits(&self) -> u8 {
        self.bits
    }
}

bitflags! {
    pub flags LockKeyState: u8 {
        const CAPS_LOCK = 1,
        const NUM_LOCK = 1 << 1,
        const SCROLL_LOCK = 1 << 2,
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    /// Tracks the physical state of each key
    pub pressed_keys: FixedBitSet,

    /// Tracks which lock keys are currently active
    pub lock_keys: LockKeyState,

    /// The keyboard layout
    pub layout: Layout,

    /// When set to true the lock key states will toggle when the keys are pressed
    ///
    /// This allows the toggling to be disabled so the keys could be used for something else (eg a full screen game)
    /// default: true
    pub lock_keys_enabled: bool,
}

impl KeyboardState {
    /// Creates a new KeyboardState instance
    pub fn new(layout: Layout) -> KeyboardState {
        KeyboardState {
            pressed_keys: FixedBitSet::with_capacity(256),
            lock_keys: LockKeyState::empty(),
            layout: layout,
            lock_keys_enabled: true,
        }
    }

    /// Returns true if the specified key is currently pressed
    pub fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(key.keycode() as usize)
    }

    /// Returns true if either shift key is pressed
    pub fn shift(&self) -> bool {
        self.key_pressed(keys::KEY_L_SHIFT) | self.key_pressed(keys::KEY_R_SHIFT)
    }

    /// Returns true if either control key is pressed
    pub fn ctrl(&self) -> bool {
        self.key_pressed(keys::KEY_L_CTRL) | self.key_pressed(keys::KEY_R_CTRL)
    }

    /// Returns true if the alt key is pressed
    /// Note: If has_alt_gr_key is false, that key would be treated as an alt key instead
    pub fn alt(&self) -> bool {
        self.key_pressed(keys::KEY_ALT) || (!self.layout.has_alt_gr_key && self.key_pressed(keys::KEY_ALT_GR))
    }

    /// Returns true if either super key is pressed
    ///
    /// Note: The super keys are also known as the "Windows key" (Windows) or "Command key" (Macintosh)
    pub fn sup(&self) -> bool {
        self.key_pressed(keys::KEY_L_SUPER) | self.key_pressed(keys::KEY_R_SUPER)
    }

    /// Returns true if the alt gr (Alternate Graphic) key is pressed
    pub fn alt_gr(&self) -> bool {
        self.layout.has_alt_gr_key && self.key_pressed(keys::KEY_ALT_GR)
    }

    /// Returns a bitflags object of logical modifier states
    pub fn get_modifiers(&self) -> Modifiers {
        let mut modifiers = Modifiers::empty();
        if self.shift() { modifiers.insert(SHIFT) }
        if self.ctrl() { modifiers.insert(CTRL) }
        if self.alt() { modifiers.insert(ALT) }
        if self.sup() { modifiers.insert(SUPER) }
        if self.alt_gr() { modifiers.insert(ALT_GR) }
        modifiers
    }

    /// Returns true if the caps lock is currently active
    pub fn caps_lock(&self) -> bool {
        self.lock_keys.contains(CAPS_LOCK)
    }

    /// Returns true if the num lock is currently active
    pub fn num_lock(&self) -> bool {
        self.lock_keys.contains(NUM_LOCK)
    }

    /// Returns true if the scroll lock is currently active
    pub fn scroll_lock(&self) -> bool {
        self.lock_keys.contains(SCROLL_LOCK)
    }

    /// Returns the currently selected key group
    ///
    /// Note: Only AltGr based group switching has been implemented so far
    pub fn get_group(&self) -> u8 {
        if self.alt_gr() { 1 } else { 0 }
    }

    /// Returns the level that is selected on the specified key type
    ///
    /// Note: The symbol/action of each level is indicated by row on the keytop
    ///
    /// Different key types need to behave differently with regards to level switching:
    ///  * Alphabetic keys can be shifted by using either the shift key or caps lock (when both
    ///    used at the same time, they will negate each other)
    ///  * NumeralsAndPunctuation keys are not shifted by the caps lock
    ///  * Numpad keys are shifted by the num lock only
    pub fn get_key_level(&self, key_type: &KeyType) -> u8 {
        let shifted = match *key_type {
            KeyType::Alphabetic => self.shift() ^ self.caps_lock(),
            KeyType::NumeralsAndPunctuation => self.shift(),
            KeyType::Numpad => !self.num_lock(),
            KeyType::Control => false,
        };

        if shifted { 1 } else { 0 }
    }

    /// Should be called whenever a key is pressed or released
    ///
    /// This alters the state and returns a KeyboardEvent
    pub fn actuate_key(&mut self, key: Key, pressed: bool) -> KeyboardEvent {
        let mut action = None;

        let mut repeat = false;
        if pressed {
            if self.key_pressed(key) {
                repeat = true;
            } else {
                self.pressed_keys.set(key.keycode() as usize, true);
            }
        } else {
            if !self.key_pressed(key) {
                repeat = true;
            } else {
                self.pressed_keys.set(key.keycode() as usize, false);
            }
        }

        if let Some(key_type) = key.key_type() {
            match key_type {
                KeyType::Alphabetic | KeyType::NumeralsAndPunctuation => {
                    if pressed {
                        // If a "control" modifier is also pressed, emit a command
                        // This handles cases like Ctrl+c and Alt+Tab
                        if self.ctrl() || self.alt() || self.sup() {
                            action = Some(KeyboardAction::Command(self.get_modifiers(), key));
                        } else {
                            // Get symbol from layout and return it if one exists
                            // This handles general typing
                            let group = self.get_group();
                            let key_level = self.get_key_level(&key_type);

                            if let Some(symbol) = self.layout.get_symbol(group, key_level, key) {
                                action = Some(KeyboardAction::Symbol(symbol));
                            }
                        }
                    }
                }
                KeyType::Control => {
                    if pressed {
                        if self.lock_keys_enabled {
                            match key {
                                keys::KEY_CAPS_LOCK => self.lock_keys.toggle(CAPS_LOCK),
                                keys::KEY_SCROLL_LOCK => self.lock_keys.toggle(SCROLL_LOCK),
                                keys::KEY_NUM_LOCK => self.lock_keys.toggle(NUM_LOCK),
                                _ => {}
                            }
                        }

                        action = Some(KeyboardAction::Command(self.get_modifiers(), key));
                    }
                }
                KeyType::Numpad => {
                    if self.num_lock() {
                        if pressed {
                            // Get symbol from layout
                            let group = self.get_group();

                            if let Some(symbol) = self.layout.get_symbol(group, 1, key) {
                                action = Some(KeyboardAction::Symbol(symbol));
                            }
                        }
                    } else {
                        if pressed {
                            // Remap to equivilent control key
                            let newkey = match key {
                                keys::KEY_NUM_7 => Some(keys::KEY_HOME),
                                keys::KEY_NUM_8 => Some(keys::KEY_UP),
                                keys::KEY_NUM_9 => Some(keys::KEY_PG_UP),
                                keys::KEY_NUM_4 => Some(keys::KEY_LEFT),
                                keys::KEY_NUM_5 => None,
                                keys::KEY_NUM_6 => Some(keys::KEY_RIGHT),
                                keys::KEY_NUM_1 => Some(keys::KEY_END),
                                keys::KEY_NUM_2 => Some(keys::KEY_DOWN),
                                keys::KEY_NUM_3 => Some(keys::KEY_PG_DOWN),
                                keys::KEY_NUM_0 => Some(keys::KEY_INS),
                                keys::KEY_NUM_DECIMAL => Some(keys::KEY_DEL),
                                _ => None,
                            };

                            if let Some(newkey) = newkey {
                                action = Some(KeyboardAction::Command(self.get_modifiers(), newkey));
                            }
                        }
                    }
                }
            }
        }

        KeyboardEvent {
            key: key,
            pressed: pressed,
            repeat: repeat,
            action: action,
        }
    }
}
