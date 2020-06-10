use std::fmt;

use super::Token;
use super::{ Roll, Results, RollOp };

#[derive(Debug, Clone, PartialEq)]
pub struct Dice {
    pub count: i64,
    pub range: i64,
    pub ops: Vec<RollOp>,
}

impl Dice {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> Option<(Dice, usize)> {
        let mut curr = idx;
        let count = match Token::expect_num(tokens, curr) {
            Some(n) => {
                curr = curr + 1;
                n
            },
            None => 1,
        };

        if Token::expect(tokens, curr, &Token::D) {
            curr = curr + 1;
        } else {
            return None
        }

        let range = match Token::expect_num(tokens, curr) {
            Some(n) => {
                curr = curr + 1;
                n
            },
            None => return None,
        };

        let mut ops: Vec<RollOp> = Vec::new();
        let mut found_op = RollOp::build(tokens, curr);
        while found_op != None {
            let (op, tmp) = found_op.unwrap();
            curr = tmp;
            ops.push(op);
            found_op = RollOp::build(tokens, curr);
        }

        Some((Dice{ count: count, range: range, ops: ops }, curr))
    }

    pub fn roll(&self) -> Results {
        let mut vec: Vec<Roll> = Vec::new();
        for _ in 0..self.count {
            vec.push(Roll::new(self.range, 0));
        }

        let total = vec.iter().map(|r| r.total).sum();
        let mut results = Results{ rolls: vec, total: total };
        for op in self.ops.iter() {
            op.apply(&mut results);
        }
        
        results
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.count > 0 {
            write!(f, "{}", self.count)?;
        }

        write!(f, "d{}", self.range)?;
        for op in self.ops.iter() {
            write!(f, "{}", op)?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_die() {
        assert_eq!(
            Dice::build(&vec![Token::D, Token::Num(4 as i64)], 0),
            Some((Dice{ count: 1, range: 4, ops: vec![] }, 2))
        );

        assert_eq!(
            Dice::build(&vec![Token::Num(3 as i64), Token::D, Token::Num(6 as i64)], 0),
            Some((Dice{ count: 3, range: 6, ops: vec![] }, 3))
        );        
    }
}
