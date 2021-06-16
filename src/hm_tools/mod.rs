pub mod art;

const WORD_COUNT: usize = 10;

pub const WORDS: [&'static str; WORD_COUNT] = [
    "house",
    "turtle",
    "building",
    "pancake",
    "catnip",
    "keyboard",
    "rust programming language",
    "made by chinky",
    "copyright included",
    "e a chacho?"
];

pub mod tools{
    use crate::hm_tools::{WORDS, WORD_COUNT};
    use rand::prelude::*;

    pub fn random_word() -> String {
        let mut rng = rand::thread_rng();
        String::from(WORDS[rng.gen_range(0..WORD_COUNT)])
    }

    pub fn encrypt_word(word: &String) -> String {
        let mut result = String::new();
        
        for l in word.chars(){
            if l as i32 >= 97 && l as i32 <= 122 {
                result.push('_');
                result.push(' ');
            }
            else if l as i32 == 32 {
                result.push(' ');
                result.push(' ');
            }
            else {
                result.push(l);
            }
        };

        result
        
    }

    pub fn remove_duplicates(list: &mut Vec<char>) {
        list.sort();
        list.dedup();
        list.retain(|&c| c != ' ');
    }

    pub fn decrypt_encrypted(guess: char, encr: &mut String, word: &String) {
        for (n, c) in word.chars().enumerate() {
            if c == guess {
                encr.replace_range(n+n..n+n+1, &guess.to_string());
            }
        }
    }

}