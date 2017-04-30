extern crate orbkb;

use orbkb::keys;
use orbkb::event::KeyboardAction;
use orbkb::layout::GB;
use orbkb::state::KeyboardState;

#[test]
fn test_simple() {
    let mut kb_state = KeyboardState::new(&GB);

    // Press 'a' key, should emit 'a'
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('a')));
    kb_state.actuate_key(keys::KEY_A, false);

    // Press '1' key, should emit '1'
    let result = kb_state.actuate_key(keys::KEY_1, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('1')));
    kb_state.actuate_key(keys::KEY_1, false);
}

#[test]
fn test_shift_modifier() {
    let mut kb_state = KeyboardState::new(&GB);

    // Press the shift key
    kb_state.actuate_key(keys::KEY_L_SHIFT, true);

    // Press 'a' key, should emit 'A'
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('A')));
    kb_state.actuate_key(keys::KEY_A, false);

    // Press '1' should emit '!'
    let result = kb_state.actuate_key(keys::KEY_1, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('!')));
    kb_state.actuate_key(keys::KEY_1, false);

    // Release shift key
    kb_state.actuate_key(keys::KEY_L_SHIFT, false);

    // Press 'a' key, should now emit 'a'
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('a')));
    kb_state.actuate_key(keys::KEY_A, false);
}

#[test]
fn test_alt_gr_modifier() {
    let mut kb_state = KeyboardState::new(&GB);

    // Press the alt gr key
    kb_state.actuate_key(keys::KEY_ALT_GR, true);

    // Press '`' key, should emit '|'
    let result = kb_state.actuate_key(keys::KEY_BACKTICK, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('|')));
    kb_state.actuate_key(keys::KEY_BACKTICK, false);
}

#[test]
fn test_caps_lock() {
    let mut kb_state = KeyboardState::new(&GB);

    assert_eq!(kb_state.caps_lock(), false);

    // Press and release the caps lock key
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, true);
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, false);

    assert_eq!(kb_state.caps_lock(), true);

    // Press 'a' key, should emit 'A'
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('A')));
    kb_state.actuate_key(keys::KEY_A, false);

    // Press '1' key, should emit '1'
    // Note: numerals and punctuation are unaffected by caps lock
    let result = kb_state.actuate_key(keys::KEY_1, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('1')));
    kb_state.actuate_key(keys::KEY_1, false);

    // Press the shift key
    kb_state.actuate_key(keys::KEY_L_SHIFT, true);

    // Press 'a' key, should emit 'a'
    // Note: The shift key negates the caps lock
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('a')));
    kb_state.actuate_key(keys::KEY_A, false);

    // Press '1' key, should emit '!'
    let result = kb_state.actuate_key(keys::KEY_1, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('!')));
    kb_state.actuate_key(keys::KEY_1, false);

    // Release the shift key
    kb_state.actuate_key(keys::KEY_L_SHIFT, false);

    // Press and release the caps lock key again
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, true);
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, false);

    assert_eq!(kb_state.caps_lock(), false);

    // Press 'a' key, should emit 'a'
    let result = kb_state.actuate_key(keys::KEY_A, true);
    assert_eq!(result.action, Some(KeyboardAction::Symbol('a')));
    kb_state.actuate_key(keys::KEY_A, false);
}

#[test]
fn test_num_lock() {
    let mut kb_state = KeyboardState::new(&GB);

    assert_eq!(kb_state.num_lock(), false);

    // Press and release the num lock key
    kb_state.actuate_key(keys::KEY_NUM_LOCK, true);
    kb_state.actuate_key(keys::KEY_NUM_LOCK, false);

    assert_eq!(kb_state.num_lock(), true);

    // TODO tests

    // Press and release the num lock key again
    kb_state.actuate_key(keys::KEY_NUM_LOCK, true);
    kb_state.actuate_key(keys::KEY_NUM_LOCK, false);

    assert_eq!(kb_state.num_lock(), false);
}

#[test]
fn test_scroll_lock() {
    let mut kb_state = KeyboardState::new(&GB);

    assert_eq!(kb_state.scroll_lock(), false);

    // Press and release the scroll lock key
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, true);
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, false);

    assert_eq!(kb_state.scroll_lock(), true);

    // Press and release the scroll lock key again
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, true);
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, false);

    assert_eq!(kb_state.scroll_lock(), false);
}

#[test]
fn test_disable_lock_keys() {
    let mut kb_state = KeyboardState::new(&GB);
    kb_state.lock_keys_enabled = false;

    assert_eq!(kb_state.caps_lock(), false);
    assert_eq!(kb_state.num_lock(), false);
    assert_eq!(kb_state.scroll_lock(), false);

    // Press and release the caps lock key
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, true);
    kb_state.actuate_key(keys::KEY_CAPS_LOCK, false);

    assert_eq!(kb_state.caps_lock(), false);

    // Press and release the num lock key
    kb_state.actuate_key(keys::KEY_NUM_LOCK, true);
    kb_state.actuate_key(keys::KEY_NUM_LOCK, false);

    assert_eq!(kb_state.num_lock(), false);

    // Press and release the scroll lock key
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, true);
    kb_state.actuate_key(keys::KEY_SCROLL_LOCK, false);

    assert_eq!(kb_state.scroll_lock(), false);
}
