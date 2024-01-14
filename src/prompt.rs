use unicode_width::UnicodeWidthChar;

use crate::event::*;
use crate::{Error, RenderSnapshot, Terminal, Theme};

/// A struct to represent the input cursor.
///
/// `InputCursor` is used in prompts like [`crate::prompts::Input`], [`crate::prompts::Password`], and [`crate::prompts::Number`], proving useful in prompts where users input text.  
/// It supports maintaining and manipulating cursor position, as well as inserting text at the cursor location.
///
/// # Examples
///
/// ```
/// use promptuity::InputCursor;
///
/// let mut cursor = InputCursor::default();
///
/// cursor.insert('f');
/// cursor.insert('o');
/// cursor.insert('o');
/// assert_eq!(cursor.value(), "foo");
/// assert_eq!(cursor.cursor(), 3);
/// assert_eq!(cursor.col(), 3);
///
/// cursor.move_home();
/// cursor.insert('b');
/// cursor.insert('a');
/// cursor.insert('r');
/// cursor.insert(' ');
/// assert_eq!(cursor.value(), "bar foo");
/// assert_eq!(cursor.cursor(), 4);
/// assert_eq!(cursor.col(), 4);
/// ```
///
/// Supports multi-byte characters. (Does not support emojis)
///
/// ```
/// use promptuity::InputCursor;
///
/// let mut cursor = InputCursor::default();
///
/// cursor.insert('日');
/// cursor.insert('本');
/// cursor.insert('語');
/// cursor.delete_left_char();
/// assert_eq!(cursor.value(), "日本");
/// assert_eq!(cursor.cursor(), 2);
/// assert_eq!(cursor.col(), 4);
/// ```
#[derive(Debug, Clone)]
pub struct InputCursor {
    value: String,
    cursor: usize,
}

impl Default for InputCursor {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}

impl InputCursor {
    /// Creates a new [`InputCursor`] with the given value and cursor position.
    pub fn new(value: String, cursor: usize) -> Self {
        Self { value, cursor }
    }

    /// Creates a new [`InputCursor`] with the given value.  
    /// The cursor position is set at the end of the input string.
    ///
    /// # Examples
    ///
    /// ```
    /// use promptuity::InputCursor;
    ///
    /// let mut cursor = InputCursor::from("foo".into());
    /// assert_eq!(cursor.value(), "foo");
    /// assert_eq!(cursor.cursor(), 3);
    /// ```
    pub fn from(value: String) -> Self {
        let cursor = value.char_indices().count();
        Self { value, cursor }
    }

    fn chars(&self) -> std::str::CharIndices {
        self.value.char_indices()
    }

    fn len(&self) -> usize {
        self.chars().count()
    }

    fn char_at(&self, index: usize) -> Option<(usize, char)> {
        self.chars().nth(index)
    }

    /// Returns the column position of the cursor.
    pub fn col(&self) -> u16 {
        let col = self
            .chars()
            .take(self.cursor)
            .map(|(_, c)| c.width().unwrap_or(0))
            .sum::<usize>();
        u16::try_from(col).unwrap_or(0)
    }

    /// Returns the value.
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    /// Returns the cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Sets the cursor position.
    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.cursor = cursor;
        self
    }

    /// Returns a tuple split into the string at the cursor position and its surrounding text.  
    /// `split` is useful when styling the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use promptuity::InputCursor;
    ///
    /// let mut cursor = InputCursor::from("Hello".into());
    /// cursor.move_left();
    /// cursor.move_left();
    /// assert_eq!(cursor.split(), ("Hel".into(), "l".into(), "o".into()));
    /// ```
    pub fn split(&self) -> (String, String, String) {
        let (left, mut cursor, right) = self.chars().enumerate().fold(
            (String::new(), String::new(), String::new()),
            |(mut left, mut cursor, mut right), (i, (_, c))| {
                match i.cmp(&self.cursor) {
                    std::cmp::Ordering::Less => {
                        left.push(c);
                    }
                    std::cmp::Ordering::Equal => {
                        cursor.push(c);
                    }
                    std::cmp::Ordering::Greater => {
                        right.push(c);
                    }
                }
                (left, cursor, right)
            },
        );

        if cursor.is_empty() && self.cursor.saturating_add(1) > self.len() {
            cursor.push(' ');
        }

        (left, cursor, right)
    }

    /// Returns whether the input is empty or not.
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }

    /// Moves the cursor one character to the left.
    pub fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Moves the cursor one character to the right.
    pub fn move_right(&mut self) {
        self.cursor = std::cmp::min(self.cursor.saturating_add(1), self.len());
    }

    /// Moves the cursor to the beginning of the line.
    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// Moves the cursor to the end of the line.
    pub fn move_end(&mut self) {
        self.cursor = self.len();
    }

    /// Inserts a character at the cursor position.
    pub fn insert(&mut self, chr: char) {
        match self.char_at(self.cursor) {
            Some((i, _)) => {
                self.value.insert(i, chr);
            }
            None => {
                self.value.push(chr);
            }
        }
        self.cursor = self.cursor.saturating_add(1);
    }

    /// Deletes the character to the left of the cursor.
    pub fn delete_left_char(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let cursor = self.cursor.saturating_sub(1);
        let found = self.char_at(cursor).map(|(i, _)| i);
        if let Some(i) = found {
            self.value.remove(i);
            self.cursor = cursor;
        }
    }

    fn prev_word_index(&mut self) -> usize {
        let mut found_word = false;
        let chars = self
            .chars()
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .take(self.cursor)
            .rev();

        for (i, (_, c)) in chars {
            if c.is_whitespace() {
                if found_word {
                    return i.saturating_add(1);
                }
            } else {
                found_word = true;
            }
        }

        0
    }

    /// Deletes the word to the left of the cursor.
    pub fn delete_left_word(&mut self) {
        let start = self.prev_word_index();
        let mut value = String::new();
        for (i, (_, c)) in self.chars().enumerate() {
            if i < start || self.cursor <= i {
                value.push(c);
            }
        }
        self.value = value;
        self.cursor = start;
    }

    /// Deletes the character to the right of the cursor.
    pub fn delete_right_char(&mut self) {
        if self.cursor >= self.len() {
            return;
        }

        let found = self.char_at(self.cursor).map(|(i, _)| i);
        if let Some(i) = found {
            self.value.remove(i);
        }
    }

    /// Deletes characters to the right of the cursor up to the end of the line.
    pub fn delete_rest_line(&mut self) {
        let found = self.char_at(self.cursor).map(|(i, _)| i);
        if let Some(i) = found {
            self.value = self.value[..i].to_string();
        }
    }

    /// Deletes the entire line.
    pub fn delete_line(&mut self) {
        self.cursor = 0;
        self.value = String::new();
    }
}

/// A struct representing the state of the prompt.
///
/// Controls the rendering of the prompt and the flow of actions like submission.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptState {
    /// An active state that accepts user inputs like key presses.
    Active,
    /// A state where the value is finalized.
    Submit,
    /// A state where the prompt is cancelled. e.g. `Esc` or `Ctrl-C`.
    Cancel,
    /// A state for recoverable errors. e.g. validation errors.
    Error(String),
    /// A state for unrecoverable errors.
    Fatal(String),
}

impl std::fmt::Display for PromptState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptState::Active => write!(f, "Active"),
            PromptState::Submit => write!(f, "Submit"),
            PromptState::Cancel => write!(f, "Cancel"),
            PromptState::Error(msg) => write!(f, "Error({})", msg),
            PromptState::Fatal(msg) => write!(f, "Fatal({})", msg),
        }
    }
}

/// A struct representing the input of the prompt.
#[derive(Debug)]
pub enum PromptInput {
    /// Do not use Input.
    None,
    /// Use [`InputCursor`].
    Cursor(InputCursor),
    /// Display Input as a string.
    Raw(String),
}

impl Default for PromptInput {
    fn default() -> Self {
        Self::None
    }
}

/// A struct representing the body of the prompt.
#[derive(Debug)]
pub enum PromptBody {
    /// Do not use Body.
    None,
    /// Display Body as a string.
    Raw(String),
}

impl Default for PromptBody {
    fn default() -> Self {
        Self::None
    }
}

/// A trait for performing prompt validation.
pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), String>;
}

impl<T, F> Validator<T> for F
where
    F: Fn(&T) -> Result<(), String>,
{
    fn validate(&self, value: &T) -> Result<(), String> {
        self(value)
    }
}

/// A struct representing the payload required for prompt rendering.
#[derive(Debug, Default)]
pub struct RenderPayload {
    pub input: PromptInput,
    pub body: PromptBody,
    pub message: String,
    pub hint: Option<String>,
    pub placeholder: Option<String>,
}

impl RenderPayload {
    /// Creates a new [`RenderPayload`].
    pub fn new(message: String, hint: Option<String>, placeholder: Option<String>) -> Self {
        Self {
            message,
            hint,
            placeholder,
            ..Default::default()
        }
    }

    /// Sets the input for the payload.
    pub fn input(mut self, input: PromptInput) -> Self {
        self.input = input;
        self
    }

    /// Sets the body for the payload.
    pub fn body(mut self, body: PromptBody) -> Self {
        self.body = body;
        self
    }
}

/// A trait representing the behavior of a prompt.
pub trait Prompt {
    /// The type returned as a result by the prompt.
    type Output;

    /// Sets up the prompt.  
    /// A lifecycle method for validating and initializing settings.
    fn setup(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Validates the prompt.  
    /// If returning an error, please return the error message as a `String`.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// Handles key presses.  
    /// Allows changing the internal state of the prompt in response to key inputs.
    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PromptState;

    /// Submits the prompt.  
    /// Called as a result of [`Prompt::handle`] returning [`PromptState::Submit`], it returns the final value.
    fn submit(&mut self) -> Self::Output;

    /// Renders the prompt.  
    /// Performs rendering based on the value of [`PromptState`].  
    /// If returning an error, please return the error message as a `String`.
    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String>;
}

/// The core struct of `promptuity`.
///
/// # Examples
///
/// ## Basic
///
/// ```no_run
/// use promptuity::themes::MinimalTheme;
/// use promptuity::prompts::{Input, Confirm};
/// use promptuity::{Promptuity, Term};
///
/// # fn main() -> Result<(), promptuity::Error> {
/// let mut term = Term::default();
/// let mut theme = MinimalTheme::default();
/// let mut p = Promptuity::new(&mut term, &mut theme);
///
/// p.begin()?;
///
/// let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
/// let full = p.prompt(Confirm::new("Are you a full-time software developer?").with_default(true))?;
///
/// p.finish()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Logging
///
/// Executing `begin` activates [Raw Mode](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode). Since log outputs like `println!` will no longer render correctly, if you need to output logs, please use log methods such as [`Promptuity::log`] or [`Promptuity::warn`].
///
/// ```no_run
/// # use promptuity::themes::MinimalTheme;
/// # use promptuity::prompts::{Input, Confirm};
/// # use promptuity::{Promptuity, Term};
/// # fn main() -> Result<(), promptuity::Error> {
/// # let mut term = Term::default();
/// # let mut theme = MinimalTheme::default();
/// # let mut p = Promptuity::new(&mut term, &mut theme);
/// p.begin()?;
///
/// p.step("Logging Step")?;
/// p.log("This is a log message")?;
/// p.info("This is a info message")?;
/// p.warn("This is a warn message")?;
/// p.error("This is a error message")?;
/// p.success("This is a success message")?;
///
/// p.finish()?;
/// # Ok(())
/// # }
/// ```
pub struct Promptuity<'a, W: std::io::Write> {
    term: &'a mut dyn Terminal<W>,
    theme: &'a mut dyn Theme<W>,
    state: PromptState,
    intro: Option<String>,
    outro: Option<String>,
    finished: bool,
}

impl<'a, W: std::io::Write> Drop for Promptuity<'a, W> {
    fn drop(&mut self) {
        if !self.finished {
            self.term.disable_raw().expect("Failed to disable raw mode");
        }
    }
}

impl<'a, W: std::io::Write> Promptuity<'a, W> {
    /// Creates a new [`Promptuity`] instance.
    pub fn new(term: &'a mut dyn Terminal<W>, theme: &'a mut dyn Theme<W>) -> Self {
        Self {
            term,
            theme,
            state: PromptState::Active,
            intro: None,
            outro: None,
            finished: false,
        }
    }

    /// Returns the currently used terminal.
    pub fn term(&mut self) -> &mut dyn Terminal<W> {
        self.term
    }

    /// Sets the intro message for the prompt session.  
    /// May be required by the Theme.
    pub fn with_intro(&mut self, intro: impl std::fmt::Display) -> &mut Self {
        self.intro = Some(intro.to_string());
        self
    }

    /// Sets the outro message for the prompt session.
    /// May be required by the Theme.
    pub fn with_outro(&mut self, outro: impl std::fmt::Display) -> &mut Self {
        self.outro = Some(outro.to_string());
        self
    }

    /// Declares the start of a prompt session.  
    /// Executing `begin` activates [Raw Mode](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode). Since log outputs like `println!` will no longer render correctly, if you need to output logs, please use log methods such as [`Promptuity::log`] or [`Promptuity::warn`].
    pub fn begin(&mut self) -> Result<(), Error> {
        self.term.enable_raw()?;
        self.theme.begin(self.term, self.intro.clone())?;
        Ok(())
    }

    /// Declares the end of a prompt session.  
    /// Executing `finish` deactivates [Raw Mode](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode).
    pub fn finish(&mut self) -> Result<(), Error> {
        self.theme
            .finish(self.term, &self.state, self.outro.clone())?;
        self.term.disable_raw()?;
        self.finished = true;
        Ok(())
    }

    /// Displays a message as a prompt step.
    pub fn step(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.step(self.term, message.to_string())?;
        Ok(())
    }

    /// Output of messages without decoration.
    pub fn log(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.log(self.term, message.to_string())?;
        Ok(())
    }

    /// Output of messages with info decoration.
    pub fn info(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.info(self.term, message.to_string())?;
        Ok(())
    }

    /// Output of messages with warning decoration.
    pub fn warn(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.warn(self.term, message.to_string())?;
        Ok(())
    }

    /// Output of messages with error decoration.
    pub fn error(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.error(self.term, message.to_string())?;
        Ok(())
    }

    /// Output of messages with success decoration.
    pub fn success(&mut self, message: impl std::fmt::Display) -> Result<(), Error> {
        self.theme.success(self.term, message.to_string())?;
        Ok(())
    }

    /// Executes the specified prompt and returns the input result.
    pub fn prompt<O>(&mut self, prompt: &mut dyn Prompt<Output = O>) -> Result<O, Error> {
        prompt.setup()?;

        self.state = PromptState::Active;

        self.render(prompt)?;

        loop {
            let (code, modifiers) = self.term.read_key()?;
            let state = prompt.handle(code, modifiers);

            self.state = match state {
                PromptState::Submit => {
                    if let Err(msg) = prompt.validate() {
                        PromptState::Error(msg)
                    } else {
                        PromptState::Submit
                    }
                }
                state => state,
            };

            self.render(prompt)?;

            match self.state.clone() {
                PromptState::Cancel => {
                    self.finish()?;
                    return Err(Error::Cancel);
                }
                PromptState::Fatal(msg) => {
                    self.finish()?;
                    return Err(Error::Prompt(msg));
                }
                PromptState::Submit => {
                    return Ok(prompt.submit());
                }
                _ => {}
            }
        }
    }

    fn render<O>(&mut self, prompt: &mut dyn Prompt<Output = O>) -> Result<(), Error> {
        let res = prompt.render(&self.state).map_err(Error::Prompt)?;

        self.theme.render(
            self.term,
            RenderSnapshot {
                state: &self.state,
                input: res.input,
                body: res.body,
                message: res.message,
                hint: res.hint,
                placeholder: res.placeholder,
            },
        )?;

        Ok(())
    }
}
