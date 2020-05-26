const RADIX: u32 = 10;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Num(i64),
    D,
    Plus,
    PlusEach,
    Minus,
    MinusEach,
    Explode,
    ExplodeEach,
    TakeHigh,
    TakeMiddle,
    TakeLow,
    NoOp,
    Comma,
}

impl Default for Token {
    fn default() -> Self {
        Token::NoOp
    }
}

pub fn tokens(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = &mut input.chars();
    let curr = iter.next();
    tokenize(&mut tokens, &mut iter, &curr);
    return tokens
}

pub fn tokenize(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, curr: &Option<char>) {
    match curr {
        Some(c) => match c {
            'd' | 'D' => {
                tokens.push(Token::D);
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
            '0'..='9' => return tokenize_num(tokens, iter, c.to_digit(RADIX).unwrap()),
            '+' | '-' | '!' => return tokenize_op(tokens, iter, *c),
            '~' => {
                tokens.push(Token::TakeMiddle);
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
            '`' => {
                tokens.push(Token::TakeLow);
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
            '^' => {
                tokens.push(Token::TakeHigh);
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
            '%' => return tokenize_percent(tokens, iter, 2),
            ',' => {
                tokens.push(Token::Comma);
                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
            _ => return,
        },
        None => return,
    }
}

fn tokenize_op(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, prev: char) {
    let curr = iter.next();
    match curr {
        Some(c) => {
            if c == prev {
                match prev {
                    '+' => tokens.push(Token::PlusEach),
                    '-' => tokens.push(Token::MinusEach),
                    '!' => tokens.push(Token::ExplodeEach),
                    _ => {}
                }

                let curr = iter.next();
                return tokenize(tokens, iter, &curr);
            }
        }
        None => {}
    }

    match prev {
        '+' => tokens.push(Token::Plus),
        '-' => tokens.push(Token::Minus),
        '!' => tokens.push(Token::Explode),
        _ => {}
    }
    return tokenize(tokens, iter, &curr);
}

fn tokenize_num(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, n: u32) {
    let curr = iter.next();
    match curr {
        Some(c) => match c {
            '0'..='9' => {
                let n = (n * 10) + c.to_digit(RADIX).unwrap();
                return tokenize_num(tokens, iter, n);
            }
            _ => {
                tokens.push(Token::Num(n as i64));
                return tokenize(tokens, iter, &curr);
            }
        },
        None => {
            tokens.push(Token::Num(n as i64));
            return tokenize(tokens, iter, &curr);
        }
    }
}

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
            tokens(&"d4".to_string()),
            vec![Token::D, Token::Num(4 as i64)]);
        assert_eq!(
            tokens(&"4d6".to_string()),
            vec![Token::Num(4 as i64), Token::D, Token::Num(6 as i64)]);
    }

    #[test]
    fn tokenize_with_modifier() {
        assert_eq!(
            tokens(&"d8+1".to_string()),
            vec![Token::D, Token::Num(8 as i64), Token::Plus, Token::Num(1 as i64)]);
    }
    
    #[test]
    fn tokenize_with_explode_modifier() {
        assert_eq!(
            tokens(&"2d4!+3".to_string()),
            vec![Token::Num(2 as i64), Token::D, Token::Num(4 as i64), Token::Explode, Token::Plus, Token::Num(3 as i64)]);
    }
}
