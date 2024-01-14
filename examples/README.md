# Examples

This document is a list of implementation examples for Promptuity.

## Quick Start

This is a quick start implementation. It demonstrates the most basic usage of Promptuity.

```bash
$ cargo run --example quick_start
```

## Prompts

This is a sample of the basic usage of built-in prompts.

### Input

```bash
$ cargo run --example prompt_input
```

### Password

```bash
$ cargo run --example prompt_password
```

### Number

```bash
$ cargo run --example prompt_number
```

### Select

```bash
$ cargo run --example prompt_select
```

### MultiSelect

```bash
$ cargo run --example prompt_multi_select
```

### Confirm

```bash
$ cargo run --example prompt_confirm
```

## Error Handling

This is an implementation example of handling prompt interruptions.

```bash
$ cargo run --example error_handling
```

## Autocomplete

This is a reference implementation of `Autocomplete`. It is useful for deepening understanding in the following areas:

- Implementing custom prompts
- Handling user input (**Input**)
- Displaying multiple items (**Body**)

```bash
$ cargo run --example autocomplete
```

Of course, you can also copy and paste this reference implementation to adapt it for your project.

## Extend Prompt

This is an example of extending built-in prompts and customizing key bindings and rendering.

```bash
$ cargo run --example extend_prompt
```

## Custom Format

This is an example of customizing the format of built-in prompts.

```bash
$ cargo run --example custom_format
```

## Custom Theme

This is a reference implementation example of an original Theme.

```bash
$ cargo run --example custom_theme
```

## Packages

This is an implementation example that mimics version bumping in a Monorepo. It aids in understanding realistic use cases.

```bash
$ cargo run --example packages
```

## Survey Fancy

This is an implementation example of a survey using the `FancyTheme`.

```bash
$ cargo run --example survey_fancy
```

## Survey Minimal

This is an implementation example of a survey using the `MinimalTheme`.

```bash
$ cargo run --example survey_minimal
```
