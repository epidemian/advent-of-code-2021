use crate::dijkstra;

pub fn run() {
  let risk_map: Vec<Vec<usize>> = include_str!("inputs/day15")
    .lines()
    .map(|l| {
      l.chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect()
    })
    .collect();

  println!("{}", shortest_path(&risk_map));

  let full_risk_map = expand_map(&risk_map);
  println!("{}", shortest_path(&full_risk_map));
}

fn expand_map(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let size = map.len();
  let full_size = size * 5;
  let mut full_map: Vec<Vec<usize>> = vec![vec![0; full_size]; full_size];
  for y in 0..full_size {
    for x in 0..full_size {
      let risk = map[y % size][x % size] + (x / size + y / size);
      let wrapped_risk = (risk - 1) % 9 + 1;
      full_map[y][x] = wrapped_risk;
    }
  }
  full_map
}

fn shortest_path(risk_map: &Vec<Vec<usize>>) -> usize {
  let size = risk_map.len();
  let start = (0, 0);
  let end = (size - 1, size - 1);

  let neighbors = |&(x, y): &(usize, usize)| {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
      .iter()
      .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
      .filter(|&(x, y)| 0 <= x && x < size as i32 && 0 <= y && y < size as i32)
      .map(|(x, y)| ((x as usize, y as usize), risk_map[y as usize][x as usize]))
  };

  dijkstra::shortest_path(&start, &end, neighbors).unwrap()
}
