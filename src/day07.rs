pub fn run() {
  let positions: Vec<i32> = include_str!("inputs/day07")
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();
  let max_pos = *positions.iter().max().unwrap();

  let best_cost: i32 = (0..=max_pos)
    .map(|pos| positions.iter().map(|p| (pos - p).abs()).sum())
    .min()
    .unwrap();
  println!("{}", best_cost);

  let best_cost: i32 = (0..=max_pos)
    .map(|pos| {
      positions
        .iter()
        .map(|p| (pos - p).abs()) // distances
        .map(|d| (d + 1) * d / 2) // sum of 1 + 2 + ... + d
        .sum()
    })
    .min()
    .unwrap();
  println!("{}", best_cost);
}
