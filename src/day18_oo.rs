use serde_json::Value;
use std::fmt;
use std::rc::Rc;

// This day 18 solution uses a quite OO approach to modeling and delegation.
// The main problem of it is that for proper memory management, it's either
// necessary use `Box`es everywhere and constantly copy data for each boxed
// pointer to own a single object; or use reference counting and avoid
// unnecessary copies.
pub fn run() {
  let snailfish_nums: Vec<Rc<SnailfishNumber>> = include_str!("inputs/day18")
    .lines()
    .map(|l| Rc::new(SnailfishNumber::from_str(l)))
    .collect();

  let mut sum = snailfish_nums[0].clone();
  for num in &snailfish_nums[1..] {
    sum = add(sum, num.clone());
  }

  println!("{}", sum.magnitude());

  let mut max_magnitude = 0;
  for n1 in snailfish_nums.iter() {
    for n2 in snailfish_nums.iter() {
      max_magnitude = max_magnitude.max(add(n1.clone(), n2.clone()).magnitude())
    }
  }
  println!("{}", max_magnitude);
}

fn add(lhs: Rc<SnailfishNumber>, rhs: Rc<SnailfishNumber>) -> Rc<SnailfishNumber> {
  reduce(Rc::new(SnailfishNumber::Pair((lhs, rhs))))
}

fn reduce(num: Rc<SnailfishNumber>) -> Rc<SnailfishNumber> {
  let mut result = num;
  loop {
    if let Some(explode_result) = result.try_explode(0) {
      result = Rc::new(explode_result.result);
      continue;
    }
    if let Some(split_result) = result.try_split() {
      result = Rc::new(split_result);
      continue;
    }
    break;
  }
  result
}

#[derive(Debug)]
enum SnailfishNumber {
  Number(u32),
  Pair((Rc<SnailfishNumber>, Rc<SnailfishNumber>)),
}

struct ExplodeResult {
  result: SnailfishNumber,
  left: Option<u32>,
  right: Option<u32>,
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
        SnailfishNumber::Pair((Rc::new(left), Rc::new(right)))
      }
      _ => unreachable!(),
    }
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
            right.clone()
          };
          Some(ExplodeResult {
            result: SnailfishNumber::Pair((Rc::new(explode_result.result), new_right)),
            left: explode_result.left,
            right: None,
          })
        } else if let Some(explode_result) = right.try_explode(depth + 1) {
          let new_left = if let Some(left_overflow) = explode_result.left {
            left.add_to_right(left_overflow)
          } else {
            left.clone()
          };
          Some(ExplodeResult {
            result: SnailfishNumber::Pair((new_left, Rc::new(explode_result.result))),
            left: None,
            right: explode_result.right,
          })
        } else {
          None
        }
      }
    }
  }

  fn add_to_left(&self, n: u32) -> Rc<SnailfishNumber> {
    Rc::new(match self {
      SnailfishNumber::Number(n2) => (SnailfishNumber::Number(n + n2)),
      SnailfishNumber::Pair((left, right)) => {
        SnailfishNumber::Pair((left.add_to_left(n), right.clone()))
      }
    })
  }

  fn add_to_right(&self, n: u32) -> Rc<SnailfishNumber> {
    Rc::new(match self {
      SnailfishNumber::Number(n2) => SnailfishNumber::Number(n + n2),
      SnailfishNumber::Pair((left, right)) => {
        SnailfishNumber::Pair((left.clone(), right.add_to_right(n)))
      }
    })
  }

  fn try_split(&self) -> Option<SnailfishNumber> {
    match self {
      SnailfishNumber::Number(n) => {
        if *n > 9 {
          let left = SnailfishNumber::Number(n / 2);
          let right = SnailfishNumber::Number((n + 1) / 2);
          Some(SnailfishNumber::Pair((Rc::new(left), Rc::new(right))))
        } else {
          None
        }
      }
      SnailfishNumber::Pair((left, right)) => {
        if let Some(split_result) = left.try_split() {
          Some(SnailfishNumber::Pair((
            Rc::new(split_result),
            right.clone(),
          )))
        } else if let Some(split_result) = right.try_split() {
          Some(SnailfishNumber::Pair((left.clone(), Rc::new(split_result))))
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

impl fmt::Display for SnailfishNumber {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SnailfishNumber::Number(n) => write!(f, "{}", n),
      SnailfishNumber::Pair((l, r)) => write!(f, "[{},{}]", l, r),
    }
  }
}
