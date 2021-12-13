pub fn run() {
  let lines: Vec<&str> = include_str!("inputs/day10").lines().collect();
  let total_score: i32 = lines
    .iter()
    .filter_map(|l| get_corrupted_char(l))
    .map(|ch| match ch {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      _ => unreachable!(),
    })
    .sum();
  println!("{}", total_score);

  let mut autocomplete_scores: Vec<i64> = lines
    .iter()
    .filter_map(|l| get_unclosed_brackets(l))
    .map(|unclosed_brackets| {
      unclosed_brackets
        .iter()
        .rev()
        .map(|ch| match ch {
          '(' => 1,
          '[' => 2,
          '{' => 3,
          '<' => 4,
          _ => unreachable!(),
        })
        .fold(0, |acc, val| acc * 5 + val)
    })
    .collect();
  autocomplete_scores.sort();
  println!("{:?}", autocomplete_scores[autocomplete_scores.len() / 2]);
}

fn get_corrupted_char(line: &str) -> Option<char> {
  let mut unclosed_brackets = vec![];
  for ch in line.chars() {
    match ch {
      '(' | '[' | '{' | '<' => unclosed_brackets.push(ch),
      ')' | ']' | '}' | '>' => {
        let open_char = match ch {
          ')' => '(',
          ']' => '[',
          '}' => '{',
          '>' => '<',
          _ => unreachable!(),
        };
        match unclosed_brackets.pop() {
          None => return Some(ch),
          Some(unclosed_ch) => {
            if unclosed_ch != open_char {
              return Some(ch);
            }
          }
        }
      }
      _ => unreachable!(),
    }
  }
  None
}

fn get_unclosed_brackets(line: &str) -> Option<Vec<char>> {
  let mut unclosed_brackets = vec![];
  for ch in line.chars() {
    match ch {
      '(' | '[' | '{' | '<' => unclosed_brackets.push(ch),
      ')' | ']' | '}' | '>' => {
        let open_char = match ch {
          ')' => '(',
          ']' => '[',
          '}' => '{',
          '>' => '<',
          _ => unreachable!(),
        };
        match unclosed_brackets.last() {
          None => return None,
          Some(&unclosed_ch) => {
            if unclosed_ch == open_char {
              unclosed_brackets.pop();
            } else {
              return None;
            }
          }
        }
      }
      _ => unreachable!(),
    }
  }
  Some(unclosed_brackets)
}
