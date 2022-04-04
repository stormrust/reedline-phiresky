use crate::{enums::ReedlineEvent, PromptEditMode};
use crossterm::event::Event;

/// Define the style of parsing for the edit events
/// Available default options:
/// - Emacs
/// - Vi
pub trait EditMode: Send {
    /// Translate the given user input event into what the `LineEditor` understands
    fn parse_event(&mut self, event: Event) -> ReedlineEvent;

    /// What to display in the prompt indicator
    fn edit_mode(&self) -> PromptEditMode;
}
