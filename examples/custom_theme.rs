use promptuity::prompts::{Input, Select, SelectOption};
use promptuity::style::*;
use promptuity::{Error, PromptBody, PromptInput, PromptState, Promptuity, Term, Terminal, Theme};

const S_STEP: Symbol = Symbol("ℹ️", "i");

struct SimpleTheme {
    prev_lines: u16,
}

impl SimpleTheme {
    fn new() -> Self {
        Self { prev_lines: 0 }
    }

    fn fmt_message(&self, message: String) -> String {
        format!("{} {}:", S_STEP, message)
    }

    fn fmt_placeholder(&self, placeholder: Option<String>) -> String {
        Styled::new(placeholder.unwrap_or_default())
            .fg(Color::DarkGrey)
            .to_string()
    }

    fn fmt_hint(&self, hint: Option<String>) -> String {
        Styled::new(hint.unwrap_or_default())
            .fg(Color::DarkGrey)
            .to_string()
    }

    fn fmt_input_active(&self, input: PromptInput, placeholder: Option<String>) -> String {
        match input {
            PromptInput::Raw(raw) => {
                format!(
                    " {}",
                    if raw.is_empty() {
                        self.fmt_placeholder(placeholder)
                    } else {
                        raw
                    }
                )
            }
            PromptInput::Cursor(cursor) => {
                format!(
                    " {}",
                    if cursor.is_empty() && placeholder.is_some() {
                        self.fmt_placeholder(placeholder)
                    } else {
                        let (left, cursor, right) = cursor.split();
                        format!("{left}{}{right}", Styled::new(cursor).rev())
                    }
                )
            }
            _ => String::new(),
        }
    }

    fn fmt_input_submit(&self, input: PromptInput) -> String {
        match input {
            PromptInput::Raw(raw) => format!(" {}", Styled::new(raw).fg(Color::Green)),
            PromptInput::Cursor(cursor) => {
                format!(" {}", Styled::new(cursor.value()).fg(Color::Green))
            }
            _ => String::new(),
        }
    }

    fn fmt_body_active(&self, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(raw) => {
                format!("\n{}", raw)
            }
            _ => String::new(),
        }
    }

    fn fmt_body_submit(&self, body: PromptBody) -> String {
        match body {
            PromptBody::Raw(raw) => {
                format!("\n{}", Styled::new(raw).fg(Color::Green))
            }
            _ => String::new(),
        }
    }
}

impl<W: std::io::Write> Theme<W> for SimpleTheme {
    fn log(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        term.writeln(&message)?;
        term.flush()?;
        Ok(())
    }

    fn info(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new("[info]").fg(Color::Cyan), message),
        )
    }

    fn warn(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new("[warn]").fg(Color::Yellow), message),
        )
    }

    fn error(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new("[error]").fg(Color::Red), message),
        )
    }

    fn success(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new("[success]").fg(Color::Green), message),
        )
    }

    fn step(&mut self, term: &mut dyn Terminal<W>, message: String) -> Result<(), Error> {
        self.log(
            term,
            format!("{} {}", Styled::new(S_STEP).fg(Color::Cyan), message),
        )
    }

    fn begin(&mut self, term: &mut dyn Terminal<W>, intro: Option<String>) -> Result<(), Error> {
        term.cursor_hide()?;
        if let Some(intro) = intro {
            term.writeln(&format!("INTRO: '{}'", intro))?;
        }
        term.flush()?;
        Ok(())
    }

    fn render(
        &mut self,
        term: &mut dyn Terminal<W>,
        payload: promptuity::RenderSnapshot,
    ) -> Result<(), Error> {
        if self.prev_lines > 0 {
            term.move_previous_line(self.prev_lines)?;
            term.clear_cursor_down()?;
        }

        let mut output = String::new();

        match payload.state {
            PromptState::Active => {
                output.push_str(&self.fmt_message(payload.message));
                output.push_str(&self.fmt_input_active(payload.input, payload.placeholder));
                output.push_str(&self.fmt_body_active(payload.body));
                output.push_str(&self.fmt_hint(payload.hint));
                self.prev_lines = output.lines().count() as u16;
            }
            PromptState::Error(msg) | PromptState::Fatal(msg) => {
                output.push_str(&self.fmt_message(payload.message));
                output.push_str(&self.fmt_input_active(payload.input, payload.placeholder));
                output.push_str(&self.fmt_body_active(payload.body));
                output.push_str(&self.fmt_hint(payload.hint));
                output.push_str(&format!("\n{}", Styled::new(msg).fg(Color::Red)));
                self.prev_lines = output.lines().count() as u16;
            }
            PromptState::Submit => {
                output.push_str(&self.fmt_message(payload.message));
                output.push_str(&self.fmt_input_submit(payload.input));
                output.push_str(&self.fmt_body_submit(payload.body));
                output.push_str(&self.fmt_hint(payload.hint));
                self.prev_lines = 0;
            }
            PromptState::Cancel => {
                output.push_str(&self.fmt_message(payload.message));
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
        _: &promptuity::PromptState,
        outro: Option<String>,
    ) -> Result<(), Error> {
        term.cursor_show()?;
        if let Some(outro) = outro {
            term.writeln(&format!("OUTRO: '{}'", outro))?;
        }
        term.flush()?;
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = SimpleTheme::new();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.with_intro("Custom Theme DEMO").begin()?;

    let _ = p.prompt(Input::new("input message").as_mut())?;

    let _ = p.prompt(
        Select::new(
            "select message",
            vec![
                SelectOption::new("value1", "value1"),
                SelectOption::new("value2", "value2"),
                SelectOption::new("value3", "value3"),
            ],
        )
        .as_mut(),
    )?;

    p.with_outro("Outro Message").finish()?;

    Ok(())
}
