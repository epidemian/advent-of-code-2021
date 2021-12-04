pub fn run() {
  let input = include_str!("inputs/day03");
  let mut counts = vec![0; input.lines().next().unwrap().len()];
  for line in input.lines() {
    for (i, ch) in line.chars().enumerate() {
      counts[i] += if ch == '1' { 1 } else { -1 }
    }
  }
  let gamma = counts.iter().fold(0, |acc, &c| (acc << 1) | (c > 0) as i32);
  let epsilon = !gamma & (1 << counts.len()) - 1;
  println!("{}", gamma * epsilon)
}
