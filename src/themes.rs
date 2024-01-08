//! A module that provides built-in themes.
//!
//! # Themes
//!
//! The `themes` module provides the following built-in themes.
//!
//! - [`MinimalTheme`]: A Theme that offers a compact and minimalistic display.
//! - [`FancyTheme`]: A Theme that displays with a rich UI.

mod fancy;
mod minimal;

pub use fancy::*;
pub use minimal::*;
