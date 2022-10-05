use crate::magic::Magic;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Write};
use std::ops::Add;
use std::vec::Vec;

struct MagicSpell {
    magic_words: Vec<String>,
}

impl MagicSpell {

    fn new() -> Self {
        MagicSpell {
            magic_words: Vec::new(),
        }
    }


    fn new_with_magic_number<T>(number: T) -> Result<Self, String>
    where
        T: Magic + Display,
    {
        match number.is_magic() {
            true => {
                let mut data = Self::new();
                data.magic_words.push(format!("{}", number));

                Ok(data)
            }
            false => Err("That's not a magic word!".to_string()),
        }
    }

    fn get_magic_words(&self) -> &Vec<String> {
        &self.magic_words
    }

    fn add_magic_word(&mut self, word: String) {
        self.magic_words.push(word)
    }

    fn set_magic_words(&mut self, data: &MagicSpell) {
        self.magic_words = data.magic_words.clone();
    }

    fn take_magic_words(self) -> Vec<String> {
        self.magic_words
    }
}

fn print_magic_words(data: &MagicSpell) {
    for (index, word) in data.magic_words.iter().enumerate() {
        println!("Magic word {} is {}", index, word);
    }
}

fn print_spell(data: MagicSpell) {
    let spell_length = data.magic_words.iter().map(|w| w.len()).sum();

    let mut spell = String::with_capacity(spell_length);

    for word in data.magic_words {
        spell.write_str(&word);
    }

    println!("{}", spell.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_and_print() -> Result<(), String> {
        let data = MagicSpell::new();
        print_magic_words(&data);

        let more_data = MagicSpell::new_with_magic_number(42)?;
        print_magic_words(&more_data);

        //more_data.set_magic_string(&data);

        Ok(())
    }

    #[test]
    fn can_print_spell() {
        let data = MagicSpell::new();

        let magic_words = data.get_magic_words();

        print_magic_words(&data);

        print_spell(data);

        //println!("There are {} magic word(s)", magic_words.len());
    }

    #[test]
    fn can_take_and_print() {
        let magic_word_1 = "abra".to_string();
        let magic_word_2 = "ca".to_string();
        let magic_word_3 = "dabra".to_string();

        let mut data = MagicSpell::new();

        data.add_magic_word(magic_word_1);
        data.add_magic_word(magic_word_2);
        data.add_magic_word(magic_word_3);

        //let magic_words = data.take_magic_words();

        print_magic_words(&data);
    }
}
