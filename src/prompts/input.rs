use crate::event::*;
use crate::{InputCursor, Prompt, PromptInput, PromptState, RenderPayload, Validator};

/// A trait for formatting the [`Input`] prompt.
///
/// All methods have default implementations, allowing you to override only the specific formatting process you need.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Input, InputFormatter};
///
/// struct CustomFormatter;
///
/// impl InputFormatter for CustomFormatter {
///     fn err_required(&self) -> String {
///         "REQUIRED!!".into()
///     }
/// }
///
/// let _ = Input::new("...").with_formatter(CustomFormatter);
/// ```
pub trait InputFormatter {
    /// Formats the error message when the input is empty and required.
    fn err_required(&self) -> String {
        "This field is required.".into()
    }
}

/// The default formatter for [`Input`].
pub struct DefaultInputFormatter;

impl DefaultInputFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl InputFormatter for DefaultInputFormatter {}

/// A prompt for general text input.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`InputFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Placeholder**: An auxiliary message displayed when no input is given.
/// - **Required**: A flag indicating whether to allow no input.
/// - **Default Value**: The default value of `String`.
/// - **Validator**: A function to validate the value at the time of submission.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::Input;
///
/// let _ = Input::new("What is your accout name?").with_hint("e.g. wadackel");
/// ```
pub struct Input {
    formatter: Box<dyn InputFormatter>,
    message: String,
    hint: Option<String>,
    placeholder: Option<String>,
    required: bool,
    validator: Option<Box<dyn Validator<String>>>,
    input: InputCursor,
}

impl Input {
    /// Creates a new [`Input`] prompt.
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            formatter: Box::new(DefaultInputFormatter::new()),
            message: message.to_string(),
            hint: None,
            placeholder: None,
            required: true,
            validator: None,
            input: InputCursor::default(),
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl InputFormatter + 'static) -> &mut Self {
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

    /// Sets the default value for the prompt.
    pub fn with_default(&mut self, value: impl std::fmt::Display) -> &mut Self {
        self.input = InputCursor::from(value.to_string());
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

impl AsMut<Input> for Input {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<W: std::io::Write> Prompt<W> for Input {
    type Output = String;

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PromptState {
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

    fn render(&mut self, _: &PromptState) -> Result<RenderPayload, String> {
        Ok(RenderPayload::new(
            self.message.clone(),
            self.hint.clone(),
            self.placeholder.clone(),
        )
        .input(PromptInput::Cursor(self.input.clone())))
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
        Input,
        Input::new("test message").with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_placeholder,
        Input,
        Input::new("test message").with_placeholder("placeholder message"),
        vec![]
    );

    test_prompt!(
        test_default,
        Input,
        Input::new("test message").as_mut(),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_required_error,
        Input,
        Input::new("test message").as_mut(),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_non_required_empty_submit,
        Input,
        Input::new("test message").with_required(false),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_move,
        Input,
        Input::new("test message").with_default("abcdef"),
        vec![
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::CONTROL),
            (KeyCode::Home, KeyModifiers::NONE),
            (KeyCode::Right, KeyModifiers::NONE),
            (KeyCode::Char('f'), KeyModifiers::CONTROL),
            (KeyCode::End, KeyModifiers::NONE),
            (KeyCode::Char('a'), KeyModifiers::CONTROL),
        ]
    );

    test_prompt!(
        test_editing,
        Input,
        Input::new("test message").with_default("abcdef"),
        vec![
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
        test_validation,
        Input,
        Input::new("test message").with_validator(|v: &String| {
            if v.as_str() == "abc" {
                Err("Error Message".into())
            } else {
                Ok(())
            }
        }),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('b'), KeyModifiers::NONE),
            (KeyCode::Char('c'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Backspace, KeyModifiers::NONE),
            (KeyCode::Char('z'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );
}
