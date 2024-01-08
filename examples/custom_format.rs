use promptuity::prompts::{
    Input, MultiSelect, MultiSelectOption, Number, Password, Select, SelectOption,
};
use promptuity::style::{Color, Styled, Symbol};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

const ERR_REQUIRED: &str = "Required!";
const S_SELECT_CURSOR: Symbol = Symbol("❯", " > ");
const S_SELECTED: Symbol = Symbol("✓", "[x]");
const S_UNSELECTED: Symbol = Symbol(" ", "[ ]");

struct InputFormatter;
impl promptuity::prompts::InputFormatter for InputFormatter {
    fn err_required(&self) -> String {
        ERR_REQUIRED.to_string()
    }
}

struct PasswordFormatter;
impl promptuity::prompts::PasswordFormatter for PasswordFormatter {
    fn err_required(&self) -> String {
        ERR_REQUIRED.to_string()
    }
}

struct NumberFormatter;
impl promptuity::prompts::NumberFormatter for NumberFormatter {
    fn err_required(&self) -> String {
        ERR_REQUIRED.to_string()
    }

    fn err_invalid_format(&self) -> String {
        "Invalid!".into()
    }

    fn err_invalid_range(&self, min: isize, max: isize) -> String {
        format!("Invalid Range! (expect min={}, max={})", min, max)
    }
}

struct SelectFormatter;
impl promptuity::prompts::SelectFormatter for SelectFormatter {
    fn option_icon(&self, active: bool) -> String {
        if active {
            Styled::new(S_SELECT_CURSOR).fg(Color::Cyan).to_string()
        } else {
            Styled::new(S_UNSELECTED).fg(Color::DarkGrey).to_string()
        }
    }

    fn option_label(&self, label: String, active: bool) -> String {
        if active {
            Styled::new(label)
                .fg(Color::Magenta)
                .underline()
                .to_string()
        } else {
            label
        }
    }

    fn option_hint(&self, hint: Option<String>, active: bool) -> String {
        if active {
            hint.as_ref().map_or_else(String::new, |hint| {
                format!(
                    " {}",
                    Styled::new(format!("({})", hint)).fg(Color::DarkGrey)
                )
            })
        } else {
            String::new()
        }
    }

    fn option(&self, icon: String, label: String, hint: String, _active: bool) -> String {
        format!("{} {}{}", icon, label, hint)
    }
}

struct MultiSelectFormatter;
impl promptuity::prompts::MultiSelectFormatter for MultiSelectFormatter {
    fn option_icon(&self, active: bool, selected: bool) -> String {
        if active {
            Styled::new(S_SELECT_CURSOR).fg(Color::Cyan).to_string()
        } else if selected {
            Styled::new(S_SELECTED).fg(Color::Cyan).to_string()
        } else {
            Styled::new(S_UNSELECTED).fg(Color::DarkGrey).to_string()
        }
    }

    fn option_label(&self, label: String, active: bool, _selected: bool) -> String {
        if active {
            Styled::new(label)
                .fg(Color::Magenta)
                .underline()
                .to_string()
        } else {
            label
        }
    }

    fn option_hint(&self, hint: Option<String>, active: bool, selected: bool) -> String {
        if active || selected {
            hint.as_ref().map_or_else(String::new, |hint| {
                format!(
                    " {}",
                    Styled::new(format!("({})", hint)).fg(Color::DarkGrey)
                )
            })
        } else {
            String::new()
        }
    }

    fn option(
        &self,
        icon: String,
        label: String,
        hint: String,
        _active: bool,
        _selected: bool,
    ) -> String {
        format!("{} {}{}", icon, label, hint)
    }

    fn submit(&self, labels: Vec<String>) -> String {
        labels.join(" / ")
    }

    fn err_required(&self) -> String {
        ERR_REQUIRED.to_string()
    }
}

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    p.with_intro("Custom Format").begin()?;

    let _ = p.prompt(Input::new("Input message").with_formatter(InputFormatter))?;

    let _ = p.prompt(
        Password::new("Password message")
            .with_formatter(PasswordFormatter)
            .with_mask('∙'),
    )?;

    let _ = p.prompt(
        Number::new("Number message")
            .with_formatter(NumberFormatter)
            .with_min(0)
            .with_max(10),
    )?;

    let _ = p.prompt(
        Select::new(
            "Select message",
            vec![
                SelectOption::new("Option1", "option1"),
                SelectOption::new("Option2", "option2").with_hint("Hint!"),
                SelectOption::new("Option3", "option3"),
            ],
        )
        .with_formatter(SelectFormatter),
    )?;

    let _ = p.prompt(
        MultiSelect::new(
            "Select message",
            vec![
                MultiSelectOption::new("Option1", "option1"),
                MultiSelectOption::new("Option2", "option2").with_hint("Hint!"),
                MultiSelectOption::new("Option3", "option3"),
            ],
        )
        .with_formatter(MultiSelectFormatter),
    )?;

    p.with_outro("Finish!").finish()?;

    Ok(())
}
