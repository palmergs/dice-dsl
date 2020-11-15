extern crate nom;

use nom::{
    IResult, 
    branch::alt,
    multi::many1_count,
    bytes::complete::tag,
    sequence::{delimited, tuple, preceded},
    character::complete::{char, digit0, digit1},
    combinator::opt,
};

const RADIX: u32 = 10;

enum ResultCmp {
    GT(i32),
    LT(i32),
    EQ(i32),
}

fn result_gt(input: &str) -> IResult<&str, ResultCmp> {
    match preceded(char('>'), digit1)(input) {
        Ok((input, chars)) => Ok((input, ResultCmp::GT(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn result_lt(input: &str) -> IResult<&str, ResultCmp> {
    match preceded(char('<'), digit1)(input) {
        Ok((input, chars)) => Ok((input, ResultCmp::LT(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn result_eq(input: &str) -> IResult<&str, ResultCmp> {
    match preceded(char('='), digit1)(input) {
        Ok((input, chars)) => Ok((input, ResultCmp::EQ(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn result_cmp(input: &str) -> IResult<&str, ResultCmp> {
    alt((result_gt, result_lt, result_eq))(input)
}

enum SumCmp {
    TargetHigh(i32),
    TargetLow(i32),
    TargetSucc(i32),
    TargetSuccNext(i32, i32),
}

fn target_high(input: &str) -> IResult<&str, SumCmp> {
    match delimited(char('['), digit1, char(']'))(input) {
        Ok((input, chars)) => Ok((input, SumCmp::TargetHigh(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn target_low(input: &str) -> IResult<&str, SumCmp> {
    match delimited(char('('), digit1, char(')'))(input) {
        Ok((input, chars)) => Ok((input, SumCmp::TargetLow(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn target_succ(input: &str) -> IResult<&str, SumCmp> {
    match delimited(char('{'), digit1, char('}'))(input) {
        Ok((input, chars)) => Ok((input, SumCmp::TargetSucc(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn target_succ_next(input: &str) -> IResult<&str, SumCmp> {
    match delimited(char('{'), tuple((digit1, digit1)), char('}'))(input) {
        Ok((input, (chars1, chars2))) => Ok((
            input, 
            SumCmp::TargetSuccNext(
                chars1.parse::<i32>().unwrap(), 
                chars2.parse::<i32>().unwrap()
            )
        )),
        Err(e) => Err(e),
    }
}

fn sum_cmp(input: &str) -> IResult<&str, SumCmp> {
    alt((target_high, target_low, target_succ, target_succ_next))(input)
}

enum RollOp {
    Explode(Option<i32>),
    ExplodeUntil(Option<i32>),
    ExplodeEach(Option<i32>),
    ExplodeEachUntil(Option<i32>),
    AddEach(Option<i32>),
    SubEach(Option<i32>),
    TakeMid(i32),
    TakeLow(i32),
    TakeHigh(i32),
    Disadvantage,
    Advantage,
    BestGroup,
}

fn option_i32(input: &str) -> Option<i32> {
    if input.len() == 0 { return None; }

    Some(input.parse::<i32>().unwrap())
}

fn explode_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(char('!'), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::Explode(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn explode_until_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(tag("!!"), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::ExplodeUntil(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn explode_each_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(char('*'), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::Explode(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn explode_each_until_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(tag("**"), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::ExplodeEachUntil(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn add_each_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(tag("++"), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::AddEach(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn sub_each_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(tag("--"), digit0)(input) {
        Ok((input, chars)) => Ok((input, RollOp::SubEach(option_i32(chars)))),
        Err(e) => Err(e)
    }
}

fn take_mid_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(char('~'), digit1)(input) {
        Ok((input, chars)) => Ok((input, RollOp::TakeMid(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e)
    }
}

fn take_low_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(char('`'), digit1)(input) {
        Ok((input, chars)) => Ok((input, RollOp::TakeLow(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e)
    }
}

fn take_high_op(input: &str) -> IResult<&str, RollOp> {
    match preceded(char('^'), digit1)(input) {
        Ok((input, chars)) => Ok((input, RollOp::TakeHigh(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e)
    }
}

fn disadvantage_op(input: &str) -> IResult<&str, RollOp> {
    match tag("DIS")(input) {
        Ok((input, _)) => Ok((input, RollOp::Disadvantage)),
        Err(e) => Err(e)
    }
}

fn advantage_op(input: &str) -> IResult<&str, RollOp> {
    match tag("ADV")(input) {
        Ok((input, _)) => Ok((input, RollOp::Advantage)),
        Err(e) => Err(e)
    }
}

fn best_group_op(input: &str) -> IResult<&str, RollOp> {
    match char('$')(input) {
        Ok((input, _)) => Ok((input, RollOp::BestGroup)),
        Err(e) => Err(e)
    }
}

fn roll_op(input: &str) -> IResult<&str, RollOp> {
    alt((
        explode_until_op, 
        explode_op, 
        explode_each_until_op,
        explode_each_op, 
        add_each_op,
        sub_each_op,
        take_mid_op,
        take_low_op,
        take_high_op,
        disadvantage_op,
        advantage_op,
        best_group_op
    ))(input)
}

fn percent(input: &str) -> IResult<&str, &str> {
    match many1_count(char('%'))(input) {
        Ok((input, n)) => {
            match n {
                1 => Ok((input, "100")),
                2 => Ok((input, "1000")),
                3 => Ok((input, "10000")),
                _ => Ok((input, "100000")),
            }
        },
        Err(e) => Err(e)
    }
}

fn range(input: &str) -> IResult<&str, i32> {
    match alt((digit1, percent))(input) {
        Ok((input, chars)) => Ok((input, chars.parse::<i32>().unwrap())),
        Err(e) => Err(e)
    }
}

struct Roller {
    count: i32,
    range: i32,
    op: Option<RollOp>,
}

fn roller(input: &str) -> IResult<&str, Roller> {
    match tuple((
            opt(digit1),
            alt((char('d'), char('D'))),
            range,
            opt(roll_op)
        ))(input) {

        Ok((input, (count, _, range, op))) => {
            let count = match count {
                Some(n) => n.parse::<i32>().unwrap(),
                None => 1,
            };

            Ok((input, Roller{ count, range, op }))
        }
        Err(e) => Err(e)
    }
}

enum Val {
    Constant(i32),
    Random(Roller),
}

fn val(input: &str) -> IResult<&str, Val> {
    match opt(roller)(input) {
        Ok((input, Some(roller))) => Ok((input, Val::Random(roller))),
        Ok((input, None)) => {
            match(digit1(input)) {
                Ok((input, chars)) => Ok((input, Val::Constant(chars.parse::<i32>().unwrap()))),
                Err(e) => Err(e),
            }
        }
    }
}

enum ModOp {
    Add,
    Sub,
}

struct ModVal {
    op: ModOp,
    val: Val,
}

struct Expr {
    first: Val,
    rest: Vec<Val>,
}

fn expr(input: &str) -> IResult<&str, Expr> {
    Ok((input, Expr{ first: Val::Constant(3), rest: vec![] }))
}

struct Scalar {
    expr: Expr,
    op: Option<SumCmp>,
}

// fn scalar(input: &str) -> IResult<&str, Expr> { }

struct Generator {
    scalars: Vec<Scalar>,
    op: Option<ResultCmp>,
}

fn generator(input: &str) -> IResult<&str, Generator> {
    Ok((input, Generator{ scalars: vec![], op: None }))
}


// Parser Notation:

// <result>        ::= <scalar> | <result> <scalar_op>
// <scalar>        ::= <vector> | <scalar> <vector_op> 
// <vector>        ::= <value> | <vector> , <value>
// <value>         ::= <seq> | <value> <mod> <num>
// <seq>           ::= <roll> | <seq> <roll_op>
// <die>          ::= [dD]<num> | <num> <die> 
// <mod>           ::= + | - 
// <num>           ::= [1-9][0-9]* | %+
// <roll_op>       ::= ! | !! | 
//                     * | ** | 
//                     ++ | -- |
//                     $ | ~ | ` | ^ | 
//                     DIS | ADV | 
//                     <roll_op> <num>
// <vector_op>     ::= = <num> | > <num> | < <num> | 
//                     [ <num> ] | ( <num> ) | { <num> } | { <num> , <num> }
// <scalar_op>     ::= >> <num> | << <num> | == <num> | Y <num>? | / <num>

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token { 
    NoOp,
    Num(i64), D, Add, Sub, 
    Op(char),
    Op2(char),
    Comma,
    Start(char), End(char),
}

impl Default for Token {
    fn default() -> Self {
        Token::NoOp
    }
}

impl Token {
    pub fn expect(tokens: &Vec<Token>, idx: usize, expected: &Token, ) -> bool {
        match tokens.get(idx) {
            Some(t) => {
                return t == expected
            },
            None => false,
        }
    }
    
    pub fn expect_num(tokens: &Vec<Token>, idx: usize) -> Option<i64> {
        match tokens.get(idx) {
            Some(t) => {
               return match t {
                    Token::Num(n) => Some(*n),
                    _ => None,
                }
            },
            None => None,
        }
    }
    
    pub fn expect_char(tokens: &Vec<Token>, idx: usize) -> Option<char> {
        match tokens.get(idx) {
            Some(t) => {
                return match t {
                    Token::Start(c) => Some(*c),
                    Token::End(c) => Some(*c),
                    _ => None,
                }
            },
            None => None,
        }
    }
}

// Given a string iterate over the characters and build a 
// vector of dice roll tokens.
pub fn tokens(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = &mut input.chars();
    let curr = iter.next();
    tokenize(&mut tokens, &mut iter, &curr);
    return tokens
}

// Populate the vector of tokens by iterating through the
// source string one character at a time. If a function consumes
// the character then it must advance the iterator before recursively
// calling tokenize again. 
pub fn tokenize(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, curr: &Option<char>) {
    match curr {
        Some(c) => match c {
            'a' | 'A' => return tokenize_word(tokens, iter, *c),
            'd' | 'D' => return tokenize_word(tokens, iter, *c),
            '0'..='9' => return tokenize_num(tokens, iter, c.to_digit(RADIX).unwrap()),
            '%' => return tokenize_percent(tokens, iter, 2),
            '+' | '-' | '!' | '*' | '>' | '<' | '=' | '$' => return tokenize_op2(tokens, iter, *c),
            '~' | '`' | '^' | 'Y' | '/' => return tokenize_op(tokens, iter, *c),
            '(' | '{' | '[' => return tokenize_start(tokens, iter, *c),
            ')' | '}' | ']' => return tokenize_end(tokens, iter, *c),   
            ',' => tokenize_comma(tokens, iter),
            '\n' | '\t' | '\r' | ' ' => tokenize_whitespace(tokens, iter, *c),
            _ => println!("Unexpected token: char={}", c),
        },
        None => return,
    }
}

fn tokenize_whitespace(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, _prev: char) {
    let curr = iter.next();
    tokenize(tokens, iter, &curr);
}

fn tokenize_start(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    tokens.push(Token::Start(prev));
    let curr = iter.next();
    tokenize(tokens, iter, &curr);
}

fn tokenize_end(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    tokens.push(Token::End(prev));
    let curr = iter.next();
    tokenize(tokens, iter, &curr);
}

fn tokenize_comma(tokens: &mut Vec<Token>, iter: &mut std::str::Chars) {
    tokens.push(Token::Comma);
    let curr = iter.next();
    tokenize(tokens, iter, &curr);
}

fn tokenize_op(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    tokens.push(Token::Op(prev));
    let curr = iter.next();
    tokenize(tokens, iter, &curr);
}

fn tokenize_op2(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    let mut curr = iter.next();
    match curr {
        Some(c) => {
            if c == prev {
                tokens.push(Token::Op2(c));
                curr = iter.next();
            } else {
                if prev == '+' {
                    tokens.push(Token::Add);
                } else if prev == '-' {
                    tokens.push(Token::Sub);
                } else {
                    tokens.push(Token::Op(prev));
                }
            }
        },
        None => {
            if prev == '+' {
                tokens.push(Token::Add);
            } else if prev == '-' {
                tokens.push(Token::Sub);
            } else {
                tokens.push(Token::Op(prev));
            }
        }
    }
    tokenize(tokens, iter, &curr)
}

// Currently there's only three discrete words: DIS (for disadvantage),
// ADV (for advantage) and D. This will need to be made more complex if 
// additional or arbitrary words (functions or labels, for example) are
// added.
fn tokenize_word(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            match c {
                'i' | 'I' => {
                    if prev == 'd' || prev == 'D' {
                        return tokenize_word(tokens, iter, c);
                    }
                },
                's' | 'S' => {
                    if prev == 'i' || prev == 'I' {
                        let curr = iter.next();
                        tokens.push(Token::Op('D'));
                        return tokenize(tokens, iter, &curr);
                    }
                },
                'd' | 'D' => {
                    if prev == 'a' || prev == 'A' {
                        return tokenize_word(tokens, iter, c);
                    }
                },
                'v' | 'V' => {
                    if prev == 'd' || prev == 'D' {
                        let curr = iter.next();
                        tokens.push(Token::Op('A'));
                        return tokenize(tokens, iter, &curr);
                    }
                },
                _ => (),        
            }
        }
        None => println!("No work char: prev={}", prev),
    }

    if prev == 'd' || prev == 'D' {
        tokens.push(Token::D);
        return tokenize(tokens, iter, &curr);
    }
}

fn tokenize_num(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, n: u32) {
    let curr = iter.next();
    match curr {
        Some(c) => match c {
            '0'..='9' => {
                let n = (n * 10) + c.to_digit(RADIX).unwrap();
                return tokenize_num(tokens, iter, n);
            },
            _ => {
                tokens.push(Token::Num(n as i64));
                return tokenize(tokens, iter, &curr);
            },
        },
        None => {
            tokens.push(Token::Num(n as i64));
            return tokenize(tokens, iter, &curr);
        }
    }
}

// % can be repeated any number of times and is 
// converted into an integer with one 0 per % char
fn tokenize_percent(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, cnt: u32) {
    let curr = iter.next();
    match curr {
        Some(c) => match c {
            '%' => return tokenize_percent(tokens, iter, cnt + 1),
            _ => {
                tokens.push(Token::Num((10 as i64).pow(cnt as u32)));
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
        },
        None => {
            tokens.push(Token::Num((10 as i64).pow(cnt as u32)));
            return tokenize(tokens, iter, &curr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        assert_eq!(
            tokens("d4"),
            vec![Token::D, Token::Num(4 as i64)]);
        assert_eq!(
            tokens("4d6"),
            vec![Token::Num(4 as i64), Token::D, Token::Num(6 as i64)]);
    }

    #[test]
    fn tokenize_percent() {
        assert_eq!(tokens("%"), vec![Token::Num(100 as i64)]);
        assert_eq!(tokens("%%"), vec![Token::Num(1000 as i64)]);
        assert_eq!(tokens("%%%"), vec![Token::Num(10000 as i64)]);
    }

    #[test]
    fn tokenize_with_modifier() {
        assert_eq!(
            tokens("d8+1"),
            vec![Token::D, Token::Num(8 as i64), Token::Add, Token::Num(1 as i64)]);
    }

    #[test]
    fn tokenize_with_function() {
        assert_eq!(
            tokens("d20 ADV + 3"),
            vec![Token::D, Token::Num(20 as i64), Token::Op('A'), Token::Add, Token::Num(3 as i64)]);

        assert_eq!(
            tokens("d20 ADV 2 + 3"),
            vec![Token::D, Token::Num(20 as i64), Token::Op('A'), Token::Num(2 as i64), Token::Add, Token::Num(3 as i64)]);

        assert_eq!(
            tokens("d20 DIS - 1"),
            vec![Token::D, Token::Num(20 as i64), Token::Op('D'), Token::Sub, Token::Num(1 as i64)]);
    }
    
    #[test]
    fn tokenize_with_explode_modifier() {
        assert_eq!(
            tokens("2d4!+3"),
            vec![Token::Num(2 as i64), Token::D, Token::Num(4 as i64), Token::Op('!'), Token::Add, Token::Num(3 as i64)]);
        assert_eq!(
            tokens("2d4!! + 3"),
            vec![Token::Num(2 as i64), Token::D, Token::Num(4 as i64), Token::Op2('!'), Token::Add, Token::Num(3 as i64)]);    
    }

    #[test]
    fn tokenize_with_explode_each_modifier() {
        assert_eq!(
            tokens("3d6*"),
            vec![Token::Num(3 as i64), Token::D, Token::Num(6 as i64), Token::Op('*')]);

        assert_eq!(
            tokens("3d6**"),
            vec![Token::Num(3 as i64), Token::D, Token::Num(6 as i64), Token::Op2('*')]);            
    }

    #[test]
    fn tokenize_commas() {
        assert_eq!(
            tokens("1, 2, 3"), 
            vec![Token::Num(1 as i64), Token::Comma, Token::Num(2 as i64), Token::Comma, Token::Num(3 as i64)]);
    }

    #[test]
    fn tokenize_brackets() {
        assert_eq!(
            tokens("d10{4,2}"), 
            vec![Token::D, Token::Num(10 as i64), Token::Start('{'), Token::Num(4 as i64), Token::Comma, Token::Num(2 as i64), Token::End('}')]);
    }

    #[test]
    fn tokenize_bad_input() {
        assert_eq!(tokens("weasel"), vec![]);
        assert_eq!(tokens("dwight"), vec![Token::D]);
        assert_eq!(tokens("=test"), vec![Token::Op('=')]);
    }
}
