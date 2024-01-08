use promptuity::prompts::Input;
use promptuity::themes::MinimalTheme;
use promptuity::{Error, Promptuity, Term};

fn ask() -> Result<String, Error> {
    let mut term = Term::default();
    let mut theme = MinimalTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.begin()?;
    let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;
    p.finish()?;

    Ok(name)
}

fn main() {
    match ask() {
        Ok(name) => println!("Hello, {}!", name),
        Err(Error::Cancel) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
