use std::io;
use std::io::prelude::*;
use std::fs;

extern crate clap;
use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("dice")
        .version(VERSION)
        .about("Generates random dice rolls")
        .author("Galen P.")
        .arg(
            Arg::with_name("tokenizer")
                .long("tokenizer")
                .help("Display the parsed tokens"),
        )
        .arg(
            Arg::with_name("generator")
                .long("generator")
                .help("Display the generators that were built"),
        )
        .arg(
            Arg::with_name("results")
                .long("results")
                .help("Deplay the results generated"),
        )
        .arg(
            Arg::with_name("file")
                .long("file")
                .takes_value(true)
                .help("Read the generator tokens from a file"),
        )
        .arg(
            Arg::with_name("chart")
                .long("chart")
                .help("Display the results as a chart"),
        )
        .arg(
            Arg::with_name("INPUT")
                .index(1)
                .multiple(true)
                .help("The dice string to parse"),
        )
        .get_matches();

    let input = input_string(&matches);
    let tokens = dice_dsl::tokens(&input);
    if matches.occurrences_of("tokenizer") > 0 {
        print!("Token:");
        for t in tokens.iter() {
            print!(" {:?}", t);
        }
        println!("");
    }

    match dice_dsl::Dice::build(&tokens, 0) {
        Some((dice, idx)) => {
            if matches.occurrences_of("generator") > 0 {
                if idx < tokens.len() {
                    println!("Warning: only {} of {} tokens consumed", idx, tokens.len());
                }
                println!("Generator: {}", dice);
            }

            if matches.occurrences_of("chart") > 0 {
                dice_dsl::chart(&dice, 100_000);
            } else {
                let results = dice.roll();
                if matches.occurrences_of("results") > 0 {
                    for r in results.rolls.iter() {
                        println!("Result: {}", r);
                    }
                    println!("Modifier: {}", results.modifier);
                }
        
                println!("{}", results.total());
            }
        },
        None => (),
    }
}

fn input_string(matcher: &clap::ArgMatches) -> String {
    match matcher.value_of("file") {
        Some(file)  => {
            match fs::read_to_string(file) {
                Ok(contents) => { 
                    return contents.trim().replace("\n", ",") 
                },
                Err(_) => panic!("file {} could not be read", file)
            }
        }
        None        => {
            match matcher.values_of("INPUT") {
                Some(values) => {
                    let mut tmp: Vec<String> = Vec::new();
                    for v in values { tmp.push(v.to_string()); }
                    return tmp.join(",")
                }
                None => {
                    let mut buffer = String::new();
                    match io::stdin().read_to_string(&mut buffer) {
                        Ok(_) => return buffer,
                        Err(_) => panic!("could not read from STDIN")
                    }
                }
            }
        }
    }
}
