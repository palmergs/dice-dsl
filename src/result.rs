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
