extern crate clap;
use clap::{ App, Arg };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("dice").
        version(VERSION).
        about("Generates random dice rolls").
        author("Galen P.").
        arg(Arg::with_name("iterations").
            long("iterations").
            help("Number of iterations to run").
            takes_value(true)).
        arg(Arg::with_name("input").
            index(1).
            help("The dice string to parse").
            required(true)).
        get_matches();

    let iterations = matches.value_of("iterations").unwrap_or("1");
    let iterations = iterations.parse::<i32>().unwrap();

    let input = matches.value_of("input").unwrap();
    for (i, c) in input.chars().enumerate() {
        println!("{}. {}", i, c);
    }
}
