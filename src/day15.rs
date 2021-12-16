pub fn run() {
  let risk_map: Vec<Vec<u32>> = include_str!("inputs/day15")
    .lines()
    .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
    .collect();

  println!("{}", get_path_total_risk(&risk_map));

  let full_risk_map = expand_map(&risk_map);
  println!("{}", get_path_total_risk(&full_risk_map));
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

// This doesn't seem like a proper path-finding algorithm, but it worked for
// this problem ¯\_(ツ)_/¯
fn get_path_total_risk(risk_map: &Vec<Vec<u32>>) -> u32 {
  let size = risk_map.len();
  let mut cost_map: Vec<Vec<u32>> = vec![vec![0; size]; size];
  for x in 1..size {
    cost_map[0][x] = cost_map[0][x - 1] + risk_map[0][x];
  }
  for y in 1..size {
    cost_map[y][0] = cost_map[y - 1][0] + risk_map[y][0];
  }
  for y in 1..size {
    for x in 1..size {
      let cost_up = cost_map[y - 1][x];
      let cost_left = cost_map[y][x - 1];
      cost_map[y][x] = cost_up.min(cost_left) + risk_map[y][x];
      if cost_up < cost_left {
        reassess_cost_at(x - 1, y, &mut cost_map, risk_map);
      } else {
        reassess_cost_at(x, y - 1, &mut cost_map, risk_map);
      }
    }
  }
  cost_map[size - 1][size - 1]
}

const NEIGHBOR_DIFFS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn reassess_cost_at(x: usize, y: usize, cost_map: &mut Vec<Vec<u32>>, risk_map: &Vec<Vec<u32>>) {
  let size = cost_map.len() as i32;
  let neighbors = NEIGHBOR_DIFFS
    .iter()
    .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
    .filter(|&(x, y)| 0 <= x && x < size && 0 <= y && y < size);
  let min_neighbor_cost = neighbors
    .clone()
    .map(|(x, y)| cost_map[y as usize][x as usize])
    .filter(|&cost| cost != 0)
    .min();

  if let Some(neighbor_cost) = min_neighbor_cost {
    if cost_map[y][x] > neighbor_cost + risk_map[y][x] {
      cost_map[y][x] = neighbor_cost + risk_map[y][x];
      for (x, y) in neighbors {
        reassess_cost_at(x as usize, y as usize, cost_map, risk_map);
      }
    }
  }
}
