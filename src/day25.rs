pub fn run() {
  let mut sea_floor: Vec<Vec<_>> = include_str!("inputs/day25")
    .lines()
    .map(|l| {
      l.chars()
        .map(|ch| match ch {
          '.' => Tile::Empty,
          '>' => Tile::EastCucumber,
          'v' => Tile::SouthCucumber,
          _ => unreachable!(),
        })
        .collect()
    })
    .collect();

  let mut steps = 1;
  while step(&mut sea_floor) {
    steps += 1;
  }
  println!("{}", steps)
}

#[derive(Eq, PartialEq, Clone)]
enum Tile {
  Empty,
  EastCucumber,
  SouthCucumber,
}

fn step(sea_floor: &mut Vec<Vec<Tile>>) -> bool {
  let height = sea_floor.len();
  let width = sea_floor[0].len();

  let mut east_moves = vec![];
  for y in 0..height {
    for x in 0..width {
      let east_x = (x + 1) % width;
      if sea_floor[y][x] == Tile::EastCucumber && sea_floor[y][east_x] == Tile::Empty {
        east_moves.push((x, y, east_x))
      }
    }
  }

  for &(x, y, east_x) in east_moves.iter() {
    sea_floor[y][x] = Tile::Empty;
    sea_floor[y][east_x] = Tile::EastCucumber;
  }

  let mut south_moves = vec![];
  for x in 0..width {
    for y in 0..height {
      let south_y = (y + 1) % height;
      if sea_floor[y][x] == Tile::SouthCucumber && sea_floor[south_y][x] == Tile::Empty {
        south_moves.push((x, y, south_y));
      }
    }
  }

  for &(x, y, south_y) in south_moves.iter() {
    sea_floor[y][x] = Tile::Empty;
    sea_floor[south_y][x] = Tile::SouthCucumber;
  }

  east_moves.len() + south_moves.len() > 0
}
