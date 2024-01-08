//! # Promptuity
//!
//! Promptuity is a library that provides interactive prompts. It is highly extensible, allowing you to build your original prompts from scratch. It brings ingenuity to various projects.
//!
//! ## Concept
//!
//! - **Not easy, But simple**
//!     - Avoids APIs with implicit behavior, aiming to provide as transparent APIs as possible.
//!     - The amount of code required to start a prompt may be more compared to other libraries.
//! - **Extensible**
//!     - You can customize built-in prompts or build your prompts from scratch.
//!     - The built-in prompts are minimal, assuming that prompt requirements vary by project.
//! - **Beautiful**
//!     - Offers two types of built-in Themes.
//!     - Themes can also be fully customized to fit your ideal.
//!
//! ## Quick Start
//!
//! The basic usage is as follows.
//!
//! ```no_run
//! use promptuity::prompts::{Confirm, Input, Select, SelectOption};
//! use promptuity::themes::FancyTheme;
//! use promptuity::{Error, Promptuity, Term};
//!
//! fn main() -> Result<(), Error> {
//!     let mut term = Term::default();
//!     let mut theme = FancyTheme::default();
//!     let mut p = Promptuity::new(&mut term, &mut theme);
//!
//!     p.term().clear()?;
//!
//!     p.with_intro("Survey").begin()?;
//!
//!     let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
//!
//!     let _ = p.prompt(Confirm::new("Are you a full-time software developer?").with_default(true))?;
//!
//!     let _ = p.prompt(
//!         Select::new(
//!             "Select your primary programming language",
//!             vec![
//!                 SelectOption::new("Rust", "rust"),
//!                 SelectOption::new("Go", "go"),
//!                 SelectOption::new("C++", "cpp"),
//!                 SelectOption::new("C", "c"),
//!                 SelectOption::new("TypeScript", "typescript"),
//!                 SelectOption::new("JavaScript", "javascript"),
//!                 SelectOption::new("Deno", "deno"),
//!                 SelectOption::new("Python", "python"),
//!                 SelectOption::new("Java", "java"),
//!                 SelectOption::new("Dart", "dart"),
//!                 SelectOption::new("Other", "other"),
//!             ],
//!         )
//!         .with_hint("Submit with Space or Enter."),
//!     )?;
//!
//!     p.with_outro(format!("Thank you for your response, {}!", name))
//!         .finish()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All errors are consolidated into [`promptuity::Error`](#).
//!
//! In many cases, prompt interruptions will need to be handled individually. Interruptions occur during user input reception, typically through inputs like <kbd>Ctrl</kbd> + <kbd>C</kbd> or <kbd>ESC</kbd>.
//!
//! ```rust
//! use promptuity::prompts::Input;
//! use promptuity::themes::MinimalTheme;
//! use promptuity::{Error, Promptuity, Term};
//!
//! fn ask() -> Result<String, Error> {
//!     let mut term = Term::default();
//!     let mut theme = MinimalTheme::default();
//!     let mut p = Promptuity::new(&mut term, &mut theme);
//!
//!     p.begin()?;
//!     let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
//!     p.finish()?;
//!
//!     Ok(name)
//! }
//!
//! fn main() {
//!     match ask() {
//!         Ok(name) => println!("Hello, {}!", name),
//!         Err(Error::Cancel) => {}
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! Prompt interruptions can be handled as [`Error::Cancel`]. In the above examples, no message is displayed in the event of an interruption.

pub mod event;
pub mod pagination;
pub mod prompts;
pub mod style;
pub mod themes;

mod error;
mod prompt;
mod term;
mod theme;

pub use error::*;
pub use prompt::*;
pub use term::*;
pub use theme::*;
