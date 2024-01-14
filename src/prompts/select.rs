use crate::event::*;
use crate::pagination::paginate;
use crate::style::*;
use crate::{Error, Prompt, PromptBody, PromptInput, PromptState, RenderPayload};

const S_UNSELECTED: Symbol = Symbol("◯", "[ ]");
const S_SELECTED: Symbol = Symbol("◉", "[x]");

/// A struct representing an option in the [`Select`] prompt.
#[derive(Debug, Clone)]
pub struct SelectOption<T: Default + Clone> {
    /// The label of the option.
    pub label: String,
    /// The value of the option.
    pub value: T,
    /// The hint message of the option. Defaults to `None`.
    pub hint: Option<String>,
}

impl<T: Default + Clone> SelectOption<T> {
    /// Creates a new [`SelectOption`] with the given label and value.
    pub fn new(label: impl std::fmt::Display, value: T) -> Self {
        Self {
            label: label.to_string(),
            value,
            hint: None,
        }
    }

    /// Sets the hint message for the option.
    pub fn with_hint(mut self, hint: impl std::fmt::Display) -> Self {
        self.hint = Some(hint.to_string());
        self
    }
}

/// A trait for customizing the display of [`Select`].
///
/// All methods have default implementations, allowing you to override only the specific formatting process you need.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Select, SelectOption, SelectFormatter};
///
/// struct CustomFormatter;
///
/// impl SelectFormatter for CustomFormatter {
///     fn option_icon(&self, active: bool) -> String {
///        if active {
///            ">".into()
///        } else {
///            " ".into()
///        }
///     }
/// }
///
/// let _ = Select::new("...", vec![SelectOption::new("...", "...")]).with_formatter(CustomFormatter);
/// ```
pub trait SelectFormatter {
    /// Icons displayed for each option.
    fn option_icon(&self, active: bool) -> String {
        if active {
            Styled::new(S_SELECTED).fg(Color::Green).to_string()
        } else {
            Styled::new(S_UNSELECTED).fg(Color::DarkGrey).to_string()
        }
    }

    /// Formats the label of the option.
    fn option_label(&self, label: String, active: bool) -> String {
        if active {
            Styled::new(label).underline().to_string()
        } else {
            Styled::new(label).fg(Color::DarkGrey).to_string()
        }
    }

    /// Formats the hint message of the option.
    fn option_hint(&self, hint: Option<String>, active: bool) -> String {
        let _ = active;
        hint.as_ref().map_or_else(String::new, |hint| {
            format!(
                " {}",
                Styled::new(format!("({})", hint)).fg(Color::DarkGrey)
            )
        })
    }

    /// Formats the option.
    fn option(&self, icon: String, label: String, hint: String, active: bool) -> String {
        let _ = active;
        format!("{} {}{}", icon, label, hint)
    }
}

/// The default formatter for [`Select`].
pub struct DefaultSelectFormatter;

impl DefaultSelectFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl SelectFormatter for DefaultSelectFormatter {}

/// A prompt for selecting a single element from a list of options.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`SelectFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Page Size**: The total number of options to displayed per page, used for pagination. Defaults to `8`.
///
/// # Notes
///
/// Passing an empty `options` will result in an error. Please ensure to provide `options` with at least one item.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{Select, SelectOption};
///
/// let _ = Select::new("What is your favorite color?", vec![
///     SelectOption::new("Red", "#ff0000"),
///     SelectOption::new("Green", "#00ff00").with_hint("recommended"),
///     SelectOption::new("Blue", "#0000ff"),
/// ]).with_page_size(5);
/// ```
pub struct Select<T: Default + Clone> {
    formatter: Box<dyn SelectFormatter>,
    message: String,
    hint: Option<String>,
    page_size: usize,
    options: Vec<SelectOption<T>>,
    index: usize,
}

impl<T: Default + Clone> Select<T> {
    /// Creates a new [`Select`] prompt with the given message and options.
    pub fn new(message: impl std::fmt::Display, options: Vec<SelectOption<T>>) -> Self {
        Self {
            formatter: Box::new(DefaultSelectFormatter::new()),
            message: message.to_string(),
            hint: None,
            page_size: 8,
            options,
            index: 0,
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl SelectFormatter + 'static) -> &mut Self {
        self.formatter = Box::new(formatter);
        self
    }

    /// Sets the hint message for the prompt.
    pub fn with_hint(&mut self, hint: impl std::fmt::Display) -> &mut Self {
        self.hint = Some(hint.to_string());
        self
    }

    /// Sets the page size for the prompt.
    pub fn with_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
}

impl<T: Default + Clone> AsMut<Select<T>> for Select<T> {
    fn as_mut(&mut self) -> &mut Select<T> {
        self
    }
}

impl<T: Default + Clone> Prompt for Select<T> {
    type Output = T;

    fn setup(&mut self) -> Result<(), Error> {
        if self.options.is_empty() {
            return Err(Error::Config("options cannot be empty.".into()));
        }
        Ok(())
    }

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> crate::PromptState {
        match (code, modifiers) {
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Enter, _) | (KeyCode::Char(' '), _) => PromptState::Submit,
            (KeyCode::Up, _)
            | (KeyCode::Char('k'), _)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.index = self.index.saturating_sub(1);
                PromptState::Active
            }
            (KeyCode::Down, _)
            | (KeyCode::Char('j'), _)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.index = std::cmp::min(
                    self.options.len().saturating_sub(1),
                    self.index.saturating_add(1),
                );
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        let option = self.options.get(self.index).unwrap();
        option.value.clone()
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let payload = RenderPayload::new(self.message.clone(), self.hint.clone(), None);

        match state {
            PromptState::Submit => {
                let option = self.options.get(self.index).unwrap();
                Ok(payload.input(PromptInput::Raw(option.label.clone())))
            }

            _ => {
                let page = paginate(self.page_size, &self.options, self.index);
                let options = page
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let active = i == page.cursor;
                        self.formatter.option(
                            self.formatter.option_icon(active),
                            self.formatter.option_label(option.label.clone(), active),
                            self.formatter.option_hint(option.hint.clone(), active),
                            active,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let raw = options.to_string();

                Ok(payload.body(PromptBody::Raw(raw)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_prompt;

    macro_rules! options {
        ($count: expr) => {{
            let mut options = Vec::new();
            for i in 1..=$count {
                options.push(SelectOption::new(
                    format!("Value{}", i),
                    format!("value{}", i),
                ));
            }
            options
        }};
    }

    test_prompt!(
        test_hint,
        Select::new("test message", options!(3)).with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_10_items,
        Select::new("test message", options!(10)).as_mut(),
        vec![]
    );

    test_prompt!(
        test_10_items_with_5_page_size,
        Select::new("test message", options!(10)).with_page_size(5),
        vec![]
    );

    test_prompt!(
        test_option_hint,
        Select::new(
            "test message",
            vec![
                SelectOption::new("Value1", "value1".to_string()).with_hint("hint1"),
                SelectOption::new("Value2", "value2".to_string()),
                SelectOption::new("Value3", "value3".to_string()).with_hint("hint3"),
            ]
        )
        .with_page_size(5),
        vec![]
    );

    test_prompt!(
        test_move,
        Select::new("test message", options!(10)).with_page_size(5),
        vec![
            (KeyCode::Char('j'), KeyModifiers::NONE),
            (KeyCode::Char('n'), KeyModifiers::CONTROL),
            (KeyCode::Char('k'), KeyModifiers::NONE),
            (KeyCode::Char('p'), KeyModifiers::CONTROL),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE), // 10
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE),
            (KeyCode::Up, KeyModifiers::NONE), // 1
            (KeyCode::Up, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_select_5,
        Select::new("test message", options!(10)).as_mut(),
        vec![
            (KeyCode::Char('j'), KeyModifiers::NONE),
            (KeyCode::Char('j'), KeyModifiers::NONE),
            (KeyCode::Char('j'), KeyModifiers::NONE),
            (KeyCode::Char('j'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );
}
