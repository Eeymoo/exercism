use std::{char, fmt::format};

pub fn reverse(input: &str) -> String {
    let mut reverse_string:String = String::new();
    for char in input.chars() {
        reverse_string = format!("{}{}", char, reverse_string) 
    }

    reverse_string
}
