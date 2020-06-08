use super::{ Token, Results };

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RollOp {
    Explode(i64), ExplodeUntil(i64),
    ExplodeEach(i64), ExplodeEachUntil(i64), 
    AddEach(i64), SubEach(i64),
    Crit(i64), TakeMid(i64), TakeLow(i64), TakeHigh(i64),
    Disadvantage(i64), Advantage(i64),
}

impl RollOp {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> Option<(RollOp, usize)> {
        let mut curr = idx;
        match tokens.get(idx) {
            Some(t) => {
                let (c, bool) = match t {
                    Token::Op(c) => {
                        curr = curr + 1;
                        (c, false)
                    },
                    Token::Op2(c) => {
                        curr = curr + 1;
                        (c, true)
                    },
                    _ => return None
                };

                let n = if let Some(n) = Token::expect_num(tokens, curr) {
                    curr = curr + 1;
                    n
                } else {
                    1
                };
                
                match (c, bool) {
                    ('!', false) => Some((RollOp::Explode(n), curr)),
                    ('!', true) => Some((RollOp::ExplodeUntil(n), curr)),
                    ('*', false) => Some((RollOp::ExplodeEach(n), curr)),
                    ('*', true) => Some((RollOp::ExplodeEachUntil(n), curr)),
                    ('$', false) => Some((RollOp::Crit(n), curr)),
                    ('~', false) => Some((RollOp::TakeMid(n), curr)),
                    ('`', false) => Some((RollOp::TakeLow(n), curr)),
                    ('^', false) => Some((RollOp::TakeHigh(n), curr)),
                    ('+', true) => Some((RollOp::AddEach(n), curr)),
                    ('-', true) => Some((RollOp::SubEach(n), curr)),
                    ('D', false) => Some((RollOp::Disadvantage(n), curr)),
                    ('A', false) => Some((RollOp::Advantage(n), curr)),
                    _ => None,
                }
            },
            None => None
        }
    }

    pub fn apply(&self, results: &Results) -> Results {
        Results{ rolls: vec![], total: 0 }
    }
}


mod tests {
    use super::*;

    #[test]
    fn build_op() {
        assert_eq!(RollOp::build(&vec![Token::Op('!')], 0), Some((RollOp::Explode(1), 1)));
        assert_eq!(RollOp::build(&vec![Token::Op2('+'), Token::Num(4 as i64)], 0), Some((RollOp::AddEach(4), 2)));
    }
}