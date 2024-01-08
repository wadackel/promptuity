use pretty_assertions::assert_eq;
use promptuity::event::*;
use promptuity::prompts::{
    Confirm, Input, MultiSelect, MultiSelectOption, Number, Select, SelectOption,
};
use promptuity::themes::MinimalTheme;
use promptuity::Promptuity;

mod fake_term;

#[derive(Debug, PartialEq)]
struct TestResult {
    input: String,
    password: String,
    number: isize,
    select: usize,
    multi_select: Vec<usize>,
    confirm: bool,
}

#[test]
fn test_prompts() {
    let mut term = fake_term::Term::new(&[
        // Input
        (KeyCode::Char('a'), KeyModifiers::NONE),
        (KeyCode::Char('b'), KeyModifiers::NONE),
        (KeyCode::Char('c'), KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        // Password
        (KeyCode::Char('f'), KeyModifiers::NONE),
        (KeyCode::Char('o'), KeyModifiers::NONE),
        (KeyCode::Char('o'), KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        // Number
        (KeyCode::Char('1'), KeyModifiers::NONE),
        (KeyCode::Char('0'), KeyModifiers::NONE),
        (KeyCode::Char('0'), KeyModifiers::NONE),
        (KeyCode::Home, KeyModifiers::NONE),
        (KeyCode::Char('-'), KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        // Select
        (KeyCode::Char('j'), KeyModifiers::NONE),
        (KeyCode::Down, KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        // MultiSelect
        (KeyCode::Char(' '), KeyModifiers::NONE),
        (KeyCode::Down, KeyModifiers::NONE),
        (KeyCode::Char('j'), KeyModifiers::NONE),
        (KeyCode::Char(' '), KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
        // Confirm
        (KeyCode::Right, KeyModifiers::NONE),
        (KeyCode::Left, KeyModifiers::NONE),
        (KeyCode::Char('n'), KeyModifiers::NONE),
        (KeyCode::Enter, KeyModifiers::NONE),
    ]);

    let mut theme = MinimalTheme::default();

    let result = {
        let mut p = Promptuity::new(&mut term, &mut theme);
        let input = p.prompt(Input::new("Input Message").as_mut()).unwrap();
        let password = p.prompt(Input::new("Password Message").as_mut()).unwrap();
        let number = p.prompt(Number::new("Number Message").as_mut()).unwrap();
        let select = p
            .prompt(
                Select::new(
                    "Select Message",
                    vec![
                        SelectOption::new("Option1", 1),
                        SelectOption::new("Option2", 2),
                        SelectOption::new("Option3", 3),
                    ],
                )
                .as_mut(),
            )
            .unwrap();
        let multi_select = p
            .prompt(
                MultiSelect::new(
                    "MultiSelect Message",
                    vec![
                        MultiSelectOption::new("Option1", 1),
                        MultiSelectOption::new("Option2", 2),
                        MultiSelectOption::new("Option3", 3),
                    ],
                )
                .as_mut(),
            )
            .unwrap();
        let confirm = p.prompt(Confirm::new("Confirm Message").as_mut()).unwrap();
        TestResult {
            input,
            password,
            number,
            select,
            multi_select,
            confirm,
        }
    };

    let output = term.output();

    assert_eq!(
        TestResult {
            input: "abc".into(),
            password: "foo".into(),
            number: -100,
            select: 3,
            multi_select: vec![1, 3],
            confirm: false,
        },
        result
    );

    insta::with_settings!({ omit_expression => true }, {
        insta::assert_snapshot!(output);
    });
}
