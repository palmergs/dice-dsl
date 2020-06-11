mod tokenizer;
pub use tokenizer::{tokenize, tokens, Token};

mod ops;
pub use ops::{ RollOp };

mod generator;
pub use generator::{ Dice };

mod result;
pub use result::{ Roll, Results };

use std::i64::MAX;
use std::collections::BTreeMap;

pub fn dice(input: &str) -> Option<Dice> {
    let tokens = tokens(input);
    match Dice::build(&tokens, 0) {
        Some((dice, _)) => Some(dice),
        None => None,
    }
}

pub fn chart(dice: &Dice, num: usize) {
    let mut min: i64 = MAX;
    let mut max: i64 = 0;
    let mut max_cnt: usize = 0;
    let mut map: BTreeMap<i64, usize> = BTreeMap::new(); 
    for _ in 0..num {
        let results = dice.roll();
        let v = results.total();
        if v < min { min = v; }
        if v > max { max = v; }
        match map.get(&v) {
            Some(n) => {
                let cnt = n + 1;
                if cnt > max_cnt { max_cnt = cnt; }
                map.insert(v, cnt);
            }
            None    => {
                map.insert(v, 1);
            }
        }
    }

    let mut cnt = num as f64;
    for k in min..=max {
        match map.get(&k) {
            Some(n) => {
                let width = max_cnt / 50;
                print!("{:>3}. {:>5.*}: ", k, 1, (cnt / num as f64) * 100.0);
                for _ in 0..=(n / width) { print!("*"); }
                println!("");
                cnt -= *n as f64;
            }
            None    => {
                println!("{:>3}. {:>5.*}:", k, 1, 0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_dice() {
        let roller = dice(&"4d4+4").unwrap();
        let results = roller.roll();
        assert!(results.total() >= 8);
    }
}
