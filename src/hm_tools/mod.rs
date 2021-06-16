pub mod art;

pub const WORDS: [&'static str; 9] = [
    "house",
    "turtle",
    "building",
    "pancake",
    "catnip",
    "keyboard",
    "rust programming language",
    "made by Leon",
    "copyright included",
];

pub mod tools{
    use crate::hm_tools::WORDS;
    use rand::prelude::*;

    pub fn random_word() -> String {
        let mut rng = rand::thread_rng();
        String::from(WORDS[rng.gen_range(0..9)])
    }

    pub fn encrypt_word(word: &String) -> String {
        let size = word.chars().count();
        let mut result = String::new();

        for l in word.chars(){
            if l as i32 >= 97 && l as i32 <= 122 {
                result.push('_');
                result.push(' ');
            }
            if l as i32 == 32 {
                result.push(' ');
            }
        };

        result
        
    }

    pub fn remove_duplicates(list: &mut Vec<char>) {
        list.sort();
        list.dedup();
    }

    /*pub fn decrypt_encrypted(guess: char, encr: &mut String, word: String) {
        let mut positions: Vec<i32> = Vec::new();
        for c in word.chars() {
            if(c == char){
                positions.push(value: T)
            }
        }
    }*/

}