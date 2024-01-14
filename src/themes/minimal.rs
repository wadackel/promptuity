use strip_ansi_escapes::strip_str;

use crate::style::*;
use crate::{Error, InputCursor, PromptBody, PromptInput, PromptState, Terminal, Theme};

const S_STEP_ACTIVE: Symbol = Symbol("?", "?");
const S_STEP_ERROR: Symbol = Symbol("▲", "x");
const S_STEP_SUBMIT: Symbol = Symbol("✔", "?");
const S_ERROR_BAR: Symbol = Symbol("└", "—");

const S_INFO: Symbol = Symbol("•", "•");
const S_WARN: Symbol = Symbol("▲", "!");
const S_ERROR: Symbol = Symbol("✘", "x");
const S_SUCCESS: Symbol = Symbol("✓", "√");

/// A Theme that offers a compact and minimalistic display.
pub struct MinimalTheme {
    prev_lines: u16,
}

impl MinimalTheme {
    pub fn new() -> Self {
        Self { prev_lines: 0 }
    }

    fn fmt_message(&self, icon: impl std::fmt::Display, message: impl std::fmt::Display) -> String {
        format!("{} {}", icon, message)
    }

    fn fmt_hint(&self, hint: Option<String>) -> String {
        hint.map(|hint| format!("\n  {}", Styled::new(hint).fg(Color::DarkGrey),))
            .unwrap_or_default()
    }

    fn fmt_cursor(&self, cursor: InputCursor) -> String {
        let (left, cursor, right) = cursor.split();
        format!("{left}{}{right}", Styled::new(cursor).rev())
    }

    fn fmt_placeholder(&self, placeholder: String) -> String {
        let input = InputCursor::new(placeholder, 0);
        let (_, cursor, right) = input.split();
        format!(
            "{}{}",
            Styled::new(cursor).rev(),
            Styled::new(right).fg(Color::DarkGrey),
        )
    }

    fn fmt_input_layout(&self, input: impl std::fmt::Display) -> String {
        format!("  {}", input)
    }

    fn fmt_input_active(&self, input: PromptInput, placeholder: Option<String>) -> String {
        match input {
            PromptInput::Raw(s) => {
                let input = if s.is_empty() {
                    self.fmt_placeholder(placeholder.unwrap_or_default())
                } else {
                    s.to_string()
                };
                self.fmt_input_layout(input)
            }
            PromptInput::Cursor(c) => {
                let input = if c.value().is_empty() {
                    self.fmt_placeholder(placeholder.unwrap_or_default())
                } else {
                    self.fmt_cursor(c)
                };
                self.fmt_input_layout(input)
            }
            _ => String::new(),
        }
    }

    fn fmt_input_submit(&self, input: PromptInput) -> String {
        match input {
            PromptInput::Raw(s) => self.fmt_input_layout(Styled::new(s).fg(Color::Cyan)),
            PromptInput::Cursor(c) => self.fmt_input_layout(Styled::new(c.value()).fg(Color::Cyan)),
            _ => String::new(),
        }
    }

    fn fmt_body_active(&self, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(s) => {
                format!("\n{}", s)
            }
            _ => String::new(),
        }
    }

    fn fmt_body_submit(&self, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(s) => {
                format!("\n{}", Styled::new(s).fg(Color::DarkGrey))
            }
            _ => String::new(),
        }
    }

    fn fmt_error(&self, message: String) -> String {
        format!(
            "\n{}",
            Styled::new(format!("{} {}", S_ERROR_BAR, message)).fg(Color::Yellow),
        )
    }
}

impl Default for MinimalTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: std::io::Write> Theme<W> for MinimalTheme {
    fn log(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        term.writeln(&message)?;
        term.flush()?;
        Ok(())
    }

    fn info(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new(S_INFO).fg(Color::Cyan), message),
        )
    }

    fn warn(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new(S_WARN).fg(Color::Yellow), message),
        )
    }

    fn error(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new(S_ERROR).fg(Color::Red), message),
        )
    }

    fn success(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new(S_SUCCESS).fg(Color::Green), message),
        )
    }

    fn step(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        term.writeln(&self.fmt_message(
            Styled::new(S_STEP_SUBMIT).fg(Color::Green),
            Styled::new(message).bold(),
        ))?;
        term.flush()?;
        Ok(())
    }

    fn begin(&mut self, term: &mut dyn Terminal<W>, _: Option<String>) -> Result<(), Error> {
        term.cursor_hide()?;
        term.flush()?;
        Ok(())
    }

    fn render(
        &mut self,
        term: &mut dyn Terminal<W>,
        payload: crate::RenderSnapshot,
    ) -> Result<(), Error> {
        if self.prev_lines > 0 {
            term.move_previous_line(self.prev_lines)?;
            term.clear_cursor_down()?;
        }

        let mut output = String::new();

        match payload.state {
            PromptState::Active => {
                output.push_str(&self.fmt_message(
                    Styled::new(S_STEP_ACTIVE).fg(Color::Cyan),
                    Styled::new(payload.message).bold(),
                ));

                output.push_str(&self.fmt_input_active(payload.input, payload.placeholder));
                output.push_str(&self.fmt_body_active(payload.body));
                output.push_str(&self.fmt_hint(payload.hint));

                self.prev_lines = wrap_text(&strip_str(&output), term.size()?.width)
                    .lines()
                    .count() as u16;
            }

            PromptState::Error(msg) | PromptState::Fatal(msg) => {
                let color = match payload.state {
                    PromptState::Error(_) => Color::Yellow,
                    PromptState::Fatal(_) => Color::Red,
                    _ => unreachable!(),
                };

                output.push_str(&self.fmt_message(
                    Styled::new(S_STEP_ERROR).fg(color),
                    Styled::new(payload.message).bold(),
                ));

                output.push_str(&self.fmt_input_active(payload.input, payload.placeholder));
                output.push_str(&self.fmt_body_active(payload.body));
                output.push_str(&self.fmt_error(msg.clone()));
                output.push_str(&self.fmt_hint(payload.hint));

                self.prev_lines = wrap_text(&strip_str(&output), term.size()?.width)
                    .lines()
                    .count() as u16;
            }

            PromptState::Submit => {
                output.push_str(&self.fmt_message(
                    Styled::new(S_STEP_SUBMIT).fg(Color::Green),
                    Styled::new(payload.message).bold(),
                ));

                output.push_str(&self.fmt_input_submit(payload.input));
                output.push_str(&self.fmt_body_submit(payload.body));

                self.prev_lines = 0;
            }

            PromptState::Cancel => {
                output.push_str(&self.fmt_message(
                    Styled::new(S_WARN).fg(Color::Yellow),
                    Styled::new(payload.message).bold(),
                ));

                self.prev_lines = 0;
            }
        }

        term.writeln(&output)?;
        term.flush()?;

        Ok(())
    }

    fn finish(
        &mut self,
        term: &mut dyn Terminal<W>,
        _: &crate::PromptState,
        _: Option<String>,
    ) -> Result<(), Error> {
        term.cursor_show()?;
        term.flush()?;
        Ok(())
    }
}
