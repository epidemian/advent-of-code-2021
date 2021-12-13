const SIZE: usize = 10;

pub fn run() {
  let mut octopi = [[0; SIZE]; SIZE];
  for (y, line) in include_str!("inputs/day11").lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      octopi[y][x] = ch.to_digit(10).unwrap();
    }
  }

  let mut step = 0;
  let mut total_flash_count = 0;
  loop {
    let mut step_flash_count = 0;
    let mut flashes = vec![];

    for y in 0..SIZE {
      for x in 0..SIZE {
        octopi[y][x] += 1;
        if octopi[y][x] >= 10 {
          flashes.push((x, y))
        }
      }
    }

    while let Some((x, y)) = flashes.pop() {
      octopi[y][x] = 0;
      step_flash_count += 1;
      for y2 in y.saturating_sub(1)..=(y + 1).min(SIZE - 1) {
        for x2 in x.saturating_sub(1)..=(x + 1).min(SIZE - 1) {
          if octopi[y2][x2] == 0 {
            // Neighbor already flashed.
            continue;
          }
          octopi[y2][x2] += 1;
          if octopi[y2][x2] >= 10 && !flashes.contains(&(x2, y2)) {
            flashes.push((x2, y2));
          }
        }
      }
    }

    step += 1;
    total_flash_count += step_flash_count;

    if step == 100 {
      println!("{}", total_flash_count);
    }
    if step_flash_count == SIZE * SIZE {
      println!("{}", step);
      break;
    }
  }
}
