use rand::Rng;

use super::Token;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Const {
    pub value: i64,
}

impl Const {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> Option<(Const, usize)> {
        match expect_num(tokens, idx) {
            Some(n) => Some((Const{ value: n }, idx + 1)),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Die {
    pub count: i64,
    pub range: i64,
    pub keep: bool,
}

impl Die {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> Option<(Die, usize)> {
        let mut curr = idx;
        let count = match expect_num(tokens, curr) {
            Some(n) => {
                curr = curr + 1;
                n
            },
            None => 1,
        };

        if expect(tokens, curr, &Token::D) {
            curr = curr + 1;
        } else {
            return None
        }

        let range = match expect_num(tokens, curr) {
            Some(n) => {
                curr = curr + 1;
                n
            },
            None => return None,
        };

        Some((Die{ count: count, range: range, keep: true }, curr))
    }

    pub fn roll(&self) -> i64 {
        let mut rng = rand::thread_rng();
        let mut total = 0i64;
        for _ in 0..self.count {
            total = total + rng.gen_range(1, self.range + 1) as i64;
        }
        total
    }
}

fn expect(tokens: &Vec<Token>, idx: usize, expected: &Token, ) -> bool {
    match tokens.get(idx) {
        Some(t) => {
            return t == expected
        },
        None => false,
    }
}

fn expect_num(tokens: &Vec<Token>, idx: usize) -> Option<i64> {
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

fn expect_char(tokens: &Vec<Token>, idx: usize) -> Option<char> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_consts() {
        assert_eq!(
            Const::build(&vec![Token::Num(123 as i64), Token::Num(234 as i64)], 0), 
            Some((Const{ value: 123 }, 1))
        );

        assert_eq!(
            Const::build(&vec![Token::D, Token::Num(4 as i64)], 0),
            None
        );
    }

    #[test]
    fn build_die() {
        assert_eq!(
            Die::build(&vec![Token::D, Token::Num(4 as i64)], 0),
            Some((Die{ count: 1, range: 4, keep: true }, 2))
        );

        assert_eq!(
            Die::build(&vec![Token::Num(3 as i64), Token::D, Token::Num(6 as i64)], 0),
            Some((Die{ count: 3, range: 6, keep: true }, 3))
        );        
    }
}