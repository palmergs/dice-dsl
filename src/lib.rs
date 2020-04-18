const RADIX: u32 = 10;

#[derive(Debug)]
pub enum Token {
    Num(u64),
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
    Comma,
}

pub fn parse(tokens: &mut Vec<Token>, iter: &mut std::str::Chars, curr: &Option<char>) {
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
                '~' => {
                    tokens.push(Token::TakeMiddle); 
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                },
                '`' => {
                    tokens.push(Token::TakeLow);
                    let curr = iter.next();
                    return parse(tokens, iter, &curr)
                },
                '^' => {
                    tokens.push(Token::TakeHigh);
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
                match prev {
                    '+' => tokens.push(Token::PlusEach),
                    '-' => tokens.push(Token::MinusEach),
                    '!' => tokens.push(Token::ExplodeEach),
                    _ => {}
                }

                let curr = iter.next();
                return parse(tokens, iter, &curr)
            }
        },
        None => {}
    }

    match prev {
        '+' => tokens.push(Token::Plus),
        '-' => tokens.push(Token::Minus),
        '!' => tokens.push(Token::Explode),
        _ => {}
    }
    return parse(tokens, iter, &curr)
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
            tokens.push(Token::Num((10 as u64).pow(cnt as u32)));
            return parse(tokens, iter, &curr)
        }
    }
}


