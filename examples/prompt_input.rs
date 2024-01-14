use promptuity::prompts::Input;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let name = p.prompt(
        Input::new("What is your accout name?")
            .with_placeholder("username")
            .with_hint("Only alphanumeric characters are allowed.")
            .with_validator(|value: &String| {
                if value.chars().all(|c| c.is_alphanumeric()) {
                    Ok(())
                } else {
                    Err("Invalid format".into())
                }
            }),
    )?;
    p.finish()?;

    println!("\nresult: {:?}", name);

    Ok(())
}
