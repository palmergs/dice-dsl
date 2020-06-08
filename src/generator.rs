use rand::Rng;

use super::Token;
use super::{ Roll, Results, RollOp };

#[derive(Debug, Clone, PartialEq)]
pub struct Die {
    pub count: i64,
    pub range: i64,
    pub ops: Vec<RollOp>,
}

impl Die {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> Option<(Die, usize)> {
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

        Some((Die{ count: count, range: range, ops: ops }, curr))
    }

    pub fn roll(&self, modifier: i64) -> Results {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<Roll> = Vec::new();
        for _ in 0..self.count {
            let roll = rng.gen_range(1, self.range + 1) as i64;
            vec.push(Roll{ 
                range: self.range, 
                roll: roll, 
                modifier: modifier, 
                total: roll + modifier, 
                keep: true,
                crit: false,
                bonus: false });
        }

        let total = vec.iter().map(|r| r.total).sum();
        let mut results = Results{ rolls: vec, total: total };
        for op in self.ops.iter() {
            results = op.apply(&results);
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_die() {
        assert_eq!(
            Die::build(&vec![Token::D, Token::Num(4 as i64)], 0),
            Some((Die{ count: 1, range: 4, ops: vec![] }, 2))
        );

        assert_eq!(
            Die::build(&vec![Token::Num(3 as i64), Token::D, Token::Num(6 as i64)], 0),
            Some((Die{ count: 3, range: 6, ops: vec![] }, 3))
        );        
    }
}