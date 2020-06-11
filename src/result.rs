use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Roll {
    pub range: i64,
    pub roll: i64,
    pub modifier: i64,
    pub total: i64,
    pub keep: bool,
    pub crit: bool,
    pub bonus: bool,
}

impl Roll {
    pub fn new(range: i64, modifier: i64) -> Roll {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1, range + 1) as i64;
        Roll{ range: range, roll: roll, modifier: modifier, total: roll + modifier, keep: true, crit: false, bonus: false }
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d{}", self.range)?;
        if self.modifier > 0 {
            write!(f, "+{}", self.modifier)?;
        } else if self.modifier < 0 {
            write!(f, "{}", self.modifier)?;
        }

        if self.bonus {
            write!(f, " BNS")?;
        }

        if self.crit {
            write!(f, " CRIT")?;
        }

        if self.keep {
            write!(f, " [{}]", self.total)
        } else {
            write!(f, " ({})", self.total)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Results {
    pub rolls: Vec<Roll>,
    pub modifier: i64,
}

impl Results {
    pub fn calc_total(rolls: &Vec<Roll>) -> i64 {
        rolls.iter().filter(|r| r.keep ).map(|r| r.total).sum()
    }

    pub fn total(&self) -> i64 {
        Results::calc_total(&self.rolls) + self.modifier
    }

    pub fn keep(&self) -> usize {
        self.rolls.iter().filter(|r| r.keep ).map(|_| 1 ).sum()
    }

    pub fn discard(&self) -> usize {
        self.rolls.iter().filter(|r| !r.keep ).map(|_| 1 ).sum()
    }

    pub fn crit(&self) -> usize {
        self.rolls.iter().filter(|r| r.keep && r.crit ).map(|_| 1 ).sum()
    }

    pub fn bonus(&self) -> usize {
        self.rolls.iter().filter(|r| r.keep && r.bonus ).map(|_| 1 ).sum()
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.rolls.len() {
            if idx > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", self.rolls[idx])?;
        }

        if self.modifier > 0 {
            write!(f, " + {}", self.modifier)?;
        } else if self.modifier < 0 {
            write!(f, " - {}", self.modifier.abs())?;
        }

        write!(f, " = {}", self.total())
    }
}
