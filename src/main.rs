mod color;
mod magic_addition;
mod magic;
mod magic_spell;


use std::{env, io};
use std::str::FromStr;
use colored::Colorize;
use figlet_rs::{FIGfont, FIGure};
use crate::color::FavouriteColor;

fn main() {
    println!("What's your favourite color?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    let color = match FavouriteColor::from_str(&input.trim_end_matches("\r\n")) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to parse color - {}", e);
            return;
        }
    };

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Hello Rust").unwrap();

    let figure_text = figure.to_string();

    println!("{}", match color {
        FavouriteColor::Red => figure_text.red(),
        FavouriteColor::Yellow => figure_text.yellow(),
        FavouriteColor::Cyan => figure_text.cyan(),
        FavouriteColor::Custom { red, green, blue } => figure_text.truecolor(red, green, blue),
    });
}


