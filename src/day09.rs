use std::collections::{HashMap, HashSet};

type Map = HashMap<(i32, i32), i32>;

pub fn run() {
  let mut map: Map = Map::new();
  for (y, line) in include_str!("inputs/day09").lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      map.insert((x as i32, y as i32), ch.to_digit(10).unwrap() as i32);
    }
  }

  let low_points = get_low_points(&map);
  let risk: i32 = low_points.iter().map(|p| map.get(p).unwrap() + 1).sum();
  println!("{}", risk);

  let mut basin_sizes = get_basin_sizes(&map, &low_points);
  basin_sizes.sort();
  println!("{}", basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b));
}

fn get_low_points(map: &Map) -> Vec<(i32, i32)> {
  map
    .iter()
    .filter(|&(&(x, y), value)| {
      [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .iter()
        .all(|point| value < map.get(point).unwrap_or(&10))
    })
    .map(|(&k, _)| k)
    .collect()
}

fn get_basin_sizes(map: &Map, low_points: &Vec<(i32, i32)>) -> Vec<usize> {
  low_points
    .iter()
    .map(|&(x, y)| {
      let mut basin = HashSet::from([(x, y)]);
      let mut unvisited = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      while let Some((x, y)) = unvisited.pop() {
        if basin.contains(&(x, y)) {
          continue;
        }
        match map.get(&(x, y)) {
          None => continue,
          Some(9) => continue,
          Some(_) => {
            basin.insert((x, y));
            for point in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
              unvisited.push(point)
            }
          }
        }
      }
      basin.len()
    })
    .collect()
}
