use promptuity::prompts::Number;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let age = p.prompt(Number::new("How old are you?").with_min(0).with_max(120))?;
    p.finish()?;

    println!("\nresult: {:?}", age);

    Ok(())
}
