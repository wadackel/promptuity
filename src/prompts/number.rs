use crate::event::*;
use crate::{Error, InputCursor, Prompt, PromptInput, PromptState, RenderPayload, Validator};

/// A trait for formatting the [`Number`] prompt.
///
/// All methods have default implementations, allowing you to override only the specific formatting process you need.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Number, NumberFormatter};
///
/// struct CustomFormatter;
///
/// impl NumberFormatter for CustomFormatter {
///     fn err_invalid_range(&self, min: isize, max: isize) -> String {
///         format!("Invalid Range: expect min={}, max={}.", min, max)
///     }
/// }
///
/// let _ = Number::new("...").with_formatter(CustomFormatter);
/// ```
pub trait NumberFormatter {
    /// Formats the error message when the input is empty and required.
    fn err_required(&self) -> String {
        "This field is required.".into()
    }

    /// Formats the error message when the input is not a number.
    fn err_invalid_format(&self) -> String {
        "Invalid number.".into()
    }

    /// Formats the error message when the input is not within the range.
    fn err_invalid_range(&self, min: isize, max: isize) -> String {
        format!("Must be a number between {} and {}.", min, max)
    }
}

/// The default formatter for [`Number`].
pub struct DefaultNumberFormatter;

impl DefaultNumberFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl NumberFormatter for DefaultNumberFormatter {}

/// A prompt for inputting only integer values.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`NumberFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Placeholder**: An auxiliary message displayed when no input is given.
/// - **Required**: A flag indicating whether to allow no input.
/// - **Min Value**: The minimum value of `isize`. Defaults to `isize::MIN`.
/// - **Max Value**: The maximum value of `isize`. Defaults to `isize::MAX`.
/// - **Default Value**: The default value of `isize`.
/// - **Validator**: A function to validate the value at the time of submission.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::Number;
///
/// let _ = Number::new("How old are you?").with_min(0).with_max(120);
/// ```
pub struct Number {
    formatter: Box<dyn NumberFormatter>,
    message: String,
    hint: Option<String>,
    placeholder: Option<String>,
    required: bool,
    min: isize,
    max: isize,
    validator: Option<Box<dyn Validator<String>>>,
    input: InputCursor,
}

impl Number {
    /// Creates a new [`Number`] prompt.
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            formatter: Box::new(DefaultNumberFormatter::new()),
            message: message.to_string(),
            hint: None,
            placeholder: None,
            required: true,
            validator: None,
            min: isize::MIN,
            max: isize::MAX,
            input: InputCursor::new(String::new(), 0),
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl NumberFormatter + 'static) -> &mut Self {
        self.formatter = Box::new(formatter);
        self
    }

    /// Sets the hint message for the prompt.
    pub fn with_hint(&mut self, hint: impl std::fmt::Display) -> &mut Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Sets the placeholder message for the prompt.
    pub fn with_placeholder(&mut self, placeholder: impl std::fmt::Display) -> &mut Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }

    /// Sets the required flag for the prompt.
    pub fn with_required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }

    /// Sets the minimum value for the prompt.
    pub fn with_min(&mut self, value: isize) -> &mut Self {
        self.min = value;
        self
    }

    /// Sets the maximum value for the prompt.
    pub fn with_max(&mut self, value: isize) -> &mut Self {
        self.max = value;
        self
    }

    /// Sets the default value for the prompt.
    pub fn with_default(&mut self, value: isize) -> &mut Self {
        let v = value.to_string();
        let len = v.len();
        self.input = InputCursor::new(v, len);
        self
    }

    /// Sets the validator for the prompt.
    pub fn with_validator(&mut self, f: impl Validator<String> + 'static) -> &mut Self {
        self.validator = Some(Box::new(move |value: &String| -> Result<(), String> {
            f.validate(value).map_err(|err| err.to_string())
        }));
        self
    }

    fn value(&self) -> isize {
        self.input.value().parse::<isize>().unwrap_or_default()
    }

    fn starts_with_op(&self) -> bool {
        let value = self.input.value();
        value.starts_with('-') || value.starts_with('+')
    }

    fn is_within_range(&self, value: isize) -> bool {
        self.min <= value && value <= self.max
    }

    fn normalize_value(&self, value: isize) -> isize {
        std::cmp::max(self.min, std::cmp::min(self.max, value))
    }

    fn insert(&mut self, chr: char) {
        match chr {
            '0'..='9' => {
                if self.input.cursor() == 0 && self.starts_with_op() {
                    self.input.move_right();
                }
                self.input.insert(chr);
            }
            '-' | '+' => {
                if self.input.cursor() == 0 && !self.starts_with_op() {
                    self.input.insert(chr);
                }
            }
            _ => {}
        }
    }

    fn increment(&mut self) {
        let value = self.value().saturating_add(1);
        self.input = InputCursor::from(self.normalize_value(value).to_string());
    }

    fn decrement(&mut self) {
        let value = self.value().saturating_sub(1);
        self.input = InputCursor::from(self.normalize_value(value).to_string());
    }
}

impl AsMut<Number> for Number {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Prompt for Number {
    type Output = isize;

    fn setup(&mut self) -> Result<(), crate::Error> {
        if self.min > self.max {
            return Err(Error::Config(format!(
                "min cannot be greater than max (min={}, max={})",
                self.min, self.max
            )));
        }

        Ok(())
    }

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PromptState {
        match (code, modifiers) {
            (KeyCode::Enter, _) => {
                if self.input.is_empty() && self.required {
                    PromptState::Error(self.formatter.err_required())
                } else if self.input.value().parse::<isize>().is_err() {
                    PromptState::Error(self.formatter.err_invalid_format())
                } else if !self.is_within_range(self.value()) {
                    PromptState::Error(self.formatter.err_invalid_range(self.min, self.max))
                } else {
                    PromptState::Submit
                }
            }
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.input.move_left();
                PromptState::Active
            }
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                self.input.move_right();
                PromptState::Active
            }
            (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.input.move_home();
                PromptState::Active
            }
            (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                self.input.move_end();
                PromptState::Active
            }
            (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.input.delete_left_char();
                PromptState::Active
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.input.delete_left_word();
                PromptState::Active
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.input.delete_right_char();
                PromptState::Active
            }
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                self.input.delete_rest_line();
                PromptState::Active
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.input.delete_line();
                PromptState::Active
            }
            (KeyCode::Up, _) => {
                self.increment();
                PromptState::Active
            }
            (KeyCode::Down, _) => {
                self.decrement();
                PromptState::Active
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                self.insert(c);
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        self.value()
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let payload = RenderPayload::new(
            self.message.clone(),
            self.hint.clone(),
            self.placeholder.clone(),
        );

        match state {
            PromptState::Submit => Ok(payload.input(PromptInput::Raw(self.value().to_string()))),
            _ => Ok(payload.input(PromptInput::Cursor(self.input.clone()))),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.validator
            .as_ref()
            .map_or(Ok(()), |validator| validator.validate(&self.input.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_prompt;

    test_prompt!(
        test_hint,
        Number::new("test message").with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_placeholder,
        Number::new("test message").with_placeholder("placeholder message"),
        vec![]
    );

    test_prompt!(
        test_default,
        Number::new("test message").with_default(100),
        vec![]
    );

    test_prompt!(
        test_required_error,
        Number::new("test message").with_required(true),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_non_required_empty_submit,
        Number::new("test message").with_required(false),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_number_input,
        Number::new("test message").as_mut(),
        vec![
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Char('2'), KeyModifiers::NONE),
            (KeyCode::Char('3'), KeyModifiers::NONE),
            (KeyCode::Char('4'), KeyModifiers::NONE),
            (KeyCode::Char('5'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::CONTROL),
            (KeyCode::Char('h'), KeyModifiers::CONTROL),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Char('k'), KeyModifiers::CONTROL),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Char('u'), KeyModifiers::CONTROL),
            (KeyCode::Char('-'), KeyModifiers::NONE),
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Char('2'), KeyModifiers::NONE),
            (KeyCode::Char('3'), KeyModifiers::NONE),
            (KeyCode::Char('w'), KeyModifiers::CONTROL),
        ]
    );

    test_prompt!(
        test_invalid_format,
        Number::new("test message").as_mut(),
        vec![
            (KeyCode::Char('-'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_min_value,
        Number::new("test message").with_min(2),
        vec![
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Char('2'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_max_value,
        Number::new("test message").with_max(2),
        vec![
            (KeyCode::Char('3'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Char('2'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_increment_decrement,
        Number::new("test message").with_min(2).with_max(4),
        vec![
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Char('0'), KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char('u'), KeyModifiers::CONTROL),
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
        ]
    );
}
