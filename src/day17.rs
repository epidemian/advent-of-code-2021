pub fn run() {
  // Doesn't really make sense to parse this input.
  let area_x = 48..=70;
  let area_y = -189..=-148;

  let probe_hits_area = |v0: (i32, i32)| {
    let (mut x, mut y) = (0, 0);
    let (mut vx, mut vy) = v0;
    while x < *area_x.end() && y > *area_y.start() {
      x += vx;
      y += vy;
      vx -= vx.min(1);
      vy -= 1;
      if area_x.contains(&x) && area_y.contains(&y) {
        return true;
      }
    }
    false
  };

  let mut successful_velocities = vec![];
  // Values sort of eyeballed after trying out larger limits.
  for vx0 in 0..100 {
    for vy0 in -200..200 {
      if probe_hits_area((vx0, vy0)) {
        successful_velocities.push((vx0, vy0));
      }
    }
  }

  let max_vy = successful_velocities
    .iter()
    .map(|(_, vy)| vy)
    .max()
    .unwrap();
  let max_height = (max_vy + 1) * max_vy / 2;

  println!("{}", max_height);
  println!("{}", successful_velocities.len());
}
