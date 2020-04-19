use std::fmt;

use super::Token;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Roll {
    pub range: i64,
    pub die_mod: i64,
    pub value: i64,
    pub explode: bool
}

impl Roll {
    pub fn roll(range: i64, modifier: i64, explode: bool) -> Roll {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1) as i64;
        return Roll {
            range: range,
            die_mod: modifier,
            value: value,
            explode: explode
        }
    }

    pub fn total(&self) -> i64 {
        return self.value + self.die_mod
    }

    pub fn max(&self) -> bool {
        return self.value == self.range
    }

    pub fn min(&self) -> bool {
        return self.value == 1
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    pub rolls: Vec<Roll>,
    pub all_mod: i64,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, r) in self.rolls.iter().enumerate() {
            if idx > 0 { write!(f, " + ")?; }
            if r.die_mod > 0 {
                write!(f, "d{}+{}", r.value, r.die_mod)?;
            } else if r.die_mod < 0 {
                write!(f, "d{}{}", r.value, r.die_mod)?;
            } else {
                write!(f, "d{}", r.value)?;
            }
            if r.explode { write!(f, " (*)")?; }
        }
        if self.all_mod > 0 { write!(f, " + {}", self.all_mod)?; }
        if self.all_mod < 0 { write!(f, " - {}", self.all_mod.abs())?; }

        write!(f, " = ")?;       
        
        let mut cnt = 0; // keep track of how many numbers are being added 
        for (idx, r) in self.rolls.iter().enumerate() {
            if idx > 0 { write!(f, " + ")?; }
            if r.die_mod > 0 {
                write!(f, "{}+{}", r.value, r.die_mod)?;
                cnt += 2;
            } else if r.die_mod < 0 {
                write!(f, "{}{}", r.value, r.die_mod)?;
                cnt += 2;
            } else {
                write!(f, "{}", r.value)?;
                cnt += 1;
            }
            if r.explode { write!(f, " (*)")?; }
        }
        if self.all_mod > 0 { 
            write!(f, " + {}", self.all_mod)?; 
            cnt += 1;
        }
        if self.all_mod < 0 { 
            write!(f, " - {}", self.all_mod.abs())?; 
            cnt += 1;
        }

        // if more than one number was added in the previous section, summarize here
        if cnt > 1 { write!(f, " = {}", self.sum())?; }

        return Ok(())
    }
}

impl Result {
    pub fn sum(&self) -> i64 {
        let mut sum: i64 = 0;
        for r in self.rolls.iter() { sum += r.total(); }
        sum += self.all_mod;
        return sum;
    }

    pub fn target(&self, tgt: i64) -> i64 {
        let mut count:i64 = 0;
        for r in self.rolls.iter() {
            if r.total() > tgt { count += 1; }
        }
        return count
    }

    pub fn max(&self) -> bool {
        for roll in self.rolls.iter() { if !roll.max() { return false } }
        return true
    }  

    pub fn min(&self) -> bool {
        for roll in self.rolls.iter() { if !roll.min() { return false } }
        return true
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
            die_mod: 0,
            all_mod: 0,
        };
        let idx = Roller::build_roller(&mut roller, tokens, idx);
        return (roller, idx);
    }

    pub fn roll(&self) -> Result {
        let mut result = Result {
            rolls: Vec::new(),
            all_mod: self.all_mod,
        };

        let explode_each = self.op.unwrap_or_default() == Token::ExplodeEach;
        for _ in 0..self.count {
            let mut roll = Roll::roll(self.range, self.die_mod, false);
            result.rolls.push(roll);
            while explode_each && roll.value == roll.range {
                roll = Roll::roll(self.range, self.die_mod, true);
                result.rolls.push(roll);
            }
        }

        let explode_all = self.op.unwrap_or_default() == Token::Explode;
        if explode_all {
            if result.max() {
                let mut roll = Roll::roll(self.range, self.die_mod, true);
                result.rolls.push(roll);
                while roll.value == roll.range {
                    roll = Roll::roll(self.range, self.die_mod, true);
                    result.rolls.push(roll);
                }
            }        
        }

        return result;
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
                        Token::PlusEach => {   
                            roller.die_mod = *n;
                        }
                        Token::MinusEach => {
                            roller.die_mod = *n * -1;
                        }
                        Token::Plus => {
                            roller.all_mod = *n;
                        }
                        Token::Minus => {
                            roller.all_mod = *n * -1;
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
