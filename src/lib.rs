use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn brainfuck(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as LitStr);

    let mut output = String::from(
        "use std::io::Write; let mut cells = [0u8; 3600]; let mut pointer: usize = 0;",
    );

    for token in input.value().chars() {
        output.push_str(
            match token {
                '>' => "pointer += 1;",
                '<' => "pointer -= 1;",
                '+' => "cells[pointer] += 1;",
                '-' => "cells[pointer] -= 1;",
                '.' => "print!(\"{}\", cells[pointer] as char); std::io::stdout().flush().unwrap();",
                ',' => "print!(\"input: \"); std::io::stdout().flush().unwrap(); let mut input = String::new(); std::io::stdin().read_line(&mut input).unwrap(); cells[pointer] = input.chars().nth(0).unwrap() as u8;",
                '[' => "while cells[pointer] != 0 {",
                ']' => "}",
                _ => ""
            }
        );
    }

    output.parse().unwrap()
}
