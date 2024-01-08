//! A module to read events.
//!
//! # Event
//!
//! The `event` module provides functionalities used in prompt handlers.
//! The main offering is the direct re-export of [`crossterm::event`].
//!
//! ## Examples
//!
//! ```ignore
//! use promptuity::event::{KeyCode, KeyModifiers};
//!
//! impl<W: std::io::Write> Prompt<W> for YourPrompt {
//!   // ...
//!   fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PromptState {
//!     match (code, modifiers) {
//!       (KeyCode::Enter, KeyModifiers::NONE) => PromptState::Submit,
//!       _ => PromptState::Active,
//!     }
//!   }
//!   // ...
//! }
//! ```

/// [`KeyCode`] re-exports from [`crossterm::event`].
pub use crossterm::event::KeyCode;
/// [`KeyModifiers`] re-exports from [`crossterm::event`].
pub use crossterm::event::KeyModifiers;
