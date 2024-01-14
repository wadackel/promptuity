use promptuity::prompts::Password;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let secret = p.prompt(
        Password::new("Set a password for your account")
            .with_hint("Please enter more than 6 alphanumeric characters.")
            .with_validator(|value: &String| {
                if value.len() < 6 {
                    Err("Password must be at least 6 characters long".into())
                } else {
                    Ok(())
                }
            }),
    )?;
    p.finish()?;

    println!("\nresult: {:?}", secret);

    Ok(())
}
