use promptuity::event::*;
use promptuity::prompts::Confirm;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Prompt, Promptuity, Term};

struct ExtendedConfirm {
    original: Confirm,
}

impl ExtendedConfirm {
    fn new(message: impl std::fmt::Display) -> Self {
        Self {
            original: Confirm::new(message),
        }
    }
}

impl AsMut<ExtendedConfirm> for ExtendedConfirm {
    fn as_mut(&mut self) -> &mut ExtendedConfirm {
        self
    }
}

impl<W: std::io::Write> Prompt<W> for ExtendedConfirm {
    type Output = bool;

    fn handle(
        &mut self,
        code: crossterm::event::KeyCode,
        modifiers: crossterm::event::KeyModifiers,
    ) -> promptuity::PromptState {
        match (code, modifiers) {
            (KeyCode::Left, KeyModifiers::NONE) => {
                // forward to `y` key
                Prompt::<W>::handle(&mut self.original, KeyCode::Char('y'), KeyModifiers::NONE)
            }
            (KeyCode::Right, KeyModifiers::NONE) => {
                // forward to `n` key
                Prompt::<W>::handle(&mut self.original, KeyCode::Char('n'), KeyModifiers::NONE)
            }
            _ => {
                // forward to original handler
                Prompt::<W>::handle(&mut self.original, code, modifiers)
            }
        }
    }

    fn submit(&mut self) -> Self::Output {
        Prompt::<W>::submit(&mut self.original)
    }

    fn render(
        &mut self,
        state: &promptuity::PromptState,
    ) -> Result<promptuity::RenderPayload, String> {
        Prompt::<W>::render(&mut self.original, state)
    }
}

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::new();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.begin()?;

    let result = p.prompt(ExtendedConfirm::new("input message").as_mut())?;

    p.finish()?;

    println!("result: {}", result);

    Ok(())
}
