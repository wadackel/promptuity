use promptuity::prompts::{Confirm, Input, Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    p.with_intro("Survey").begin()?;

    let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;

    let _ = p.prompt(Confirm::new("Are you a full-time software developer?").with_default(true))?;

    let _ = p.prompt(
        Select::new(
            "Select your primary programming language",
            vec![
                SelectOption::new("Rust", "rust"),
                SelectOption::new("Go", "go"),
                SelectOption::new("C++", "cpp"),
                SelectOption::new("C", "c"),
                SelectOption::new("TypeScript", "typescript"),
                SelectOption::new("JavaScript", "javascript"),
                SelectOption::new("Deno", "deno"),
                SelectOption::new("Python", "python"),
                SelectOption::new("Java", "java"),
                SelectOption::new("Dart", "dart"),
                SelectOption::new("Other", "other"),
            ],
        )
        .with_hint("Submit with Space or Enter."),
    )?;

    p.with_outro(format!("Thank you for your response, {}!", name))
        .finish()?;

    Ok(())
}
