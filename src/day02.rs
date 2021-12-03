pub fn run() {
  part1();
  part2();
}

fn part1() {
  let mut pos = 0;
  let mut depth = 0;
  for (command, value) in parse_commands() {
    match command {
      Command::Forward => pos += value,
      Command::Down => depth += value,
      Command::Up => depth -= value,
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
      Command::Forward => {
        pos += value;
        depth += aim * value
      }
      Command::Down => aim += value,
      Command::Up => aim -= value,
    }
  }
  println!("{}", pos * depth)
}

enum Command {
  Forward,
  Down,
  Up,
}

fn parse_commands() -> Vec<(Command, i32)> {
  include_str!("inputs/day02")
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();
      let command = parts.next().unwrap();
      let value: i32 = parts.next().unwrap().parse().unwrap();
      match command {
        "forward" => (Command::Forward, value),
        "down" => (Command::Down, value),
        "up" => (Command::Up, value),
        _ => panic!("unknown command {}", command),
      }
    })
    .collect()
}
