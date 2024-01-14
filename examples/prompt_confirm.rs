use promptuity::prompts::Confirm;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let like = p.prompt(
        Confirm::new("Do you like dogs?")
            .with_hint("This is just a sample prompt :)")
            .with_default(true),
    )?;
    p.finish()?;

    println!("\nresult: {:?}", like);

    Ok(())
}
