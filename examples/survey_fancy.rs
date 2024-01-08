use promptuity::prompts::{
    Confirm, Input, MultiSelect, MultiSelectOption, Number, Password, Select, SelectOption,
};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    p.with_intro("Promptuity Demo").begin()?;

    let name = p.prompt(Input::new("Please enter your username").with_placeholder("username"))?;

    let years = p.prompt(
        Number::new("How many years of software development experience do you have?")
            .with_min(0)
            .with_max(120),
    )?;

    let full =
        p.prompt(Confirm::new("Are you a full-time software developer?").with_default(true))?;

    let language = p.prompt(
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
        .as_mut(),
    )?;

    let interests = p.prompt(
        MultiSelect::new(
            "Select the fields you are interested in",
            vec![
                MultiSelectOption::new("Web Development", "web"),
                MultiSelectOption::new("Mobile App Development", "app"),
                MultiSelectOption::new("Data Science", "ds"),
                MultiSelectOption::new("Cloud Computing", "cloud"),
                MultiSelectOption::new("Database Management", "db"),
                MultiSelectOption::new("Devices and IoT", "iot"),
                MultiSelectOption::new("AI and Machine Learning", "ml"),
                MultiSelectOption::new("Game Development", "game"),
                MultiSelectOption::new("Security", "security"),
                MultiSelectOption::new("Frontend Development", "frontend"),
                MultiSelectOption::new("Backend Development", "backend"),
                MultiSelectOption::new("Open Source Projects", "oss"),
            ],
        )
        .with_hint("Multiple selections allowed"),
    )?;

    let password = p.prompt(
        Password::new("Set a password for your account").with_validator(|value: &String| {
            if value.len() < 8 {
                Err("Password must be at least 8 characters long".into())
            } else {
                Ok(())
            }
        }),
    )?;

    p.with_outro("Thank you for taking the time to complete the survey!")
        .finish()?;

    println!("name: {name:?}");
    println!("years: {years:?}");
    println!("full: {full:?}");
    println!("language: {language:?}");
    println!("interests: {interests:?}");
    println!("password: {password:?}");

    Ok(())
}
