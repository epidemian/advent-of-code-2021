pub fn run() {
  part1();
  part2();
}

fn part1() {
  let mut pos = 0;
  let mut depth = 0;
  for (command, value) in parse_commands() {
    match command {
      "forward" => pos += value,
      "down" => depth += value,
      "up" => depth -= value,
      _ => unreachable!(),
    }
  }
  println!("{}", pos * depth)
}

fn part2() {
  let mut pos = 0;
  let mut depth = 0;
  let mut aim = 0;
  for (command, value) in parse_commands() {
    match command {
      "forward" => {
        pos += value;
        depth += aim * value
      }
      "down" => aim += value,
      "up" => aim -= value,
      _ => unreachable!(),
    }
  }
  println!("{}", pos * depth)
}

fn parse_commands() -> Vec<(&'static str, i32)> {
  include_str!("inputs/day02")
    .lines()
    .map(|line: &str| {
      let (command, value) = line.split_once(" ").unwrap();
      (command, value.parse().unwrap())
    })
    .collect()
}
