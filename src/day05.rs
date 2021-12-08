use std::collections::HashMap;

pub fn run() {
  let all_lines: Vec<Line> = include_str!("inputs/day05")
    .lines()
    .map(parse_line)
    .collect();
  let non_diagonal_lines: Vec<Line> = all_lines
    .iter()
    .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
    .map(|l| l.clone())
    .collect();

  println!("{}", count_overlaps(&non_diagonal_lines));
  println!("{}", count_overlaps(&all_lines));
}

fn count_overlaps(lines: &[Line]) -> usize {
  let mut counts = HashMap::new();
  for &(x1, y1, x2, y2) in lines {
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    let mut x = x1;
    let mut y = y1;
    *counts.entry((x, y)).or_insert(0) += 1;
    while !(x == x2 && y == y2) {
      x += dx;
      y += dy;
      *counts.entry((x, y)).or_insert(0) += 1
    }
  }
  counts.values().filter(|count| **count >= 2).count()
}

type Line = (i32, i32, i32, i32);

fn parse_line(s: &str) -> Line {
  let points: Vec<i32> = s
    .split(" -> ")
    .flat_map(|p| p.split(","))
    .map(|s| s.parse().unwrap())
    .collect();
  (points[0], points[1], points[2], points[3])
}
