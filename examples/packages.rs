use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use promptuity::prompts::{Input, MultiSelect, MultiSelectOption};
use promptuity::style::{Color, Styled};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

fn make_progress_bar(message: &'static str, writer: impl std::io::Write) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.wrap_write(writer);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan}  {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.set_message(message);
    pb
}

fn prompt() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.with_intro("Bump Package Version").begin()?;

    let packages = p.prompt(
        MultiSelect::new(
            "Which packages would you like to include?",
            vec![
                MultiSelectOption::new("@scope/a", "@scope/a"),
                MultiSelectOption::new("@scope/b", "@scope/b"),
                MultiSelectOption::new("@scope/c", "@scope/c"),
                MultiSelectOption::new("@scope/d", "@scope/d"),
                MultiSelectOption::new("@scope/e", "@scope/e"),
                MultiSelectOption::new("@scope/f", "@scope/f"),
            ],
        )
        .with_required(false),
    )?;

    if packages.is_empty() {
        p.with_outro(Styled::new("Version bump has been skipped.").fg(Color::Yellow))
            .finish()?;
        return Ok(());
    }

    let major = p.prompt(
        MultiSelect::new(
            format!(
                "Which packages should have a {} version bump?",
                Styled::new("major").fg(Color::Yellow),
            ),
            packages
                .clone()
                .into_iter()
                .map(|p| MultiSelectOption::new(p, p))
                .collect(),
        )
        .with_required(false),
    )?;

    let rest = packages
        .clone()
        .into_iter()
        .filter(|p| !major.contains(p))
        .collect::<Vec<_>>();

    let minor = if rest.is_empty() {
        vec![]
    } else {
        p.prompt(
            MultiSelect::new(
                format!(
                    "Which packages should have a {} version bump?",
                    Styled::new("minor").fg(Color::Yellow),
                ),
                rest.into_iter()
                    .map(|p| MultiSelectOption::new(p, p))
                    .collect(),
            )
            .with_required(false),
        )?
    };

    let rest = packages
        .clone()
        .into_iter()
        .filter(|p| !major.contains(p) && !minor.contains(p))
        .collect::<Vec<_>>();

    let patch = if rest.is_empty() {
        vec![]
    } else {
        p.prompt(
            MultiSelect::new(
                format!(
                    "Which packages should have a {} version bump?",
                    Styled::new("patch").fg(Color::Yellow),
                ),
                rest.into_iter()
                    .map(|p| MultiSelectOption::new(p, p))
                    .collect(),
            )
            .with_required(false),
        )?
    };

    if major.is_empty() && minor.is_empty() && patch.is_empty() {
        p.with_outro(Styled::new("Version bump has been skipped.").fg(Color::Yellow))
            .finish()?;
        return Ok(());
    }

    let message =
        p.prompt(Input::new("Please enter a summary for this change").with_placeholder("Summary"))?;

    let pb = make_progress_bar("Bumping versinos...", p.term().writer());
    thread::sleep(Duration::from_secs(4));
    pb.finish_and_clear();

    p.step("Bumped versions")?;
    p.log(
        Styled::new(format!(
            "update {} packages",
            major.len() + minor.len() + patch.len()
        ))
        .fg(Color::DarkGrey),
    )?;
    p.log("")?; // layout adjustment

    p.with_outro(Styled::new("Version bumped!").fg(Color::Green))
        .finish()?;

    // debug log
    println!("major: {major:?}");
    println!("minor: {minor:?}");
    println!("patch: {patch:?}");
    println!("message: {message:?}");

    Ok(())
}

fn main() {
    match prompt() {
        Ok(_) => {}
        Err(Error::Cancel) => {
            println!("Cancelled");
        }
        Err(err) => {
            eprintln!("main::Error: {}", err);
        }
    }
}
