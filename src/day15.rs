use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn run() {
  let risk_map: Vec<Vec<u32>> = include_str!("inputs/day15")
    .lines()
    .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
    .collect();

  println!("{}", shortest_path(&risk_map));

  let full_risk_map = expand_map(&risk_map);
  println!("{}", shortest_path(&full_risk_map));
}

fn expand_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let size = map.len();
  let full_size = size * 5;
  let mut full_map: Vec<Vec<u32>> = vec![vec![0; full_size]; full_size];
  for y in 0..full_size {
    for x in 0..full_size {
      let risk = map[y % size][x % size] + (x / size + y / size) as u32;
      let wrapped_risk = (risk - 1) % 9 + 1;
      full_map[y][x] = wrapped_risk;
    }
  }
  full_map
}

// A very ad-hoc attempt at Dijkstra's algorithm.
fn shortest_path(risk_map: &Vec<Vec<u32>>) -> u32 {
  let size = risk_map.len();
  let start = (0, 0);
  let end = (size - 1, size - 1);

  let mut unvisited = BinaryHeap::new();
  let mut distances = vec![vec![u32::MAX; size]; size];

  distances[start.1][start.0] = 0;
  unvisited.push(Node {
    position: start,
    cost: 0,
  });

  while let Some(min_cost_node) = unvisited.pop() {
    let Node { position, cost } = min_cost_node;
    let (x, y) = position;

    if position == end {
      return cost;
    }
    if distances[y][x] < cost {
      continue;
    }

    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
      .iter()
      .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
      .filter(|&(x, y)| 0 <= x && x < size as i32 && 0 <= y && y < size as i32);

    for (nx, ny) in neighbors {
      let (nx, ny) = (nx as usize, ny as usize);
      let neighbor_cost = cost + risk_map[ny][nx];
      if neighbor_cost < distances[ny][nx] {
        distances[ny][nx] = neighbor_cost;
        unvisited.push(Node {
          position: (nx, ny),
          cost: neighbor_cost,
        });
      }
    }
  }
  unreachable!()
}

#[derive(PartialEq, Eq)]
struct Node {
  position: (usize, usize),
  cost: u32,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    // Compare cost in the other way so the binary heap is min-sorted.
    let cost_order = other.cost.cmp(&self.cost);
    cost_order.then_with(|| self.position.cmp(&other.position))
  }
}

// Rust requires this for some reason too.
impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
