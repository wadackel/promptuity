use crate::event::*;
use crate::{InputCursor, Prompt, PromptInput, PromptState, RenderPayload, Validator};

/// A trait for formatting the [`Password`] prompt.
///
/// All methods have default implementations, allowing you to override only the specific formatting process you need.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Password, PasswordFormatter};
///
/// struct CustomFormatter;
///
/// impl PasswordFormatter for CustomFormatter {
///     fn err_required(&self) -> String {
///         "REQUIRED!!".into()
///     }
/// }
///
/// let _ = Password::new("...").with_formatter(CustomFormatter);
/// ```
pub trait PasswordFormatter {
    /// Formats the error message when the input is empty and required.
    fn err_required(&self) -> String {
        "This field is required.".into()
    }
}

/// The default formatter for [`Password`].
pub struct DefaultPasswordFormatter;

impl DefaultPasswordFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordFormatter for DefaultPasswordFormatter {}

/// A text input prompt where the input is not displayed.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`PasswordFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Required**: A flag indicating whether to allow no input.
/// - **Mask**: A string used to mask the input value. Defaults to `*`.
/// - **Validator**: A function to validate the value at the time of submission.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::Password;
///
/// let _ = Password::new("What is your password?").with_required(false);
/// ```
pub struct Password {
    formatter: Box<dyn PasswordFormatter>,
    message: String,
    hint: Option<String>,
    required: bool,
    mask: char,
    validator: Option<Box<dyn Validator<String>>>,
    input: InputCursor,
}

impl Password {
    /// Creates a new [`Password`] prompt.
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            formatter: Box::new(DefaultPasswordFormatter::new()),
            message: message.to_string(),
            hint: None,
            required: true,
            mask: '*',
            validator: None,
            input: InputCursor::new(String::new(), 0),
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl PasswordFormatter + 'static) -> &mut Self {
        self.formatter = Box::new(formatter);
        self
    }

    /// Sets the hint message for the prompt.
    pub fn with_hint(&mut self, hint: impl std::fmt::Display) -> &mut Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Sets the required flag for the prompt.
    pub fn with_required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }

    /// Sets the mask char for the prompt.
    pub fn with_mask(&mut self, mask: char) -> &mut Self {
        self.mask = mask;
        self
    }

    /// Sets the validator for the prompt.
    pub fn with_validator(&mut self, f: impl Validator<String> + 'static) -> &mut Self {
        self.validator = Some(Box::new(move |value: &String| -> Result<(), String> {
            f.validate(value).map_err(|err| err.to_string())
        }));
        self
    }
}

impl AsMut<Password> for Password {
    fn as_mut(&mut self) -> &mut Password {
        self
    }
}

impl Prompt for Password {
    type Output = String;

    fn handle(
        &mut self,
        code: crossterm::event::KeyCode,
        modifiers: crossterm::event::KeyModifiers,
    ) -> crate::PromptState {
        match (code, modifiers) {
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Enter, _) => {
                if self.input.is_empty() && self.required {
                    PromptState::Error(self.formatter.err_required())
                } else {
                    PromptState::Submit
                }
            }
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
            (KeyCode::Char(c), _) => {
                self.input.insert(c);
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        self.input.value()
    }

    fn render(&mut self, _: &crate::PromptState) -> Result<crate::RenderPayload, String> {
        let input = InputCursor::new(
            self.input.value().chars().map(|_| self.mask).collect(),
            self.input.cursor(),
        );

        Ok(
            RenderPayload::new(self.message.clone(), self.hint.clone(), None)
                .input(PromptInput::Cursor(input)),
        )
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
        Password::new("test message").with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_required_error,
        Password::new("test message").with_required(true),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_non_required_empty_submit,
        Password::new("test message").with_required(false),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_input,
        Password::new("test message").as_mut(),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('1'), KeyModifiers::NONE),
            (KeyCode::Char('0'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_editing,
        Password::new("test message").as_mut(),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Char('d'), KeyModifiers::NONE),
            (KeyCode::Char('e'), KeyModifiers::NONE),
            (KeyCode::Char('f'), KeyModifiers::NONE),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Char('h'), KeyModifiers::CONTROL),
            (KeyCode::Char('h'), KeyModifiers::NONE),
            (KeyCode::Home, KeyModifiers::NONE),
            (KeyCode::Delete, KeyModifiers::NONE),
            (KeyCode::Right, KeyModifiers::NONE),
            (KeyCode::Char('d'), KeyModifiers::CONTROL),
            (KeyCode::Char('k'), KeyModifiers::CONTROL),
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('r'), KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('z'), KeyModifiers::NONE),
            (KeyCode::Char('w'), KeyModifiers::CONTROL),
            (KeyCode::Char('w'), KeyModifiers::CONTROL),
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Char('u'), KeyModifiers::CONTROL),
        ]
    );

    test_prompt!(
        test_custom_mask,
        Password::new("test message").with_mask('∙'),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );
}
