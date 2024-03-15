/*!
# Input Functionality

These functions enable menu/dialogue creation & interaction through
arrow-key selection and type-checking input boxes.
*/

use dialoguer;
use std::{fmt::Display, io::Write, str::FromStr};

/**
Select an item using arrow keys from an array.

# Examples

```ignore
use albion_terminal_rpg::prelude::select;

let choice1: usize = select(&["1. Login", "2. Register", "3. Exit"], None);

let strings: Vec<String> = vec![String::from("Test"), String::from("No")];
let choice2: usize = select(&strings, Some("Select one of these."));
```
*/
pub fn select<T: Display>(options: &[T], optional_prompt: Option<&str>) -> usize {
    if let Some(prompt_text) = optional_prompt {
        println!("{prompt_text}");
    }

    dialoguer::Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(0)
}

/**
Ask user for input with a prompt string.

# Examples

```ignore
use albion_terminal_rpg::prelude::prompt;

let answer: String = prompt("test"); // "test > {input here}"
```
*/
pub fn prompt(text: &str) -> String {
    print!("{text} > ");

    if let Err(message) = std::io::stdout().flush() {
        panic!("{}", message);
    }

    let mut input: String = String::new();

    if std::io::stdin().read_line(&mut input).is_err() {
        return prompt(text);
    }

    input.trim().to_string()
}

/**
Prompts user for input and attempts to
cast the string to a passed generic type.

# Examples

```ignore
use albion_terminal_rpg::prelude::{Result, input_generic};

let number: Result<usize> = input_generic("Number"); // "Number > {input here}"
let string: Result<String> = input_generic("Any String"); // "Any String > {input here}"
```
*/
pub fn input_generic<T>(text: &str) -> Result<T, <T as std::str::FromStr>::Err>
where
    T: FromStr,
{
    let input_string = prompt(text);
    let trimmed = input_string.trim();

    trimmed.parse::<T>()
}
