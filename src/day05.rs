const SIZE: usize = 1000;

pub fn run() {
  let all_lines: Vec<Line> = include_str!("inputs/day05")
    .lines()
    .map(Line::parse)
    .collect();
  let non_diagonal_lines = all_lines
    .iter()
    .filter(|Line(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

  println!("{}", count_overlaps(non_diagonal_lines));
  println!("{}", count_overlaps(all_lines.iter()));
}

fn count_overlaps<'a, I>(lines: I) -> usize
where
  I: Iterator<Item = &'a Line>,
{
  let mut map = [[0; SIZE]; SIZE];
  for &Line(x1, y1, x2, y2) in lines {
    map[x1 as usize][y1 as usize] += 1;

    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    let mut x = x1;
    let mut y = y1;
    while !(x == x2 && y == y2) {
      x += dx;
      y += dy;
      map[x as usize][y as usize] += 1;
    }
  }
  map
    .iter()
    .map(|line| line.iter().filter(|&&cell| cell >= 2).count())
    .sum()
}

#[derive(Debug)]
struct Line(i32, i32, i32, i32);

impl Line {
  fn parse(s: &str) -> Line {
    let points: Vec<i32> = s
      .split(" -> ")
      .flat_map(|p| p.split(","))
      .map(|s| s.parse().unwrap())
      .collect();
    Line(points[0], points[1], points[2], points[3])
  }
}
