use keys::Key;
use state::Modifiers;

/// Represents logical keyboard actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardAction {
    /// A symbol has been inputted
    Symbol(char),

    /// A command has been run
    ///
    /// This action is used when either a command key (eg. enter, backspace, escape)
    /// is pressed or a command modifier (ctrl, alt, super) was used in conjunction
    /// with any other key. For example, Alt+Tab, Ctrl+c, Ctrl+Alt+Delete.
    Command(Modifiers, Key),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    /// The key that was actuated
    pub key: Key,

    /// true if they key was pressed. false if the key was released
    pub pressed: bool,

    /// true if the key is being held down, causing it to auto-repeat
    pub repeat: bool,

    /// If this event initiated a high-level action, details of that action will go here.
    ///
    /// For example: Pressing an alphabetic key will create a 'Symbol' action and pressing
    /// 'Alt+Tab' together will create a 'Command' action.
    pub action: Option<KeyboardAction>,
}
