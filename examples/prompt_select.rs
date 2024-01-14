use promptuity::prompts::{Select, SelectOption};
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let color = p.prompt(
        Select::new(
            "What is your favorite color?",
            vec![
                SelectOption::new("Red", "#ff0000"),
                SelectOption::new("Green", "#00ff00").with_hint("recommended"),
                SelectOption::new("Blue", "#0000ff"),
            ],
        )
        .as_mut(),
    )?;
    p.finish()?;

    println!("\nresult: {:?}", color);

    Ok(())
}
