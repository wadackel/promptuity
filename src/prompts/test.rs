use crate::event::*;
use crate::{Prompt, PromptBody, PromptInput, PromptState, RenderPayload};

pub fn render_payload_display(payload: RenderPayload, full: bool) -> String {
    let mut segments = Vec::new();

    if full {
        segments.push(format!("message: {}", payload.message));
        segments.push(format!("hint: {}", payload.hint.unwrap_or("none".into())));
        segments.push(format!(
            "placeholder: {}",
            payload.placeholder.unwrap_or("none".into())
        ));
    }

    segments.push(match payload.input {
        PromptInput::Raw(raw) => format!("input(raw): {}", raw),
        PromptInput::Cursor(cursor) => {
            let (left, cursor, right) = cursor.split();
            format!("input(cursor): {}|{}{}", left, cursor, right)
        }
        PromptInput::None => "input(none):".into(),
    });

    segments.push(match payload.body {
        PromptBody::Raw(raw) => format!("body(raw):\n{}", raw),
        PromptBody::None => "body(none):".into(),
    });

    segments.join("\n")
}

pub fn render_display<T, W>(state: PromptState, prompt: &mut T, full: bool) -> String
where
    T: Prompt<W>,
    W: std::io::Write,
{
    format!(
        "state: {}\n{}",
        state,
        render_payload_display(prompt.render(&state).unwrap(), full)
    )
}

pub fn handle_actions<T, W>(prompt: &mut T, actions: Vec<(KeyCode, KeyModifiers)>) -> String
where
    T: Prompt<W> + Prompt<Vec<u8>>,
    W: std::io::Write,
{
    let mut output = Vec::new();

    output.push(render_display::<T, Vec<_>>(
        PromptState::Active,
        prompt,
        true,
    ));

    for (code, modifiers) in actions {
        let state = Prompt::<Vec<_>>::handle(prompt, code, modifiers);
        let state = match state {
            PromptState::Submit => {
                if let Err(msg) = Prompt::<Vec<_>>::validate(prompt) {
                    PromptState::Error(msg)
                } else {
                    PromptState::Submit
                }
            }
            state => state,
        };
        output.push(render_display::<T, Vec<_>>(state, prompt, false));
    }

    output.join("\n---\n")
}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! test_prompt {
    ($name: ident, $ty: ty, $prompt: expr, $actions: expr) => {
        #[test]
        fn $name() {
            let output = crate::prompts::test::handle_actions::<$ty, Vec<_>>($prompt, $actions);
            insta::with_settings!({ omit_expression => true }, {
                insta::assert_snapshot!(output);
            });
        }
    };
}
