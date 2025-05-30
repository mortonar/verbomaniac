use clap::Parser;
use std::process::exit;
use verbomaniac_lib;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Define a word
struct Args {
    /// word to define
    word: String,
}

fn main() {
    let args = Args::parse();
    let word = &args.word;

    if let Ok(Some(word)) = verbomaniac_lib::define(word) {
        println!("-- {} --", word.word);
        for meaning in word.meanings {
            println!("\t{}", meaning.part_of_speech);
            for definition in meaning.definitions.iter().take(3) {
                println!("\t\t- {}", definition.definition);
            }
        }
    } else {
        eprintln!("Definition not found for \"{}\"", word);
        exit(2);
    }
}
