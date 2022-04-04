//! # reedline `\|/`
//! # A readline replacement written in Rust
//!
//! Reedline is a project to create a line editor (like bash's `readline` or
//! zsh's `zle`) that supports many of the modern conveniences of CLIs,
//! including syntax highlighting, completions, multiline support, Unicode
//! support, and more.  It is currently primarily developed as the interactive
//! editor for [nushell](https://github.com/nushell/nushell) (starting with
//! `v0.60`) striving to provide a pleasant interactive experience.
//!
//! ## Basic example
//!
//! ```rust,no_run
//! // Create a default reedline object to handle user input
//!
//! use reedline::{DefaultPrompt, Reedline, Signal};
//!
//!  let mut line_editor = Reedline::create();
//!  let prompt = DefaultPrompt::default();
//!
//!  loop {
//!      let sig = line_editor.read_line(&prompt);
//!      match sig {
//!          Ok(Signal::Success(buffer)) => {
//!              println!("We processed: {}", buffer);
//!          }
//!          Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
//!              println!("\nAborted!");
//!              break;
//!          }
//!          Ok(Signal::CtrlL) => {
//!              line_editor.clear_screen();
//!          }
//!          x => {
//!              println!("Event: {:?}", x);
//!          }
//!      }
//!  }
//! ```
//! ## Integrate with custom keybindings
//!
//! ```rust,no_run
//! // Configure reedline with custom keybindings
//!
//! //Cargo.toml
//! //    [dependencies]
//! //    crossterm = "*"
//!
//! use {
//!   crossterm::event::{KeyCode, KeyModifiers},
//!   reedline::{default_emacs_keybindings, EditCommand, Reedline, Emacs, ReedlineEvent},
//! };
//!
//! let mut keybindings = default_emacs_keybindings();
//! keybindings.add_binding(
//!     KeyModifiers::ALT,
//!     KeyCode::Char('m'),
//!     ReedlineEvent::Edit(vec![EditCommand::BackspaceWord]),
//! );
//! let edit_mode = Box::new(Emacs::new(keybindings));
//!
//! let mut line_editor = Reedline::create().with_edit_mode(edit_mode);
//! ```
//!
//! ## Integrate with [`History`]
//!
//! ```rust,no_run
//! // Create a reedline object with history support, including history size limits
//!
//! use reedline::{FileBackedHistory, Reedline};
//!
//! let history = Box::new(
//!     FileBackedHistory::with_file(5, "history.txt".into())
//!         .expect("Error configuring history with file"),
//! );
//! let mut line_editor = Reedline::create()
//!     .with_history(history);
//! ```
//!
//! ## Integrate with custom syntax [`Highlighter`]
//!
//! ```rust,no_run
//! // Create a reedline object with highlighter support
//!
//! use reedline::{ExampleHighlighter, Reedline};
//!
//! let commands = vec![
//!   "test".into(),
//!   "hello world".into(),
//!   "hello world reedline".into(),
//!   "this is the reedline crate".into(),
//! ];
//! let mut line_editor =
//! Reedline::create().with_highlighter(Box::new(ExampleHighlighter::new(commands)));
//! ```
//!
//! ## Integrate with custom tab completion
//!
//! ```rust,no_run
//! // Create a reedline object with tab completions support
//!
//! use reedline::{DefaultCompleter, Reedline, CompletionMenu};
//!
//! let commands = vec![
//!   "test".into(),
//!   "hello world".into(),
//!   "hello world reedline".into(),
//!   "this is the reedline crate".into(),
//! ];
//! let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
//! // Use the interactive menu to select options from the completer
//! let completion_menu = Box::new(CompletionMenu::default());
//!
//! let mut line_editor = Reedline::create().with_completer(completer).with_menu(completion_menu, None);
//! ```
//!
//! ## Integrate with [`Hinter`] for fish-style history autosuggestions
//!
//! ```rust,no_run
//! // Create a reedline object with in-line hint support
//!
//! //Cargo.toml
//! //    [dependencies]
//! //    nu-ansi-term = "*"
//!
//! use {
//!   nu_ansi_term::{Color, Style},
//!   reedline::{DefaultHinter, Reedline},
//! };
//!
//!
//! let mut line_editor = Reedline::create().with_hinter(Box::new(
//!   DefaultHinter::default()
//!   .with_style(Style::new().italic().fg(Color::LightGray)),
//! ));
//! ```
//!
//! ## Are we prompt yet? (Development status)
//!
//! Nushell has now all the basic features to become the primary line editor for [nushell](https://github.com/nushell/nushell
//! )
//!
//! - General editing functionality, that should feel familiar coming from other shells (e.g. bash, fish, zsh).
//! - Configurable keybindings (emacs-style bindings and basic vi-style).
//! - Configurable prompt
//! - Content-aware syntax highlighting.
//! - Autocompletion (With graphical selection menu or simple cycling inline).
//! - History with interactive search options (optionally persists to file, can support multilple sessions accessing the same file)
//! - Fish-style history autosuggestion hints
//! - Undo support.
//! - Clipboard integration
//! - Line completeness validation for seamless entry of multiline command sequences.
//!
//! ### Areas for future improvements
//!
//! - [ ] Support for Unicode beyond simple left-to-right scripts
//! - [ ] Easier keybinding configuration
//! - [ ] Support for more advanced vi commands
//! - [ ] Visual selection
//! - [ ] Smooth experience if completion or prompt content takes long to compute
//! - [ ] Support for a concurrent output stream from background tasks to be displayed, while the input prompt is active. ("Full duplex" mode)
//!
//! For more ideas check out the [feature discussion](https://github.com/nushell/reedline/issues/63) or hop on the `#reedline` channel of the [nushell discord](https://discordapp.com/invite/NtAbbGn).
//!
//! ### Development history
//!
//! If you want to follow along with the history how reedline got started, you can watch the [recordings](https://youtube.com/playlist?list=PLP2yfE2-FXdQw0I6O4YdIX_mzBeF5TDdv) of [JT](https://github.com/jntrnr)`s [live-coding streams](https://www.twitch.tv/jntrnr).
//!
//! [Playlist: Creating a line editor in Rust](https://youtube.com/playlist?list=PLP2yfE2-FXdQw0I6O4YdIX_mzBeF5TDdv)
//!
//! ### Alternatives
//!
//! For currently more mature Rust line editing check out:
//!
//! - [rustyline](https://crates.io/crates/rustyline)
//!
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(missing_docs)]
// #![deny(warnings)]
mod core_editor;
pub use core_editor::LineBuffer;

mod enums;
pub use enums::{EditCommand, ReedlineEvent, Signal, UndoBehavior};

mod painting;
pub use painting::{Painter, StyledText};

mod engine;
pub use engine::Reedline;

mod history;
pub use history::{FileBackedHistory, History, HistoryNavigationQuery, HISTORY_SIZE};
#[cfg(feature="sqlite")]
pub use history::{SqliteBackedHistory};

mod prompt;
pub use prompt::{
    DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus,
    PromptViMode,
};

mod edit_mode;
pub use edit_mode::{
    default_emacs_keybindings, default_vi_insert_keybindings, default_vi_normal_keybindings,
    EditMode, Emacs, Keybindings, Vi,
};

mod highlighter;
pub use highlighter::{ExampleHighlighter, Highlighter, SimpleMatchHighlighter};

mod completion;
pub use completion::{Completer, DefaultCompleter, Span, Suggestion};

mod hinter;
pub use hinter::{DefaultHinter, Hinter};

mod validator;
pub use validator::{DefaultValidator, ValidationResult, Validator};

mod menu;
pub use menu::{menu_functions, CompletionMenu, HistoryMenu, Menu, MenuEvent, MenuTextStyle};

mod utils;
pub use utils::{
    get_reedline_default_keybindings, get_reedline_edit_commands,
    get_reedline_keybinding_modifiers, get_reedline_keycodes, get_reedline_prompt_edit_modes,
    get_reedline_reedline_events,
};
