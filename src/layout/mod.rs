use std::collections::HashMap;

use keys::Key;

#[derive(Debug, Clone)]
pub struct Layout {
    pub has_alt_gr_key: bool,
    pub key_symbol_map: HashMap<(u8, u8, Key), char>,
}

impl Layout {
    pub fn get_symbol(&self, group: u8, level: u8, key: Key) -> Option<char> {
        self.key_symbol_map.get(&(group, level, key)).cloned()
    }
}

mod us;
mod gb;

pub use self::us::us;
pub use self::gb::gb;
