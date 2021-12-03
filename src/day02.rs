pub fn run() {
  part1();
  part2();
}

fn part1() {
  let input = include_str!("inputs/day02");
  let mut pos = 0;
  let mut depth = 0;
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();
    let value: i32 = parts.next().unwrap().parse().unwrap();
    match command {
      "forward" => pos += value,
      "down" => depth += value,
      "up" => depth -= value,
      _ => println!("unknown command {}", command),
    }
  }
  println!("{}", pos * depth)
}

fn part2() {
  let input = include_str!("inputs/day02");
  let mut pos = 0;
  let mut depth = 0;
  let mut aim = 0;
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();
    let value: i32 = parts.next().unwrap().parse().unwrap();
    match command {
      "forward" => {
        pos += value;
        depth += aim * value
      }
      "down" => aim += value,
      "up" => aim -= value,
      _ => println!("unknown command {}", command),
    }
  }
  println!("{}", pos * depth)
}
