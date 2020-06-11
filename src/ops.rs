use std::fmt;

use super::{ Dice, Roll, Token, Results };

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
                    // NOTE: for may operations the number 0 may be treated specially;
                    // e.g. Crit -1 == "maximum range value"; all other values are "n or higher"
                    -1 
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

    pub fn apply(&self, results: &mut Results) {
        match self {
            RollOp::Explode(n) => return RollOp::explode(results, *n, false),
            RollOp::ExplodeUntil(n) => return RollOp::explode(results, *n, true),
            RollOp::ExplodeEach(n) => return RollOp::explode_each(results, *n, false),
            RollOp::ExplodeEachUntil(n) => return RollOp::explode_each(results, *n, true),
            RollOp::TakeHigh(n) => return RollOp::take_end(results, *n, true),
            RollOp::TakeLow(n) => return RollOp::take_end(results, *n, false),
            RollOp::TakeMid(n) => return RollOp::take_mid(results, *n),
            RollOp::AddEach(n) => return RollOp::add_each(results, *n),
            RollOp::SubEach(n) => return RollOp::add_each(results, (*n) * -1),
            RollOp::Advantage(n) => return RollOp::advantage(results, *n),
            RollOp::Disadvantage(n) => return RollOp::disadvantage(results, *n),
            RollOp::Crit(n) => return RollOp::apply_crit(results, *n),
        }
    }

    fn take_end(results: &mut Results, amt: i64, low: bool) {
        let len: usize = results.rolls.len();
        let max: usize = if amt < 0 {
            len + amt as usize
        } else {
            amt as usize
        };

        results.rolls.sort_by(|a, b| b.total.cmp(&a.total));
        for idx in 0..len {
            if low {
                results.rolls[idx].keep = idx < max;
            } else {
                results.rolls[idx].keep = idx >= (len - max);
            }
        }
    }

    fn take_mid(results: &mut Results, amt: i64) {
        let len: usize = results.rolls.len();
        let max: usize = if amt < 0 {
            len + amt as usize
        } else {
            amt as usize
        };

        results.rolls.sort_by(|a, b| b.total.cmp(&a.total));
        let skip_start = (len - max) / 2;
        let skip_end = skip_start + max;
        for idx in 0..len {
            results.rolls[idx].keep = idx >= skip_start && idx < skip_end;
        }
    }

    fn explode(results: &mut Results, amt: i64, until: bool) {
        let mut rolls: Vec<Roll> = Vec::new();
        let mut range = 0;
        for r in results.rolls.iter() {
            rolls.push(r.clone());
            if r.keep {
                if (amt < 0 && r.roll <= r.range + amt) || (r.roll < amt) {
                    return
                }

                // NOTE: if the source dice are not all the same, the last
                // die becomes the range of the bonus rolls.
                range = r.range
            }
        }

        loop {
            let mut r = Roll::new(range, 0);
            r.bonus = true;
            rolls.push(r);
            if !until || r.roll < r.range {
                break
            }
        }

        results.rolls = rolls;
    }

    fn explode_each(results: &mut Results, amt: i64, until: bool) {
        let mut rolls: Vec<Roll> = Vec::new();
        for r in results.rolls.iter() {
            rolls.push(r.clone());
            if r.keep {
                if (amt < 0 && r.roll > r.range + amt) || (amt > 0 && r.roll >= amt) {
                    loop {
                        let mut bonus = Roll::new(r.range, 0);
                        bonus.bonus = true;
                        rolls.push(bonus);
                        if !until || (amt < 0 && bonus.roll <= bonus.range + amt) || (amt > 0 && bonus.roll < amt) { 
                            break;
                        }
                    }
                }
            }
        }

        results.rolls = rolls;
    }

    fn add_each(results: &mut Results, amt: i64) {
        let mut rolls: Vec<Roll> = Vec::new();
        let amt = if amt < 0 {
            1
        } else {
            amt
        };

        for r in results.rolls.iter() {
            if r.keep {
                rolls.push(Roll{
                    modifier: r.modifier + amt, 
                    total: r.total + amt,
                    ..*r });
            } else {
                rolls.push(r.clone());
            }
        }
       
        results.rolls = rolls;
    }

    // Apply Crit is the simplest operator; iterate through the rolls
    // and mark all those that are above a thresold as "critical"
    fn apply_crit(results: &mut Results, amt: i64) {
        let mut rolls: Vec<Roll> = Vec::new();
        for r in results.rolls.iter() {
            if amt < 0 {
                rolls.push(Roll{ crit: r.keep && r.roll > r.roll + amt, ..*r })
            } else {
                rolls.push(Roll{ crit: r.keep && r.roll >= amt, ..*r })
            }
        }
    }

    // Advantage: roll `amt` additional dice for each die in the current
    // results keeping the highest alternate roll.
    fn advantage(results: &mut Results, amt: i64) {
        let mut rolls: Vec<Roll> = Vec::new();
        let amt = if amt < 0 {
            1
        } else {
            amt
        };

        for r in results.rolls.iter() {
            if r.keep {
                let mut tmp: Vec<Roll> = Vec::new();
                let mut high = 0;
                tmp.push(r.clone());
                for idx in 1..=amt as usize {
                    let mut reroll = Roll::new(r.range, r.modifier);
                    if reroll.roll > tmp[high].roll  {
                        tmp[high].keep = false;
                        high = idx;
                    } else {
                        reroll.keep = false;
                    }
                    tmp.push(reroll);
                }

                for r2 in tmp {
                    rolls.push(r2);
                }
            } else {
                rolls.push(r.clone());
            }
        }
        
        results.rolls = rolls;
    }

    // Disadvantage: same as advantage except the lowest alternate roll is
    // kept.
    fn disadvantage(results: &mut Results, amt: i64) {
        let mut rolls: Vec<Roll> = Vec::new();
        let amt = if amt < 0 {
            1
        } else {
            amt
        };

        for r in results.rolls.iter() {
            if r.keep {
                let mut tmp: Vec<Roll> = Vec::new();
                let mut lo = 0;
                tmp.push(r.clone());
                for idx in 1..=amt as usize {
                    let mut reroll = Roll::new(r.range, r.modifier);
                    if reroll.roll < tmp[lo].roll  {
                        tmp[lo].keep = false;
                        lo = idx;
                    } else {
                        reroll.keep = false;
                    }
                    tmp.push(reroll);
                }

                for r2 in tmp {
                    rolls.push(r2);
                }
            } else {
                rolls.push(r.clone());
            }
        }
        
        results.rolls = rolls;
    }    
}

impl fmt::Display for RollOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollOp::Explode(n) => if *n < 0 {
                    write!(f, "!")
                } else {
                    write!(f, "!{}", n)
                },
            RollOp::ExplodeUntil(n) => if *n < 0 {
                    write!(f, "!!")
                } else {
                    write!(f, "!!{}", n)
                },
            RollOp::ExplodeEach(n) => if *n < 0 {
                    write!(f, "*")
                } else {
                    write!(f, "*{}", n)
                },
            RollOp::ExplodeEachUntil(n) => if *n < 0 {
                    write!(f, "**")
                } else {
                    write!(f, "**{}", n)
                },
            RollOp::TakeHigh(n) => if *n < 0 {
                    write!(f, "^")
                } else {
                    write!(f, "^{}", n)
                },
            RollOp::TakeLow(n) => if *n < 0 {
                    write!(f, "`")
                } else {
                    write!(f, "`{}", n)
                },
            RollOp::TakeMid(n) => if *n < 0 {
                    write!(f, "~")
                } else {
                    write!(f, "~{}", n)
                },
            RollOp::AddEach(n) => if *n < 0 {
                    write!(f, "++")
                } else {
                    write!(f, "++{}", n)
                },
            RollOp::SubEach(n) => if *n < 0 {
                    write!(f, "--")
                } else {
                    write!(f, "--{}", n)
                },
            RollOp::Advantage(n) => if *n < 0 {
                    write!(f, "ADV")
                } else {
                    write!(f, "ADV {}", n)
                },
            RollOp::Disadvantage(n) => if *n < 0 {
                    write!(f, "DIS")
                } else {
                    write!(f, "DIS {}", n)
                },
            RollOp::Crit(n) => if *n < 0 {
                    write!(f, "$")
                } else {
                    write!(f, "${}", n)
                },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_op() {
        assert_eq!(RollOp::build(&vec![Token::Op('!')], 0), Some((RollOp::Explode(-1), 1)));
        assert_eq!(RollOp::build(&vec![Token::Op2('+'), Token::Num(4 as i64)], 0), Some((RollOp::AddEach(4), 2)));
    }

    #[test]
    fn add_each() {
        let mut results = Dice{ count: 2, range: 8, ops: vec![], modifier: 0 }.roll();
        let total = results.total();
        RollOp::add_each(&mut results, 5);
        assert_eq!(results.total(), total + 10);
        for r in results.rolls { 
            assert_eq!(r.modifier, 5);
        }
    }

    #[test]
    fn advantage() {
        let mut results = Dice::new(1, 20, 0).roll();
        let old_total = results.total();
        assert_eq!(results.rolls.len(), 1);

        RollOp::advantage(&mut results, 1);
        assert_eq!(results.rolls.len(), 2);
        assert!(results.total() >= old_total);
        if results.rolls[0].keep {
            assert!(results.rolls[0].roll >= results.rolls[1].roll);
            assert!(results.rolls[0].keep);
            assert!(!results.rolls[1].keep);
            assert_eq!(results.total(), results.rolls[0].total);
        } else {
            assert!(results.rolls[0].roll <= results.rolls[1].roll);
            assert!(results.rolls[1].keep);
            assert!(!results.rolls[0].keep);
            assert_eq!(results.total(), results.rolls[1].total);
        }
    }

    #[test]
    fn disadvantage() {
        let mut results = Dice::new(1, 20, 0).roll();
        assert_eq!(results.rolls.len(), 1);

        let old_total = results.total();
        assert!(old_total > 0);

        RollOp::disadvantage(&mut results, 1);
        assert!(results.total() <= old_total);
    }
}
