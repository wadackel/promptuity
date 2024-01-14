use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use promptuity::event::*;
use promptuity::pagination::paginate;
use promptuity::prompts::{DefaultSelectFormatter, SelectFormatter, SelectOption};
use promptuity::style::*;
use promptuity::themes::MinimalTheme;
use promptuity::{
    Error, InputCursor, Prompt, PromptBody, PromptInput, PromptState, Promptuity, RenderPayload,
    Term,
};

struct Autocomplete {
    formatter: DefaultSelectFormatter,
    message: String,
    page_size: usize,
    options: Vec<SelectOption<String>>,
    filtered_options: Vec<usize>,
    index: usize,
    input: InputCursor,
    matcher: SkimMatcherV2,
}

impl Autocomplete {
    pub fn new(message: impl std::fmt::Display, options: Vec<SelectOption<String>>) -> Self {
        Self {
            formatter: DefaultSelectFormatter::new(),
            message: message.to_string(),
            page_size: 8,
            options,
            filtered_options: Vec::new(),
            index: 0,
            input: InputCursor::default(),
            matcher: SkimMatcherV2::default(),
        }
    }

    fn run_filter(&mut self) {
        let pattern = self.input.value();

        self.filtered_options = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, option)| self.matcher.fuzzy_match(&option.label, &pattern).map(|_| i))
            .collect::<Vec<_>>();

        self.index = std::cmp::min(self.filtered_options.len().saturating_sub(1), self.index);
    }

    fn current_option(&self) -> Option<&SelectOption<String>> {
        self.filtered_options
            .get(self.index)
            .and_then(|idx| self.options.get(*idx))
    }
}

impl AsMut<Autocomplete> for Autocomplete {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Prompt for Autocomplete {
    type Output = String;

    fn setup(&mut self) -> Result<(), Error> {
        if self.options.is_empty() {
            return Err(Error::Config("options cannot be empty.".into()));
        }

        self.filtered_options = (0..self.options.len()).collect();

        Ok(())
    }

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> promptuity::PromptState {
        match (code, modifiers) {
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => PromptState::Cancel,
            (KeyCode::Enter, _) => match self.current_option() {
                Some(_) => PromptState::Submit,
                _ => PromptState::Error("No matches found".into()),
            },
            (KeyCode::Up, _)
            | (KeyCode::Char('k'), KeyModifiers::CONTROL)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.index = self.index.saturating_sub(1);
                PromptState::Active
            }
            (KeyCode::Down, _)
            | (KeyCode::Char('j'), KeyModifiers::CONTROL)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.index = std::cmp::min(
                    self.filtered_options.len().saturating_sub(1),
                    self.index.saturating_add(1),
                );
                PromptState::Active
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
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.input.delete_left_word();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.input.delete_right_char();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.input.delete_line();
                self.run_filter();
                PromptState::Active
            }
            (KeyCode::Char(c), _) => {
                self.input.insert(c);
                self.run_filter();
                PromptState::Active
            }
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {
        self.current_option().unwrap().value.clone()
    }

    fn render(&mut self, state: &PromptState) -> Result<RenderPayload, String> {
        let payload = RenderPayload::new(self.message.clone(), None, None);

        match state {
            PromptState::Submit => {
                let option = self.current_option().unwrap();
                Ok(payload.input(PromptInput::Raw(option.label.clone())))
            }

            _ => {
                let page = paginate(self.page_size, &self.filtered_options, self.index);
                let options = page
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, idx)| {
                        let option = self.options.get(*idx).unwrap();
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

                let raw = if options.is_empty() {
                    Styled::new("<No matches found>")
                        .fg(Color::DarkGrey)
                        .to_string()
                } else {
                    options.to_string()
                };

                Ok(payload
                    .input(PromptInput::Cursor(self.input.clone()))
                    .body(PromptBody::Raw(raw)))
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    p.begin()?;

    let keywords = vec![
        "Python",
        "PHP",
        "Perl",
        "Programming",
        "JSON",
        "jQuery",
        "Java",
        "JavaScript",
        "C++",
        "C#",
        "CSS",
        "CSV",
        "HTML",
        "HTTP",
        "HTTPS",
        "Hashing",
        "SVG",
        "Swift",
        "Scala",
        "SQL",
    ];

    let value = p.prompt(
        Autocomplete::new(
            "Demo message:",
            keywords
                .into_iter()
                .map(|k| SelectOption::new(k.to_string(), k.to_string()))
                .collect::<Vec<_>>(),
        )
        .as_mut(),
    )?;

    p.finish()?;

    println!("\nYou selected: {}", value);

    Ok(())
}
