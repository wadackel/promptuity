use crate::event::*;
use crate::pagination::paginate;
use crate::prompts::{DefaultSelectFormatter, SelectFormatter};
use crate::{Error, Prompt, PromptBody, PromptInput, PromptState, RenderPayload};

/// A struct representing an option in the [`MultiSelect`] prompt.
#[derive(Debug, Clone)]
pub struct MultiSelectOption<T: Default + Clone> {
    /// The label of the option.
    pub label: String,
    /// The value of the option.
    pub value: T,
    /// The hint of the option.
    pub hint: Option<String>,
    /// The selected flag of the option.
    pub selected: bool,
}

impl<T: Default + Clone> MultiSelectOption<T> {
    /// Creates a new [`MultiSelectOption`] with the given label and value.
    pub fn new(label: impl std::fmt::Display, value: T) -> Self {
        Self {
            label: label.to_string(),
            value,
            hint: None,
            selected: false,
        }
    }

    /// Sets the hint message for the option.
    pub fn with_hint(mut self, hint: impl std::fmt::Display) -> Self {
        self.hint = Some(hint.to_string());
        self
    }
}

/// A trait for formatting the [`MultiSelect`] prompt.
///
/// `MultiSelectFormatter` does not default-implement some of the formatting processes in the trait.  
/// In the implementation of [`DefaultMultiSelectFormatter`], it internally uses [`DefaultSelectFormatter`] for formatting. Please refer to this when implementing custom formats.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{MultiSelect, MultiSelectOption, MultiSelectFormatter};
///
/// struct CustomFormatter;
///
/// impl MultiSelectFormatter for CustomFormatter {
///     fn option_icon(&self, active: bool, selected: bool) -> String {
///        if selected {
///            "[x]".into()
///        } else if active {
///            " > ".into()
///        } else {
///            "[ ]".into()
///        }
///     }
///
///     fn option_label(&self, label: String, _active: bool, _selected: bool) -> String {
///         label.clone()
///     }
///
///     fn option_hint(&self, hint: Option<String>, active: bool, _selected: bool) -> String {
///         if active {
///             hint.as_ref().map_or_else(String::new, |hint| {
///                 format!(" - {}", hint)
///             })
///         } else {
///             String::new()
///         }
///     }
///
///     fn option(&self, icon: String, label: String, hint: String, _active: bool, _selected: bool) -> String {
///         format!("{} {}{}", icon, label, hint)
///     }
/// }
///
/// let _ = MultiSelect::new("...", vec![MultiSelectOption::new("...", "...")]).with_formatter(CustomFormatter);
/// ```
pub trait MultiSelectFormatter {
    /// Icons displayed for each option.
    fn option_icon(&self, active: bool, selected: bool) -> String;
    /// Formats the label of the option.
    fn option_label(&self, label: String, active: bool, selected: bool) -> String;
    /// Formats the hint message of the option.
    fn option_hint(&self, hint: Option<String>, active: bool, selected: bool) -> String;
    /// Formats the option.
    fn option(
        &self,
        icon: String,
        label: String,
        hint: String,
        active: bool,
        selected: bool,
    ) -> String;

    /// Formats the submitted value.
    fn submit(&self, labels: Vec<String>) -> String {
        labels.join(", ")
    }

    /// Formats the error message for the required flag.
    fn err_required(&self) -> String {
        "This field is required.".into()
    }

    /// Formats errors for selections below the minimum number.
    fn err_min(&self, min: usize) -> String {
        format!("Please select at least {} options.", min)
    }

    /// Formats errors for selections above the maximum number.
    fn err_max(&self, max: usize) -> String {
        format!("Please select no more than {} options.", max)
    }
}

/// The default formatter for [`MultiSelect`].
pub struct DefaultMultiSelectFormatter {
    inner: DefaultSelectFormatter,
}

impl DefaultMultiSelectFormatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: DefaultSelectFormatter::new(),
        }
    }
}

impl MultiSelectFormatter for DefaultMultiSelectFormatter {
    fn option_icon(&self, _active: bool, selected: bool) -> String {
        self.inner.option_icon(selected)
    }

    fn option_label(&self, label: String, active: bool, _selected: bool) -> String {
        self.inner.option_label(label, active)
    }

    fn option_hint(&self, hint: Option<String>, active: bool, _selected: bool) -> String {
        self.inner.option_hint(hint, active)
    }

    fn option(
        &self,
        icon: String,
        label: String,
        hint: String,
        active: bool,
        _selected: bool,
    ) -> String {
        self.inner.option(icon, label, hint, active)
    }
}

/// A prompt for selecting multiple elements from a list of options.
///
/// # Options
///
/// - **Formatter**: Customizes the prompt display. See [`MultiSelectFormatter`].
/// - **Hint**: A message to assist with field input. Defaults to `None`.
/// - **Required**: A flag indicating whether to allow no input.
/// - **Minimum Selections**: The minimum number of selections required. Defaults to `0`.
/// - **Maximum Selections**: The maximum number of selections allowed. Defaults to `usize::MAX`.
/// - **Page Size**: The total number of options to displayed per page, used for pagination. Defaults to `8`.
/// - **Validator**: A function to validate the value at the time of submission.
///
/// # Examples
///
/// ```no_run
/// use promptuity::prompts::{MultiSelect, MultiSelectOption};
///
/// let _ = MultiSelect::new("What is your favorite color?", vec![
///     MultiSelectOption::new("Red", "#ff0000"),
///     MultiSelectOption::new("Green", "#00ff00").with_hint("recommended"),
///     MultiSelectOption::new("Blue", "#0000ff"),
/// ]).with_page_size(5);
/// ```
pub struct MultiSelect<T: Default + Clone> {
    formatter: Box<dyn MultiSelectFormatter>,
    message: String,
    hint: Option<String>,
    required: bool,
    min: usize,
    max: usize,
    page_size: usize,
    options: Vec<MultiSelectOption<T>>,
    index: usize,
}

impl<T: Default + Clone> MultiSelect<T> {
    /// Creates a new [`MultiSelect`] prompt with the given message and options.
    pub fn new(message: impl std::fmt::Display, options: Vec<MultiSelectOption<T>>) -> Self {
        Self {
            formatter: Box::new(DefaultMultiSelectFormatter::new()),
            message: message.to_string(),
            hint: None,
            required: true,
            min: 0,
            max: usize::MAX,
            page_size: 8,
            options,
            index: 0,
        }
    }

    /// Sets the formatter for the prompt.
    pub fn with_formatter(&mut self, formatter: impl MultiSelectFormatter + 'static) -> &mut Self {
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

    /// Sets the minimum number of selections for the prompt.
    pub fn with_min(&mut self, value: usize) -> &mut Self {
        self.min = value;
        self
    }

    /// Sets the maximum number of selections for the prompt.
    pub fn with_max(&mut self, value: usize) -> &mut Self {
        self.max = value;
        self
    }

    /// Sets the page size for the prompt.
    pub fn with_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }

    fn values(&mut self) -> Vec<T> {
        self.options
            .iter()
            .filter_map(|option| {
                if option.selected {
                    Some(option.value.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn map_options<F>(&mut self, f: F) -> Vec<MultiSelectOption<T>>
    where
        F: Fn(&MultiSelectOption<T>) -> MultiSelectOption<T>,
    {
        self.options.iter().map(f).collect::<Vec<_>>()
    }
}

impl<T: Default + Clone> AsMut<MultiSelect<T>> for MultiSelect<T> {
    fn as_mut(&mut self) -> &mut MultiSelect<T> {
        self
    }
}

impl<T: Default + Clone> Prompt for MultiSelect<T> {
    type Output = Vec<T>;

    fn setup(&mut self) -> Result<(), Error> {
        if self.options.is_empty() {
            return Err(Error::Config("options cannot be empty.".into()));
        }

        if self.min > self.max {
            return Err(Error::Config(format!(
                "min cannot be greater than max (min={}, max={})",
                self.min, self.max
            )));
        }

        Ok(())
    }

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> crate::PromptState {
        match (code, modifiers) {
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Enter, _) => {
                let values = self.values();
                if values.is_empty() && self.required {
                    PromptState::Error(self.formatter.err_required())
                } else if values.len() < self.min {
                    PromptState::Error(self.formatter.err_min(self.min))
                } else if values.len() > self.max {
                    PromptState::Error(self.formatter.err_max(self.max))
                } else {
                    PromptState::Submit
                }
            }
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
            (KeyCode::Char(' '), KeyModifiers::NONE) => {
                let mut option = self.options.get(self.index).unwrap().clone();
                option.selected = !option.selected;
                self.options[self.index] = option;
                PromptState::Active
            }
            (KeyCode::Char('a'), KeyModifiers::NONE) => {
                self.options = self.map_options(|option| {
                    let mut option = option.clone();
                    option.selected = true;
                    option
                });
                PromptState::Active
            }
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                self.options = self.map_options(|option| {
                    let mut option = option.clone();
                    option.selected = !option.selected;
                    option
                });
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        self.values()
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let payload = RenderPayload::new(self.message.clone(), self.hint.clone(), None);

        match state {
            PromptState::Submit => {
                let raw = self.formatter.submit(
                    self.options
                        .iter()
                        .filter_map(|option| {
                            if option.selected {
                                Some(option.label.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                );
                Ok(payload.input(PromptInput::Raw(raw)))
            }

            _ => {
                let page = paginate(self.page_size, &self.options, self.index);
                let options = page
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let active = i == page.cursor;
                        let selected = option.selected;
                        self.formatter.option(
                            self.formatter.option_icon(active, selected),
                            self.formatter
                                .option_label(option.label.clone(), active, selected),
                            self.formatter
                                .option_hint(option.hint.clone(), active, selected),
                            active,
                            selected,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(payload.body(PromptBody::Raw(options)))
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
                options.push(MultiSelectOption::new(
                    format!("Value{}", i),
                    format!("value{}", i),
                ));
            }
            options
        }};
    }

    test_prompt!(
        test_hint,
        MultiSelect::new("test message", options!(3)).with_hint("hint message"),
        vec![]
    );

    test_prompt!(
        test_10_items_with_5_page_size,
        MultiSelect::new("test message", options!(10)).with_page_size(5),
        vec![]
    );

    test_prompt!(
        test_option_hint,
        MultiSelect::new(
            "test message",
            vec![
                MultiSelectOption::new("Value1", "value1".to_string()).with_hint("hint1"),
                MultiSelectOption::new("Value2", "value2".to_string()),
                MultiSelectOption::new("Value3", "value3".to_string()).with_hint("hint3"),
            ]
        )
        .with_page_size(5),
        vec![]
    );

    test_prompt!(
        test_move,
        MultiSelect::new("test message", options!(10)).with_page_size(5),
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
        test_select_2_and_5,
        MultiSelect::new("test message", options!(10)).with_page_size(5),
        vec![
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_select_all_and_inverse,
        MultiSelect::new("test message", options!(5)).as_mut(),
        vec![
            (KeyCode::Char('a'), KeyModifiers::NONE),
            (KeyCode::Char('i'), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Char('i'), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_required_error,
        MultiSelect::new("test message", options!(5)).with_required(true),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_non_required_empty_submit,
        MultiSelect::new("test message", options!(5)).with_required(false),
        vec![(KeyCode::Enter, KeyModifiers::NONE)]
    );

    test_prompt!(
        test_min_error,
        MultiSelect::new("test message", options!(5)).with_min(2),
        vec![
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );

    test_prompt!(
        test_max_error,
        MultiSelect::new("test message", options!(5)).with_max(3),
        vec![
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Down, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
            (KeyCode::Char(' '), KeyModifiers::NONE),
            (KeyCode::Enter, KeyModifiers::NONE),
        ]
    );
}
