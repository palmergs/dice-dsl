use rand::Rng;

use super::Die;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Results {
    pub rolls: Vec<Roll>,
    pub total: i64,
}

impl Results {
    pub fn total(rolls: &Vec<Roll>) -> i64 {
        rolls.iter().filter(|r| r.keep ).map(|r| r.total).sum()
    }

    pub fn calc_total(&mut self) -> i64 {
        self.total = Results::total(&self.rolls);
        self.total
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
