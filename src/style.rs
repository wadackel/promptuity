//! A module to apply attributes and colors on your text.
//!
//! # Style
//!
//! The `style` module offers functionality to apply styling to your text.
//! In promptuity, it exposes a simple API that wraps the features of [`crossterm::style`].
//!
//! ## Examples
//!
//! This is an example of styling text.
//!
//! ```
//! use promptuity::style::{Color, Styled};
//!
//! let styled_text = Styled::new("Colored text").fg(Color::Green).bold().to_string();
//!
//! assert_eq!(styled_text, "\u{1b}[38;5;10m\u{1b}[1mColored text\u{1b}[0m");
//! ```
//!
//! This is an example of changing the displayed text based on whether Unicode is supported or not.  
//! Useful for representing icons and other symbols.
//!
//! ```no_run
//! use promptuity::style::Symbol;
//!
//! //              Unicode (main)       ASCII (fallback)
//! //                            \     /
//! let unselected_icon = Symbol("◯", "[ ]");
//! let selected_icon = Symbol("◉", "[x]");
//! ```

/// [`Color`] re-exports from [`crossterm::style::Color`].
pub use crossterm::style::Color;
use crossterm::style::{Attribute, Attributes, ContentStyle, Stylize};

/// A styling utility for strings wrapped in [`crossterm::style::ContentStyle`].
#[derive(Debug)]
pub struct Styled {
    content: String,
    fg: Option<Color>,
    bg: Option<Color>,
    attr: Option<Attributes>,
}

impl Styled {
    pub fn new(content: impl std::fmt::Display) -> Self {
        Self {
            content: content.to_string(),
            fg: None,
            bg: None,
            attr: None,
        }
    }

    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.bg = Some(color);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.attr(Attribute::Bold)
    }

    pub fn italic(&mut self) -> &mut Self {
        self.attr(Attribute::Italic)
    }

    pub fn underline(&mut self) -> &mut Self {
        self.attr(Attribute::Underlined)
    }

    pub fn dim(&mut self) -> &mut Self {
        self.attr(Attribute::Dim)
    }

    pub fn rev(&mut self) -> &mut Self {
        self.attr(Attribute::Reverse)
    }

    pub fn blink(&mut self) -> &mut Self {
        self.attr(Attribute::SlowBlink)
    }

    fn attr(&mut self, attr: Attribute) -> &mut Self {
        match self.attr {
            Some(ref mut attrs) => attrs.set(attr),
            None => self.attr = Some(Attributes::from(attr)),
        }
        self
    }
}

impl std::fmt::Display for Styled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = ContentStyle::new();
        if let Some(fg) = self.fg {
            style = style.with(fg);
        }
        if let Some(bg) = self.bg {
            style = style.on(bg);
        }
        if let Some(attr) = &self.attr {
            style.attributes = *attr;
        }
        write!(f, "{}", style.apply(self.content.as_str()))
    }
}

#[cfg(windows)]
fn is_unicode_supported() -> bool {
    use std::env;

    if env::var("WT_SESSION").is_ok() {
        return true;
    }

    if let Ok(program) = env::var("TERM_PROGRAM") {
        match program.as_str() {
            "vscode" => return true,
            _ => {}
        }
    }

    if let Ok(term) = env::var("TERM") {
        match term.as_str() {
            "xterm-256color" | "alacritty" => return true,
            _ => {}
        }
    }

    false
}

#[cfg(unix)]
fn is_unicode_supported() -> bool {
    true
}

/// A symbol that can be displayed in unicode or ascii.
#[derive(Debug, Copy, Clone)]
pub struct Symbol<'a, 'b>(
    /// The main symbol to display when unicode is supported.
    pub &'a str,
    /// The fallback symbol to display when unicode is not supported.
    pub &'b str,
);

impl<'a, 'b> Symbol<'a, 'b> {
    /// Creates a new [`Symbol`].
    pub fn new(main: &'a str, fallback: &'b str) -> Self {
        Self(main, fallback)
    }
}

impl<'a, 'b> std::fmt::Display for Symbol<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if is_unicode_supported() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}
