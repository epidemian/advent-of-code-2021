use serde_json::Value;
use std::fmt;
use std::ops;

pub fn run() {
  let snailfish_nums: Vec<SnailfishNumber> = include_str!("inputs/day18")
    .lines()
    .map(|l| SnailfishNumber::from_str(l))
    .collect();

  let mut sum = snailfish_nums[0].clone();
  for num in &snailfish_nums[1..] {
    sum = sum.add(num);
  }

  println!("{}", sum.magnitude());

  let mut max_magnitude = 0;
  for n1 in &snailfish_nums {
    for n2 in &snailfish_nums {
      max_magnitude = max_magnitude.max(n1.add(n2).magnitude())
    }
  }
  println!("{}", max_magnitude);
}

#[derive(Debug, Clone)]
enum SnailfishNumber {
  Number(u32),
  Pair((Box<SnailfishNumber>, Box<SnailfishNumber>)),
}

impl SnailfishNumber {
  fn from_str(s: &str) -> SnailfishNumber {
    Self::from_json(&serde_json::from_str(s).unwrap())
  }

  fn from_json(json: &Value) -> SnailfishNumber {
    match json {
      Value::Number(n) => SnailfishNumber::Number(n.as_u64().unwrap() as u32),
      Value::Array(pair) => {
        let left = Self::from_json(&pair[0]);
        let right = Self::from_json(&pair[1]);
        SnailfishNumber::Pair((Box::new(left), Box::new(right)))
      }
      _ => unreachable!(),
    }
  }

  fn add(&self, rhs: &SnailfishNumber) -> SnailfishNumber {
    SnailfishNumber::Pair((Box::new(self.clone()), Box::new(rhs.clone()))).reduce()
  }

  fn reduce(&self) -> SnailfishNumber {
    let mut result = self.clone();
    loop {
      if let Some(explode_result) = result.try_explode(0) {
        result = explode_result.result;
        continue;
      }
      if let Some(split_result) = result.try_split() {
        result = split_result;
        continue;
      }
      break;
    }
    result
  }

  fn try_explode(&self, depth: usize) -> Option<ExplodeResult> {
    match self {
      SnailfishNumber::Number(_) => None,
      SnailfishNumber::Pair((left, right)) => {
        if depth == 4 {
          if let (SnailfishNumber::Number(l), SnailfishNumber::Number(r)) =
            (left.as_ref(), right.as_ref())
          {
            return Some(ExplodeResult {
              result: SnailfishNumber::Number(0),
              left: Some(*l),
              right: Some(*r),
            });
          }
        }

        if let Some(explode_result) = left.try_explode(depth + 1) {
          let new_right = if let Some(right_overflow) = explode_result.right {
            right.add_to_left(right_overflow)
          } else {
            right.as_ref().clone()
          };
          Some(ExplodeResult {
            result: SnailfishNumber::Pair((Box::new(explode_result.result), Box::new(new_right))),
            left: explode_result.left,
            right: None,
          })
        } else if let Some(explode_result) = right.try_explode(depth + 1) {
          let new_left = if let Some(left_overflow) = explode_result.left {
            left.add_to_right(left_overflow)
          } else {
            left.as_ref().clone()
          };
          Some(ExplodeResult {
            result: SnailfishNumber::Pair((Box::new(new_left), Box::new(explode_result.result))),
            left: None,
            right: explode_result.right,
          })
        } else {
          None
        }
      }
    }
  }

  fn add_to_left(&self, n: u32) -> SnailfishNumber {
    match self {
      SnailfishNumber::Number(n2) => SnailfishNumber::Number(n + n2),
      SnailfishNumber::Pair((left, right)) => SnailfishNumber::Pair((
        Box::new(left.add_to_left(n)),
        Box::new(right.as_ref().clone()),
      )),
    }
  }

  fn add_to_right(&self, n: u32) -> SnailfishNumber {
    match self {
      SnailfishNumber::Number(n2) => SnailfishNumber::Number(n + n2),
      SnailfishNumber::Pair((left, right)) => SnailfishNumber::Pair((
        Box::new(left.as_ref().clone()),
        Box::new(right.add_to_right(n)),
      )),
    }
  }

  fn try_split(&self) -> Option<SnailfishNumber> {
    match self {
      SnailfishNumber::Number(n) => {
        if *n > 9 {
          let left = SnailfishNumber::Number(n / 2);
          let right = SnailfishNumber::Number((n + 1) / 2);
          Some(SnailfishNumber::Pair((Box::new(left), Box::new(right))))
        } else {
          None
        }
      }
      SnailfishNumber::Pair((left, right)) => {
        if let Some(split_result) = left.try_split() {
          Some(SnailfishNumber::Pair((
            Box::new(split_result),
            Box::new(right.as_ref().clone()),
          )))
        } else if let Some(split_result) = right.try_split() {
          Some(SnailfishNumber::Pair((
            Box::new(left.as_ref().clone()),
            Box::new(split_result),
          )))
        } else {
          None
        }
      }
    }
  }

  fn magnitude(&self) -> u32 {
    match self {
      SnailfishNumber::Number(n) => *n,
      SnailfishNumber::Pair((left, right)) => 3 * left.magnitude() + 2 * right.magnitude(),
    }
  }
}

impl ops::Add<SnailfishNumber> for SnailfishNumber {
  type Output = SnailfishNumber;
  fn add(self, rhs: SnailfishNumber) -> SnailfishNumber {
    SnailfishNumber::Pair((Box::new(self), Box::new(rhs))).reduce()
  }
}

impl fmt::Display for SnailfishNumber {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SnailfishNumber::Number(n) => write!(f, "{}", n),
      SnailfishNumber::Pair((l, r)) => write!(f, "[{},{}]", l, r),
    }
  }
}

struct ExplodeResult {
  result: SnailfishNumber,
  left: Option<u32>,
  right: Option<u32>,
}
