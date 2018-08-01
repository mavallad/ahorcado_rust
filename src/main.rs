mod graphics;

#[macro_use] extern crate text_io;
extern crate colored;
use std::io::prelude::*;
use std::io;
use std::collections::HashSet;
use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid parameters
            Usage: ahorcado <palabra secreta>");
        std::process::exit(1);
    }
    let secret = args[1].to_uppercase();
    clear_console();
    play_game(secret);
}

fn play_game(secret: String) {
    let secret_letters = extract_letters(&secret);
    let mut finish = false;
    let mut attempts_left: u8 = 6;
    let mut letters_found: HashSet<char> = HashSet::new();
    while !finish {
        println!("INTENTOS: {}", attempts_left);
        println!("{}", graphics::paint(attempts_left));
        println!("\n      {}\n", pattern_secret(&secret, &letters_found).yellow().bold());
        print!("Letra: ");
        io::stdout().flush().ok().expect("No podemos limpiar la salida");
        let rc: Result<char, _> = try_read!();
        clear_console();
        match rc {
            Ok(c) if is_valid_letter(&c) => {
                let new_char = uppercase(c);
                if secret_letters.contains(&new_char) {
                    letters_found.insert(new_char);
                    if letters_found == secret_letters {
                        println!("CAMPEÓN: Has acertado la palabra secreta {}", secret.blue().bold());
                        println!("{}", graphics::paint(attempts_left));
                        finish = true;
                    }
                } else {
                    println!("ERROR. {} no está en la palabra", new_char.to_string().bold());
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        println!("{}", "HAS PERDIDO!".red());
                        println!("{}", graphics::paint(attempts_left));
                        println!("Has acabado tus intentos. La palabra secreta es: {}", secret);
                        finish = true;
                    }
                }
            },
            _ => println!("Por favor, introduce sólo una letra")
        }
    }
}

fn extract_letters(secret: &String) -> HashSet<char> {
    let mut letters = HashSet::new();
    for c in secret.chars() {
        letters.insert(c);
    }
    letters
}

fn pattern_secret(secret: &String, letters_found: &HashSet<char>) -> String {
    let mut ret = String::new();
    for c in secret.chars() {
        if letters_found.contains(&c) {
            ret.push(c);
        } else {
            ret.push('_');
        }
    }
    ret
}

fn is_valid_letter(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == 'ñ' || *c == 'Ñ'
}

fn uppercase(c: char) -> char {
    if c == 'ñ' {
        'Ñ'
    } else {
        c.to_ascii_uppercase()
    }
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}
