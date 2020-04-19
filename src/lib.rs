mod tokenizer;
pub use tokenizer::{ tokenize, Token };

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Roll {
  // range of the die; the 20 in d20
  pub range: u64,

  // the integer modifier for this roll; the +3 in d6+3
  pub modifier: i64,

  // the value rolled; 0 until the die is rolled
  pub value: i64,

  // true if this role was due to an "explode" c
  pub exploded: bool,

  // true if this die is to be counted in a larger result
  pub kept: bool,

  // true if this roll had a target number and was successful
  pub success: bool,
}

#[derive(Debug)]
pub struct Result {
  pub count: u64,
  pub range: u64,
  pub op: Option<tokenizer::Token>,
  pub mod_or_tgt: i64
}

impl Result {
  pub fn build(tokens: &Vec<tokenizer::Token>, idx: usize) -> (Result, usize) {
    let mut result = Result{ count: 0, range: 0, op: None, mod_or_tgt: 0 };
    let idx = Result::build_result(&mut result, tokens, idx);
    return (result, idx)
  }

  pub fn roll(&mut self) -> Vec<Roll> {
    let mut rolls: Vec<Roll> = Vec::new();
    for _ in 0..self.count {
      let mut roll = self.roll_one();
    }
    return rolls
  }

  pub fn roll_one(&self) -> Roll {
    let mut rng = rand::thread_rng();
    let mut value = rng.gen_range(1, self.range + 1) as i64;
    let mut modifier: i64 = 0;
    match self.op {
      Some(t) => {
        match t {
          Token::PlusEach => modifier = self.mod_or_tgt,
          Token::MinusEach => modifier = -1 * self.mod_or_tgt,
          _ => ()
        }
      }
      None => ()
    }
    return Roll{
      range: self.range,
      value: value + modifier,
      modifier: modifier,
      exploded: false,
      success: true,
      kept: true
    }
  }

  fn build_result(result: &mut Result, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
    let idx = Result::build_count(result, tokens, idx);
    let idx = idx + 1; // skil over Token::D
    let idx = Result::build_range(result, tokens, idx);
    let idx = Result::build_op(result, tokens, idx);
    let idx = Result::build_mod(result, tokens, idx);
    return idx
  }

  fn build_count(result: &mut Result, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
    if idx >= tokens.len() { return idx }

    let token = &tokens[idx];
    match token {
      Token::D => {
        result.count = 1;
        return idx + 1;
      },
      Token::Num(n) => result.count = *n,
      _ => ()
    }
    return idx
  }

  fn build_range(result: &mut Result, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
    if idx >= tokens.len() { return idx }

    let token = &tokens[idx];
    match token {
      Token::Num(n) => {
        result.range = *n;
        return idx + 1
      },
      _ => ()
    }
    return idx
  }
  
  fn build_op(result: &mut Result, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
    if idx >= tokens.len() { return idx }

    let token = &tokens[idx];
    match token {
      Token::Plus | Token::PlusEach | 
      Token::Minus | Token::MinusEach |
      Token::Explode |  Token::ExplodeEach |
      Token::TakeHigh | Token::TakeMiddle | Token::TakeLow => {
        result.op = Some(*token);
        return idx + 1
      },
      _ => ()
    }

    return idx
  }
  
  fn build_mod(result: &mut Result, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
    if idx >= tokens.len() { return idx }

    let token = &tokens[idx];
    match token {
      Token::Num(n) => {
        result.range = *n;
        return idx + 1
      },
      _ => ()
    }
    return idx
  }  
}

#[derive(Debug)]
pub struct ResultList {
  pub results: Vec<Result>,
}

impl ResultList {
  pub fn build(tokens: &Vec<tokenizer::Token>) -> (ResultList, usize) {
    let mut list = ResultList{ results: Vec::new() };
    let mut idx = 0;
    while idx < tokens.len() {
      let (result, new_idx) = Result::build(tokens, idx);
      list.results.push(result);
      match tokens[idx] {
        Token::Comma => {
          idx = new_idx + 1;
        },
        _ => {
          return (list, new_idx)
        }
      }

    }
    return (list, idx)
  }

  pub fn roll(&self) -> Vec<Roll> {
    return Vec::new()
  }  
}