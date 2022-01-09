use std::collections::{HashMap, HashSet};

pub fn run() {
  let scanners: Vec<Vec<Point>> = include_str!("inputs/day19")
    .split("\n\n")
    .map(|section| {
      section
        .lines()
        .filter(|l| !l.starts_with("---"))
        .map(parse_point)
        .collect()
    })
    .collect();

  let mut known_beacons = HashSet::new();
  let mut scanner_positions = HashMap::new();

  known_beacons.extend(&scanners[0]);
  scanner_positions.insert(0, (0, 0, 0));

  while scanner_positions.len() < scanners.len() {
    for scanner_index in 1..scanners.len() {
      if scanner_positions.contains_key(&scanner_index) {
        continue;
      }
      let scanner = &scanners[scanner_index];
      let rots: Vec<_> = scanner.iter().map(all_rotations).collect();

      for rot_index in 0..rots[0].len() {
        let mut dist_counts = HashMap::new();
        for p in rots.iter().map(|rot| &rot[rot_index]) {
          for known_point in known_beacons.iter() {
            let d = sub(p, known_point);
            *dist_counts.entry(d).or_insert(0) += 1;
          }
        }

        if let Some((common_dist, _)) = dist_counts.iter().find(|&(_, count)| *count >= 12) {
          let scanner_points = rots.iter().map(|rot| &rot[rot_index]);
          known_beacons.extend(scanner_points.map(|p| sub(p, common_dist)));
          scanner_positions.insert(scanner_index, *common_dist);
          break;
        }
      }
    }
  }
  println!("{}", known_beacons.len());

  let max_dist = scanner_positions
    .values()
    .flat_map(|d1| scanner_positions.values().map(|d2| taxicab_dist(d1, d2)))
    .max()
    .unwrap();
  println!("{}", max_dist);
}

type Point = (i32, i32, i32);

fn parse_point(s: &str) -> Point {
  let ns: Vec<_> = s.split(",").map(|n| n.parse().unwrap()).collect();
  (ns[0], ns[1], ns[2])
}

fn sub((x1, y1, z1): &Point, (x2, y2, z2): &Point) -> Point {
  (x1 - x2, y1 - y2, z1 - z2)
}

fn all_rotations(&(x, y, z): &Point) -> [Point; 24] {
  // There surely is an algorithmic way of getting these rotations but, oh well...
  [
    // z is "up"
    (x, y, z),
    (-y, x, z),
    (-x, -y, z),
    (y, -x, z),
    // z is "down"
    (x, -y, -z),
    (y, x, -z),
    (-x, y, -z),
    (-y, -x, -z),
    // y is "up"
    (x, z, -y),
    (y, z, x),
    (-x, z, y),
    (-y, z, -x),
    // y is "down"
    (x, -z, y),
    (-y, -z, x),
    (-x, -z, -y),
    (y, -z, -x),
    // x is "up"
    (z, y, -x),
    (z, x, y),
    (z, -y, x),
    (z, -x, -y),
    // x is "down"
    (-z, y, x),
    (-z, -x, y),
    (-z, -y, -x),
    (-z, x, -y),
  ]
}

fn taxicab_dist((x1, y1, z1): &Point, (x2, y2, z2): &Point) -> i32 {
  (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}
