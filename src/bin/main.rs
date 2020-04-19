extern crate clap;
use clap::{ App, Arg };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("dice").
        version(VERSION).
        about("Generates random dice rolls").
        author("Galen P.").
        arg(Arg::with_name("tokenizer").
            long("tokenizer").
            help("Display the parsed tokens")).
        arg(Arg::with_name("generator").
            long("generator").
            help("Display the generators that were built")).
        arg(Arg::with_name("results").
            long("results").
            help("Deplay the results generated")).
        arg(Arg::with_name("input").
            index(1).
            help("The dice string to parse").
            required(true)).
        get_matches();

    // let iterations = args.value_of("iterations").unwrap_or("1");
    // let iterations = iterations.parse::<i32>().unwrap();

    let input = matches.value_of("input").unwrap();
    let mut tokens: Vec<dice_dsl::Token> = Vec::new();
    let mut iter = &mut input.chars();
    let curr = iter.next();
    dice_dsl::tokenize(&mut tokens, &mut iter, &curr);

    if matches.occurrences_of("tokenizer") > 0 {
        print!("Token:");
        for t in tokens.iter() { print!(" {:?}", t); }
        println!("");
    }

    let (roller, idx) = dice_dsl::ListRoller::build(&tokens, 0);
    if matches.occurrences_of("generator") > 0 {
        if idx < tokens.len() { 
            println!("Warning: only {} of {} tokens consumed", idx, tokens.len());
        }
        println!("Generator: {:?}", roller);
    }

    let results = roller.roll();
    if matches.occurrences_of("results") > 0 {
        for r in results.iter() {
            println!("Result: {:?}", r);
        }
    }

    let mut sum: i64 = 0;
    for r in results.iter() {
        sum += r.sum();
    }
    println!("{}", sum);
}

