use super::Token;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Roll {
    pub range: i64,
    pub die_mod: i64,
    pub value: i64,
    pub explode: bool,
    pub success: bool,
    pub kept: bool,
}

impl Roll {
    pub fn roll(range: i64, modifier: i64, explode: bool) {
        let mut rng = rand::thread_rng();
        return Roll {
            range: range,
            die_mode: modifier,
            explode: explode,
            success: true,
            kept: true,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    pub rolls: Vec<Roll>,
    pub val_tgt: i64,
    pub all_mod: i64,
}

impl Result {
    pub fn sum(&self) -> i64 {
        let mut sum: i64 = 0;
        for r in self.rolls.iter() {
            if r.success && r.kept && r.value > self.val_tgt {
                sum += r.value;
            }
        }
        sum += self.all_mod;
        return sum;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Roller {
    // number of dice to roll
    pub count: i64,

    // the range of the die for this rolle (d4 => 4)
    pub range: i64,

    // the modifier type for this roller: e.g. ++, --, +, -, !
    pub op: Option<Token>,

    // target value: rolls that exceed this value are successes
    pub val_tgt: i64,

    // die mod: the modifier for each die rolled
    pub die_mod: i64,

    // all mod: the modifier for the entire roll
    pub all_mod: i64,
}

impl Roller {
    pub fn build(tokens: &Vec<Token>, idx: usize) -> (Roller, usize) {
        let mut roller = Roller {
            count: 0,
            range: 0,
            op: None,
            val_tgt: 0,
            die_mod: 0,
            all_mod: 0,
        };
        let idx = Roller::build_roller(&mut roller, tokens, idx);
        return (roller, idx);
    }

    pub fn roll(&self) -> Result {
        let mut result = Result {
            rolls: Vec::new(),
            val_tgt: self.val_tgt,
            all_mod: self.all_mod,
        };

        for _ in 0..self.count {
            let mut roll = self.roll_one();
            result.rolls.push(roll);
        }
        return result;
    }

    pub fn roll_one(&self) -> Roll {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, self.range + 1) as i64;
        let explode = self.op.unwrap_or_default() == Token::ExplodeEach;
        return Roll {
            range: self.range,
            die_mod: self.die_mod,
            value: (value + self.die_mod),
            explode: (explode && value == self.range),
            success: (value > self.val_tgt),
            kept: true,
        };
    }

    fn build_roller(roller: &mut Roller, tokens: &Vec<Token>, idx: usize) -> usize {
        let idx = Roller::build_count(roller, tokens, idx);
        let idx = Roller::build_range(roller, tokens, idx);
        let idx = Roller::build_op(roller, tokens, idx);
        let idx = Roller::build_mod(roller, tokens, idx);
        return idx;
    }

    fn build_count(roller: &mut Roller, tokens: &Vec<Token>, idx: usize) -> usize {
        if idx >= tokens.len() {
            return idx;
        }

        let token = &tokens[idx];
        match token {
            Token::D => {
                roller.count = 1;
                return idx + 1;
            }
            Token::Num(n) => {
                roller.count = *n;
                return idx + 2; // skip over D
            }
            _ => (),
        }
        return idx;
    }

    fn build_range(roller: &mut Roller, tokens: &Vec<Token>, idx: usize) -> usize {
        if idx >= tokens.len() {
            return idx;
        }

        let token = &tokens[idx];
        match token {
            Token::Num(n) => {
                roller.range = *n;
                return idx + 1;
            }
            _ => (),
        }
        return idx;
    }

    fn build_op(roller: &mut Roller, tokens: &Vec<Token>, idx: usize) -> usize {
        if idx >= tokens.len() {
            return idx;
        }

        let token = &tokens[idx];
        match token {
            Token::Plus
            | Token::PlusEach
            | Token::Minus
            | Token::MinusEach
            | Token::Explode
            | Token::ExplodeEach
            | Token::TakeHigh
            | Token::TakeMiddle
            | Token::TakeLow => {
                roller.op = Some(*token);
                return idx + 1;
            }
            _ => (),
        }

        return idx;
    }

    fn build_mod(roller: &mut Roller, tokens: &Vec<Token>, idx: usize) -> usize {
        if idx >= tokens.len() {
            return idx;
        }

        let token = &tokens[idx];
        match token {
            Token::Num(n) => {
                match roller.op {
                    Some(t) => match t {
                        Token::PlusEach | Token::MinusEach => {
                            roller.die_mod = *n;
                        }
                        Token::Plus | Token::Minus => {
                            roller.all_mod = *n;
                        }
                        _ => (),
                    },
                    None => (),
                }
                return idx + 1;
            }
            _ => (),
        }
        return idx;
    }
}

#[derive(Debug, Clone)]
pub struct ListRoller {
    pub rollers: Vec<Roller>,
}

impl ListRoller {
    // build a ListRoller from the vector of tokens, return the
    // generated ListRoller and the index of the next unconsumed
    // token.
    pub fn build(tokens: &Vec<Token>, start_idx: usize) -> (ListRoller, usize) {
        let mut list = ListRoller {
            rollers: Vec::new(),
        };
        let mut idx = start_idx;
        while idx < tokens.len() {
            let (result, loop_idx) = Roller::build(tokens, idx);
            list.rollers.push(result);
            idx = loop_idx;

            // if therer's unconsumed tokens but its not a comma, escape early
            if idx < tokens.len() {
                match tokens[idx] {
                    Token::Comma => {
                        idx += 1;
                    }
                    _ => return (list, idx),
                }
            }
        }
        return (list, idx);
    }

    // generate a list of results from the rollers in this generator
    pub fn roll(&self) -> Vec<Result> {
        let mut results: Vec<Result> = Vec::new();
        for roller in self.rollers.iter() {
            let result = roller.roll();
            results.push(result);
        }
        return results;
    }
}
