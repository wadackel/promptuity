use promptuity::event::*;
use promptuity::themes::{FancyTheme, MinimalTheme};
use promptuity::{
    InputCursor, Prompt, PromptBody, PromptInput, PromptState, Promptuity, RenderPayload, Theme,
};

mod fake_term;

enum DummyInputKind {
    None,
    Cursor,
    Raw,
}

enum DummyBodyKind {
    None,
    Raw,
}

struct DummyPrompt {
    input: DummyInputKind,
    body: DummyBodyKind,
    hint: bool,
    placeholder: bool,
}

impl Prompt for DummyPrompt {
    type Output = ();

    fn handle(&mut self, code: KeyCode, modifiers: KeyModifiers) -> promptuity::PromptState {
        match (code, modifiers) {
            (KeyCode::Enter, KeyModifiers::NONE) => PromptState::Submit,
            _ => PromptState::Active,
        }
    }

    fn submit(&mut self) -> Self::Output {}

    fn render(&mut self, _: &PromptState) -> Result<RenderPayload, String> {
        let hint = if self.hint {
            Some("Hint Message".into())
        } else {
            None
        };
        let placeholder = if self.placeholder {
            Some("Placeholder Message".into())
        } else {
            None
        };
        let input = match self.input {
            DummyInputKind::None => PromptInput::None,
            DummyInputKind::Cursor => PromptInput::Cursor(InputCursor::from("Cursor".into())),
            DummyInputKind::Raw => PromptInput::Raw("Raw".into()),
        };
        let body = match self.body {
            DummyBodyKind::None => PromptBody::None,
            DummyBodyKind::Raw => PromptBody::Raw("Raw1\nRaw2\nRaw3".into()),
        };
        Ok(RenderPayload::new("DummyPrompt".into(), hint, placeholder)
            .input(input)
            .body(body))
    }
}

fn run_theme(theme: &mut dyn Theme<Vec<u8>>) -> String {
    let mut term = fake_term::Term::new(&[
        (KeyCode::Enter, KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
    ]);
    {
        let mut p = Promptuity::new(&mut term, theme);
        // Input::None + Body::None
        let _ = p.prompt(&mut DummyPrompt {
            input: DummyInputKind::None,
            body: DummyBodyKind::None,
            hint: false,
            placeholder: false,
        });
        // Input::Raw + Body::None
        let _ = p.prompt(&mut DummyPrompt {
            input: DummyInputKind::Raw,
            body: DummyBodyKind::None,
            hint: false,
            placeholder: false,
        });
        // Input::Cursor + Body::None
        let _ = p.prompt(&mut DummyPrompt {
            input: DummyInputKind::Cursor,
            body: DummyBodyKind::None,
            hint: false,
            placeholder: false,
        });
        // Input::Raw + Body::Raw
        let _ = p.prompt(&mut DummyPrompt {
            input: DummyInputKind::Raw,
            body: DummyBodyKind::Raw,
            hint: false,
            placeholder: false,
        });
        // Input::Cursor + Body::Raw
        let _ = p.prompt(&mut DummyPrompt {
            input: DummyInputKind::Cursor,
            body: DummyBodyKind::Raw,
            hint: false,
            placeholder: false,
        });
    }
    term.output()
}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! test_theme {
    ($name: ident, $theme: expr) => {
        #[test]
        fn $name() {
            let output = crate::run_theme($theme);
            insta::with_settings!({ omit_expression => true }, {
                insta::assert_snapshot!(output);
            });
        }
    };
}

test_theme!(test_theme_minimal, &mut MinimalTheme::default());
test_theme!(test_theme_fancy, &mut FancyTheme::default());
