const RADIX: u32 = 10;

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
            'a' | 'A' => return tokenize_word(tokens, iter, &curr, *c),
            'd' | 'D' => return tokenize_word(tokens, iter, &curr, *c),
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
fn tokenize_word(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, curr: &Option<char>, prev: char) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            match c {
                'i' | 'I' => {
                    if prev == 'd' || prev == 'D' {
                        return tokenize_word(tokens, iter, &curr, c);
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
                        return tokenize_word(tokens, iter, &curr, c);
                    }
                },
                'v' | 'V' => {
                    if prev == 'd' || prev == 'D' {
                        let curr = iter.next();
                        tokens.push(Token::Op('A'));
                        return tokenize(tokens, iter, &curr);
                    }
                },
                _ => println!("Tokenize word char: curr={} prev={}", c, prev),        
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
