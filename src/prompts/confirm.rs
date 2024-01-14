use crate::event::*;
use crate::style::{Color, Styled, Symbol};
use crate::{Prompt, PromptInput, PromptState, RenderPayload};

const S_ACTIVE: Symbol = Symbol("●", ">");
const S_INACTIVE: Symbol = Symbol("○", " ");

/// A trait for formatting the [`Confirm`] prompt.
///
/// All methods have default implementations, allowing you to override only the specific formatting process you need.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Confirm, ConfirmFormatter};
///
/// struct CustomFormatter;
///
/// impl ConfirmFormatter for CustomFormatter {
///     fn layout(&self, yes: String, no: String) -> String {
///         format!("{} or {}", yes, no)
///     }
/// }
///
/// let _ = Confirm::new("...").with_formatter(CustomFormatter);
/// ```
pub trait ConfirmFormatter {
    /// Formats the "Yes" option.
    fn yes(&self, active: bool) -> String {
        let icon = if active {
            Styled::new(S_ACTIVE).fg(Color::Green).to_string()
        } else {
            Styled::new(S_INACTIVE).fg(Color::DarkGrey).to_string()
        };
        format!("{} Yes", icon)
    }

    /// Formats the "No" option.
    fn no(&self, active: bool) -> String {
        let icon = if active {
            Styled::new(S_ACTIVE).fg(Color::Green).to_string()
        } else {
            Styled::new(S_INACTIVE).fg(Color::DarkGrey).to_string()
        };
        format!("{} No", icon)
    }

    /// Formats the layout of the active prompt.
    fn layout(&self, yes: String, no: String) -> String {
        format!("{}  /  {}", yes, no)
    }

    /// Formats the submitted value.
    fn submit(&self, value: bool) -> String {
        if value {
            "Yes".into()
        } else {
            "No".into()
        }
    }
}

/// The default formatter for [`Confirm`].
pub struct DefaultConfirmFormatter;

impl DefaultConfirmFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl ConfirmFormatter for DefaultConfirmFormatter {}

/// A prompt for inputting a Yes/No choice.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`ConfirmFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Default Value**: The default value of `bool`. Defaults to `false`.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::Confirm;
///
/// let _ = Confirm::new("Are you sure?").with_default(true);
/// ```
pub struct Confirm {
    formatter: Box<dyn ConfirmFormatter>,
    message: String,
    hint: Option<String>,
    value: bool,
}

impl Confirm {
    /// Creates a new [`Confirm`] prompt.
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            formatter: Box::new(DefaultConfirmFormatter),
            message: message.to_string(),
            hint: None,
            value: false,
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl ConfirmFormatter + 'static) -> &mut Self {
        self.formatter = Box::new(formatter);
        self
    }

    /// Sets the hint message for the prompt.
    pub fn with_hint(&mut self, hint: impl std::fmt::Display) -> &mut Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Sets the default value for the prompt.
    pub fn with_default(&mut self, value: bool) -> &mut Self {
        self.value = value;
        self
    }
}

impl AsMut<Confirm> for Confirm {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Prompt for Confirm {
    type Output = bool;

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PromptState {
        match (code, modifiers) {
            (KeyCode::Enter, _) => PromptState::Submit,
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Char('y'), KeyModifiers::NONE) | (KeyCode::Char('Y'), KeyModifiers::NONE) => {
                self.value = true;
                PromptState::Submit
            }
            (KeyCode::Char('n'), KeyModifiers::NONE) | (KeyCode::Char('N'), KeyModifiers::NONE) => {
                self.value = false;
                PromptState::Submit
            }
            (KeyCode::Left, _)
            | (KeyCode::Char('h'), _)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.value = true;
                PromptState::Active
            }
            (KeyCode::Right, _)
            | (KeyCode::Char('l'), _)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.value = false;
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let payload = RenderPayload::new(self.message.clone(), self.hint.clone(), None);

        match state {
            PromptState::Submit => {
                Ok(payload.input(PromptInput::Raw(self.formatter.submit(self.value))))
            }

            PromptState::Cancel => Ok(payload),

            _ => Ok(payload.input(PromptInput::Raw(self.formatter.layout(
                self.formatter.yes(self.value),
                self.formatter.no(!self.value),
            )))),
        }
    }

    fn submit(&mut self) -> Self::Output {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_prompt;

    test_prompt!(
        test_hint,
        Confirm::new("test message").with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_default_enter,
        Confirm::new("test message").as_mut(),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_default_yes,
        Confirm::new("test message").with_default(true),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_move,
        Confirm::new("test message").as_mut(),
        vec![
            (KeyCode::Left, KeyModifiers::NONE),
            (KeyCode::Right, KeyModifiers::NONE),
            (KeyCode::Char('h'), KeyModifiers::NONE),
            (KeyCode::Char('l'), KeyModifiers::NONE),
            (KeyCode::Char('h'), KeyModifiers::CONTROL),
            (KeyCode::Char('l'), KeyModifiers::CONTROL),
            (KeyCode::Char('p'), KeyModifiers::CONTROL),
            (KeyCode::Char('n'), KeyModifiers::CONTROL),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_direct_lower_yes,
        Confirm::new("test message").with_default(false),
        vec![(KeyCode::Char('y'), KeyModifiers::NONE)]
    );

    test_prompt!(
        test_direct_upper_yes,
        Confirm::new("test message").with_default(false),
        vec![(KeyCode::Char('Y'), KeyModifiers::NONE)]
    );

    test_prompt!(
        test_direct_lower_no,
        Confirm::new("test message").with_default(true),
        vec![(KeyCode::Char('n'), KeyModifiers::NONE)]
    );

    test_prompt!(
        test_direct_upper_no,
        Confirm::new("test message").with_default(true),
        vec![(KeyCode::Char('N'), KeyModifiers::NONE)]
    );
}
