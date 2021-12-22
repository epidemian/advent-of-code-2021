pub fn run() {
  let snailfish_nums: Vec<_> = include_str!("inputs/day18")
    .lines()
    .map(parse_snailfish_number)
    .collect();

  let mut sum = snailfish_nums[0].clone();
  for num in &snailfish_nums[1..] {
    sum = add(&sum, num);
  }
  println!("{}", magnitude(&sum));

  let mut max_magnitude = 0;
  for n1 in snailfish_nums.iter() {
    for n2 in snailfish_nums.iter() {
      max_magnitude = max_magnitude.max(magnitude(&add(n1, n2)))
    }
  }
  println!("{}", max_magnitude);
}

// Instead of modelling snailfish numbers as tree-like structures, just
// represent them as a list of tokens and then manipulate them symbolically.
type SnailfishTokens = Vec<Token>;

#[derive(Debug, Clone, Copy)]
enum Token {
  LParen,
  RParen,
  Number(u32),
}

fn parse_snailfish_number(s: &str) -> SnailfishTokens {
  s.chars()
    .filter_map(|ch| match ch {
      '[' => Some(Token::LParen),
      ']' => Some(Token::RParen),
      '0'..='9' => Some(Token::Number(ch.to_digit(10).unwrap())),
      _ => None,
    })
    .collect()
}

fn add(lhs: &SnailfishTokens, rhs: &SnailfishTokens) -> SnailfishTokens {
  let mut result = Vec::with_capacity(lhs.len() + rhs.len() + 2);
  result.push(Token::LParen);
  result.extend(lhs);
  result.extend(rhs);
  result.push(Token::RParen);

  reduce(&mut result);
  result
}

fn reduce(num: &mut SnailfishTokens) {
  loop {
    if explode(num) {
      continue;
    }
    if split(num) {
      continue;
    }
    break;
  }
}

fn explode(tokens: &mut SnailfishTokens) -> bool {
  let mut explode_index = None;
  let mut depth = 0;
  for (i, token) in tokens.iter().enumerate() {
    match token {
      Token::LParen => {
        if depth == 4 {
          explode_index = Some(i);
          break;
        }
        depth += 1;
      }
      Token::RParen => depth -= 1,
      _ => {}
    }
  }
  if let Some(index) = explode_index {
    match &tokens[index..index + 4] {
      &[Token::LParen, Token::Number(exploded_l), Token::Number(exploded_r), Token::RParen] => {
        tokens.splice(index..index + 4, [Token::Number(0)]);
        for i in (0..index).rev() {
          if let Token::Number(first_num_left) = tokens[i] {
            tokens[i] = Token::Number(first_num_left + exploded_l);
            break;
          }
        }
        for i in index + 1..tokens.len() {
          if let Token::Number(first_num_right) = tokens[i] {
            tokens[i] = Token::Number(first_num_right + exploded_r);
            break;
          }
        }
      }
      unexpected => panic!("unexpected pattern on explosion {:?}", unexpected),
    }
    return true;
  }
  false
}

fn split(tokens: &mut SnailfishTokens) -> bool {
  for (index, token) in tokens.iter().enumerate() {
    match token {
      &Token::Number(n) if n > 9 => {
        tokens.splice(
          index..=index,
          [
            Token::LParen,
            Token::Number(n / 2),
            Token::Number((n + 1) / 2),
            Token::RParen,
          ],
        );
        return true;
      }
      _ => {}
    }
  }
  false
}

fn magnitude(tokens: &SnailfishTokens) -> u32 {
  // Keep a stack of magnitudes to the left of the current token. Whenever a
  // pair closes, pop the last two magnitudes and compute the pair's magnitude.
  let mut stack = vec![];
  for token in tokens {
    match token {
      Token::Number(n) => stack.push(*n),
      Token::RParen => {
        let r = stack.pop().unwrap();
        let l = stack.pop().unwrap();
        stack.push(3 * l + 2 * r);
      }
      _ => {}
    }
  }
  assert_eq!(stack.len(), 1);
  stack[0]
}
