mod hm_tools;

use hm_tools::{tools, art};
use std::io;

fn main() {
    std::process::Command::new("clear").status().unwrap();
    println!("Welcome to Hangman Official Terminal Game 1978 Copyright, all Right Reserved");

    start();

}

fn start() {

    let mut mistakes = 0;
    let secret = tools::random_word();
    let mut temp: Vec<char> = secret.clone().chars().collect();
    tools::remove_duplicates(&mut temp);
    let mut encrypted = tools::encrypt_word(&secret);
    let mut guess: char;

    loop{
        println!("{}\n", art::SPRITES[mistakes]);
        println!("{}\n", encrypted);
        //println!("{}\n", secret);
        //println!("{:?}\n", temp);
        
        guess = get_input();
        if is_guessed(guess, &mut temp) {
            tools::decrypt_encrypted(guess, &mut encrypted, &secret);
        }
        else {
            mistakes += 1;
        }

        std::process::Command::new("clear").status().unwrap();

        if temp.is_empty() {
            println!("______________________You Win!________________________");
            break;
        }

    };

}

fn is_guessed(guess: char, list: &mut Vec<char>) -> bool{

    if list.contains(&guess){
        list.remove(list.iter().position(|&p| p == guess).unwrap());

        return true;
    }

    false
}

fn get_input() -> char {

    let mut input: String;
    
    loop {
        input = String::new();
        println!("Try guessing the letter!");
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.trim().chars().count() == 1 {
                    return input.trim().chars().nth(0).unwrap();
                }
                println!("Enter only one letter!");
                continue;
            },
            Err(_err) => {
                println!("Failed reading the input!");
                continue;
            },
        }
    };

}