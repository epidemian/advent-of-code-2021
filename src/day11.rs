use std::collections::HashSet;

const SIZE: usize = 10;

pub fn run() {
  let mut octopi = [[0; SIZE]; SIZE];
  for (y, line) in include_str!("inputs/day11").lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      octopi[y][x] = ch.to_digit(10).unwrap();
    }
  }

  let mut step = 0;
  let mut total_flashes = 0;
  loop {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..SIZE {
      for x in 0..SIZE {
        octopi[y][x] += 1;
      }
    }

    'process_flashes: loop {
      for y in 0..SIZE {
        for x in 0..SIZE {
          if octopi[y][x] >= 10 && !flashed.contains(&(x, y)) {
            flashed.insert((x, y));
            for dy in -1_i32..=1 {
              for dx in -1_i32..=1 {
                let y2 = (y as i32) + dy;
                let x2 = (x as i32) + dx;
                if x2 >= 0 && x2 < SIZE as i32 && y2 >= 0 && y2 < SIZE as i32 {
                  octopi[y2 as usize][x2 as usize] += 1;
                }
              }
            }
            continue 'process_flashes;
          }
        }
      }
      break 'process_flashes;
    }

    for y in 0..SIZE {
      for x in 0..SIZE {
        if octopi[y][x] >= 10 {
          octopi[y][x] = 0
        }
      }
    }

    step += 1;
    total_flashes += flashed.len();

    if step == 100 {
      println!("{}", total_flashes);
    }
    if flashed.len() == SIZE * SIZE {
      println!("{}", step);
      break;
    }
  }
}
