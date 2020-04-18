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

    // let iterations = matches.value_of("iterations").unwrap_or("1");
    // let iterations = iterations.parse::<i32>().unwrap();

    let input = matches.value_of("input").unwrap();
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = &mut input.chars();
    let curr = iter.next();
    parse(&mut tokens, &mut iter, &curr);

    println!("Tokens parsed are:");
    for t in tokens {
        println!("{:?}", t);
    }
}

const RADIX: u32 = 10;

fn parse(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, curr: &Option<char>) {
    match curr {
        Some(c) => {
            match c {
                'd' | 'D' => {
                    tokens.push(Token::D);
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                },
                '0' ..= '9' => {
                    return parse_num(tokens, iter, c.to_digit(RADIX).unwrap())
                },
                '+' | '-' | '!' => {
                    return parse_op(tokens, iter, *c)
                },
                '~' | '`' | '^' => {
                    let op: String = vec![ c ].into_iter().collect();
                    tokens.push(Token::Op(String::from(op)));
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                },
                '%' => {
                    return parse_percent(tokens, iter, 2)
                },
                ',' => {
                    tokens.push(Token::Comma);
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                },
                _ => return
            }
        },
        None => return
    }
}

fn parse_op(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            if c == prev {
                let op: String = vec![ prev, c ].into_iter().collect();
                tokens.push(Token::Op(String::from(op)));
                let curr = iter.next();
                return parse(tokens, iter, &curr)
            }

            let op: String = vec![ prev ].into_iter().collect();
            tokens.push(Token::Op(String::from(op)));
            return parse(tokens, iter, &curr)
        },
        None => {
            let op: String = vec![ prev ].into_iter().collect();
            tokens.push(Token::Op(String::from(op)));
            return parse(tokens, iter, &curr)
        }
    }
}

fn parse_num(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, n: u32) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            match c {
                '0' ..= '9' => {
                    let n = (n * 10) + c.to_digit(RADIX).unwrap();
                    return parse_num(tokens, iter, n)
                },
                _ => {
                    tokens.push(Token::Num(n as u64));
                    return parse(tokens, iter, &curr)
                }
            }
        },
        None => {
            tokens.push(Token::Num(n as u64));
            return parse(tokens, iter, &curr)
        }
    }
}

fn parse_percent(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, cnt: u32) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            match c {
                '%' => return parse_percent(tokens, iter, cnt + 1),
                _ => {
                    tokens.push(Token::Num((10 as u64).pow(cnt as u32)));
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                }
            }
        },
        None => {
            tokens.push(Token::Num(cnt.pow(10) as u64));
            return parse(tokens, iter, &curr)
        }
    }
}

#[derive(Debug)]
enum Token {
    Num(u64),
    D,
    Op(String),
    Comma,
}
