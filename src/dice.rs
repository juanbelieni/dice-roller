use rand::{self, Rng};
use std::fmt;

pub struct Dice {
  pub op: String,
  pub mult: u64,
  pub sides: u64,
}

impl fmt::Display for Dice {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}{}d{}", self.op, self.mult, self.sides)
  }
}

impl Dice {
  pub fn roll(&self) -> i128 {
      let mut rng = rand::thread_rng();
      let mut sum: i128 = 0;

      for _ in 0..self.mult {
          let roll = rng.gen_range(1, self.sides + 1) as i128;
          sum += roll;
      }

      if self.op == "+".to_string() {
          return sum;
      } else {
          return sum * -1;
      }
  }
}