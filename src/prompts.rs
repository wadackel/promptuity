//! A module that provides built-in prompts.
//!
//! # Prompts
//!
//! The `prompts` module provides the following built-in prompts.  
//! These built-in prompts can also be used as reference implementations when building your custom prompts.
//!
//! - [`Input`]: A prompt for general text input.
//! - [`Password`]: A text input prompt where the input is not displayed.
//! - [`Number`]: A prompt for inputting only integer values.
//! - [`Select`]: A prompt for selecting a single element from a list of options.
//! - [`MultiSelect`]: A prompt for selecting multiple elements from a list of options.
//! - [`Confirm`]: A prompt for inputting a Yes/No choice.
//!
//! # Examples
//!
//! This is an example of using a simple built-in prompt.
//!
//! ```no_run
//! use promptuity::themes::MinimalTheme;
//! use promptuity::prompts::{Input, Confirm};
//! use promptuity::{Promptuity, Term};
//!
//! # fn main() -> Result<(), promptuity::Error> {
//! let mut term = Term::default();
//! let mut theme = MinimalTheme::default();
//! let mut p = Promptuity::new(&mut term, &mut theme);
//!
//! p.begin()?;
//!
//! let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
//! let full = p.prompt(Confirm::new("Are you a full-time software developer?").with_default(true))?;
//!
//! p.finish()?;
//! # Ok(())
//! # }
//! ```

mod confirm;
mod input;
mod multi_select;
mod number;
mod password;
mod select;
#[cfg(test)]
pub(crate) mod test;

pub use confirm::*;
pub use input::*;
pub use multi_select::*;
pub use number::*;
pub use password::*;
pub use select::*;
