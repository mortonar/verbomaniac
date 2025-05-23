use std::env::args;
use std::process::exit;
use verbomaniac_lib;

fn main() {
    let word = match args().nth(1) {
        None => {
            eprintln!("Please provide a word to define");
            exit(1);
        }
        Some(w) => w,
    };

    let word = verbomaniac_lib::define(&word).unwrap().expect("No result");
    println!("-- {} --", word.word);
    for meaning in word.meanings {
        println!("\t{}", meaning.part_of_speech);
        for definition in meaning.definitions.iter().take(3) {
            println!("\t\t- {}", definition.definition);
        }
    }
}
