use std::collections::HashSet;

pub fn run() {
  let map: Vec<Vec<i32>> = include_str!("inputs/day09")
    .lines()
    .map(|l| {
      l.chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect()
    })
    .collect();
  let low_points = get_low_points(&map);
  let low_points_risk: i32 = low_points
    .iter()
    .map(|(x, y)| map[*y as usize][*x as usize] + 1)
    .sum();
  println!("{}", low_points_risk);

  let mut basin_sizes = get_basin_sizes(&map, &low_points);
  basin_sizes.sort();
  println!("{}", basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b));
}

fn get_low_points(map: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
  let height = map.len() as i32;
  let width = map[0].len() as i32;
  let mut low_points = vec![];
  for y in 0..height {
    for x in 0..width {
      let value = map[y as usize][x as usize];
      let is_low_point = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .iter()
        .filter(|&&(nx, ny)| 0 <= nx && nx < width && 0 <= ny && ny < height)
        .all(|&(nx, ny)| value < map[ny as usize][nx as usize]);
      if is_low_point {
        low_points.push((x, y));
      }
    }
  }
  low_points
}

fn get_basin_sizes(map: &Vec<Vec<i32>>, low_points: &Vec<(i32, i32)>) -> Vec<usize> {
  let height = map.len() as i32;
  let width = map[0].len() as i32;
  low_points
    .iter()
    .map(|&(x, y)| {
      let mut basin = HashSet::from([(x, y)]);
      let mut unvisited = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      while let Some((x, y)) = unvisited.pop() {
        if x < 0 || x >= width || y < 0 || y >= height {
          continue;
        }
        if basin.contains(&(x, y)) {
          continue;
        }
        if map[y as usize][x as usize] == 9 {
          continue;
        }
        basin.insert((x, y));
        for point in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
          unvisited.push(point)
        }
      }
      basin.len()
    })
    .collect()
}
