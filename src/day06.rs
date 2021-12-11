pub fn run() {
  let fish_timers: Vec<usize> = include_str!("inputs/day06")
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  println!("{}", simulate_fish_growth(&fish_timers, 80));
  println!("{}", simulate_fish_growth(&fish_timers, 256));
}

fn simulate_fish_growth(initial_timers: &[usize], days: usize) -> u64 {
  let mut fish_by_timer = [0 as u64; 9];
  for &t in initial_timers {
    fish_by_timer[t] += 1
  }

  for _ in 1..=days {
    // All fish timers decrease by 1; fish with timer=0 become new fish with
    // timer 8.
    fish_by_timer.rotate_left(1);
    // Put the fish that had timer=0 into the timer=6 bucket.
    fish_by_timer[6] += fish_by_timer[8];
  }
  fish_by_timer.iter().sum()
}
