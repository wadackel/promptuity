use promptuity::prompts::{MultiSelect, MultiSelectOption};
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;
    let color = p.prompt(
        MultiSelect::new(
            "What are your favorite colors?",
            vec![
                MultiSelectOption::new("Red", "#ff0000"),
                MultiSelectOption::new("Green", "#00ff00").with_hint("recommended"),
                MultiSelectOption::new("Blue", "#0000ff"),
            ],
        )
        .as_mut(),
    )?;
    p.finish()?;

    println!("\nresult: {:?}", color);

    Ok(())
}
