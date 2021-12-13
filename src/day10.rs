pub fn run() {
  let mut corrupted_chars = vec![];
  let mut unclosed_brackets = vec![];

  for line in include_str!("inputs/day10").lines() {
    match analyze_line(line) {
      Ok(Incomplete { unclosed }) => unclosed_brackets.push(unclosed),
      Err(Corrupted(ch)) => corrupted_chars.push(ch),
    }
  }

  let syntax_error_score: i32 = corrupted_chars
    .iter()
    .map(|ch| match ch {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      _ => unreachable!(),
    })
    .sum();
  println!("{}", syntax_error_score);

  let mut autocomplete_scores: Vec<i64> = unclosed_brackets
    .iter()
    .map(|unclosed| {
      unclosed
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

struct Corrupted(char);
struct Incomplete {
  unclosed: Vec<char>,
}

fn analyze_line(line: &str) -> Result<Incomplete, Corrupted> {
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
          None => return Err(Corrupted(ch)),
          Some(&unclosed_ch) => {
            if unclosed_ch == open_char {
              unclosed_brackets.pop();
            } else {
              return Err(Corrupted(ch));
            }
          }
        }
      }
      _ => unreachable!(),
    }
  }
  Ok(Incomplete {
    unclosed: unclosed_brackets,
  })
}
