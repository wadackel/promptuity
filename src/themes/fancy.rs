use crate::style::*;
use crate::{
    Error, InputCursor, PromptBody, PromptInput, PromptState, RenderSnapshot, Terminal, Theme,
};

const S_STEP_ACTIVE: Symbol = Symbol("◆", "*");
const S_STEP_ERROR: Symbol = Symbol("▲", "x");
const S_STEP_SUBMIT: Symbol = Symbol("◇", "o");

const S_BAR_START: Symbol = Symbol("┌", "T");
const S_BAR: Symbol = Symbol("│", "|");
const S_BAR_END: Symbol = Symbol("└", "—");

const S_INFO: Symbol = Symbol("•", "•");
const S_WARN: Symbol = Symbol("▲", "!");
const S_ERROR: Symbol = Symbol("✘", "x");
const S_SUCCESS: Symbol = Symbol("✔", "√");

/// A Theme that displays with a rich UI.
pub struct FancyTheme {
    prev_lines: u16,
}

impl FancyTheme {
    pub fn new() -> Self {
        Self { prev_lines: 0 }
    }

    fn fmt_line_with(
        &self,
        symbol: impl std::fmt::Display,
        line: impl std::fmt::Display,
    ) -> String {
        format!("{}  {}\n", symbol, line)
    }

    fn fmt_line(&self, color: Color, line: impl std::fmt::Display) -> String {
        self.fmt_line_with(Styled::new(S_BAR).fg(color), line)
    }

    fn fmt_message(
        &self,
        icon: impl std::fmt::Display,
        message: impl std::fmt::Display,
        hint: Option<String>,
    ) -> String {
        let hint = hint
            .map(|hint| {
                format!(
                    " {}",
                    Styled::new(format!("({})", hint)).fg(Color::DarkGrey)
                )
            })
            .unwrap_or_default();

        self.fmt_line_with(icon, format!("{}{}", message, hint))
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

    fn fmt_input_active(
        &self,
        color: Color,
        input: PromptInput,
        placeholder: Option<String>,
    ) -> String {
        match input {
            PromptInput::Raw(s) => {
                let input = if s.is_empty() {
                    Styled::new(placeholder.unwrap_or_default())
                        .fg(Color::DarkGrey)
                        .to_string()
                } else {
                    s.to_string()
                };
                self.fmt_line(color, input)
            }
            PromptInput::Cursor(c) => {
                let input = if c.value().is_empty() {
                    self.fmt_placeholder(placeholder.unwrap_or_default())
                } else {
                    self.fmt_cursor(c)
                };
                self.fmt_line(color, input)
            }
            _ => String::new(),
        }
    }

    fn fmt_input_submit(&self, color: Color, input: PromptInput) -> String {
        match input {
            PromptInput::Raw(s) => self.fmt_line(color, Styled::new(s).fg(Color::DarkGrey)),
            PromptInput::Cursor(c) => {
                self.fmt_line(color, Styled::new(c.value()).fg(Color::DarkGrey))
            }
            _ => String::new(),
        }
    }

    fn fmt_body_active(&self, color: Color, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(s) => s
                .lines()
                .map(|line| self.fmt_line(color, line))
                .collect::<Vec<_>>()
                .join("")
                .to_string(),
            _ => String::new(),
        }
    }

    fn fmt_body_submit(&self, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(s) => s
                .lines()
                .map(|line| self.fmt_line(Color::DarkGrey, Styled::new(line).fg(Color::DarkGrey)))
                .collect::<Vec<_>>()
                .join("")
                .to_string(),
            _ => String::new(),
        }
    }

    fn fmt_end(&self, color: Color, edge: bool) -> String {
        let symbol = if edge { S_BAR_END } else { S_BAR };
        Styled::new(symbol).fg(color).to_string()
    }

    fn fmt_error(&self, message: String) -> String {
        Styled::new(message).fg(Color::Yellow).to_string()
    }
}

impl Default for FancyTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: std::io::Write> Theme<W> for FancyTheme {
    fn log(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        let prefix = Styled::new(S_BAR).fg(Color::DarkGrey).to_string();
        if message.is_empty() {
            term.writeln(&prefix)?;
        } else {
            for line in message.lines() {
                term.writeln(&format!("{}  {}", prefix, line))?;
            }
        }
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
        term.writeln(
            self.fmt_message(
                Styled::new(S_STEP_SUBMIT).fg(Color::Green),
                Styled::new(message).bold(),
                None,
            )
            .trim(),
        )?;
        term.flush()?;
        Ok(())
    }

    fn begin(&mut self, term: &mut dyn Terminal<W>, intro: Option<String>) -> Result<(), Error> {
        term.cursor_hide()?;
        term.writeln(&format!(
            "{}  {}",
            Styled::new(S_BAR_START).fg(Color::DarkGrey),
            Styled::new(format!(" {} ", intro.unwrap_or("INTRO".into())))
                .rev()
                .fg(Color::Cyan)
        ))?;
        term.writeln(&Styled::new(S_BAR).fg(Color::DarkGrey).to_string())?;
        term.flush()?;
        Ok(())
    }

    fn render(&mut self, term: &mut dyn Terminal<W>, payload: RenderSnapshot) -> Result<(), Error> {
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
                    payload.hint,
                ));

                output.push_str(&self.fmt_input_active(
                    Color::Cyan,
                    payload.input,
                    payload.placeholder,
                ));

                output.push_str(&self.fmt_body_active(Color::Cyan, payload.body));
                output.push_str(&self.fmt_end(Color::Cyan, true));

                self.prev_lines = output.lines().count() as u16;
            }

            PromptState::Error(msg) | PromptState::Fatal(msg) => {
                let color = match payload.state {
                    PromptState::Error(_) => Color::Yellow,
                    PromptState::Fatal(_) => Color::Red,
                    _ => unreachable!(),
                };

                let mut out = String::new();

                out.push_str(&self.fmt_message(
                    Styled::new(S_STEP_ERROR).fg(color),
                    Styled::new(payload.message).bold(),
                    payload.hint,
                ));

                out.push_str(&self.fmt_input_active(color, payload.input, payload.placeholder));

                out.push_str(&self.fmt_body_active(color, payload.body));

                if out.lines().count() < 2 {
                    out.push_str(&self.fmt_line(color, ""));
                }

                output.push_str(&out);

                output.push_str(&format!(
                    "{}  {}",
                    self.fmt_end(color, true),
                    self.fmt_error(msg.clone()),
                ));

                self.prev_lines = output.lines().count() as u16;
            }

            PromptState::Submit => {
                output.push_str(&self.fmt_message(
                    Styled::new(S_STEP_SUBMIT).fg(Color::Green),
                    Styled::new(payload.message).bold(),
                    None,
                ));

                output.push_str(&self.fmt_input_submit(Color::DarkGrey, payload.input));
                output.push_str(&self.fmt_body_submit(payload.body));
                output.push_str(&self.fmt_end(Color::DarkGrey, false));

                self.prev_lines = 0;
            }

            PromptState::Cancel => {
                output.push_str(&self.fmt_message(
                    Styled::new(S_STEP_SUBMIT).fg(Color::Yellow),
                    Styled::new(payload.message).bold(),
                    None,
                ));

                output.push_str(&self.fmt_input_submit(Color::Yellow, payload.input));
                output.push_str(&self.fmt_end(Color::Yellow, false));

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
        state: &PromptState,
        outro: Option<String>,
    ) -> Result<(), Error> {
        term.cursor_show()?;

        match state {
            PromptState::Cancel => {
                term.writeln(&self.fmt_line_with(
                    Styled::new(S_BAR_END).fg(Color::Yellow),
                    Styled::new(format!("Operation canceled {}", S_WARN)).fg(Color::Yellow),
                ))?;
            }
            _ => {
                if let Some(outro) = outro {
                    term.writeln(&self.fmt_line_with(
                        Styled::new(S_BAR_END).fg(Color::DarkGrey),
                        Styled::new(outro).bold(),
                    ))?;
                }
            }
        }

        term.flush()?;

        Ok(())
    }
}
