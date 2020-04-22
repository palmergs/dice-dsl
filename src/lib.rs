mod tokenizer;
pub use tokenizer::{tokenize, tokens, Token};

mod generator;
pub use generator::{ListRoller, Result, Roll, Roller};

pub fn roller(input: &String) -> ListRoller {
    let tokens = tokens(input);
    let (roller, _) = ListRoller::build(&tokens, 0);
    return roller
}

