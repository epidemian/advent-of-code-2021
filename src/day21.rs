pub fn run() {
  let p1_pos = 6;
  let p2_pos = 7;
  part1(p1_pos, p2_pos);
}

fn part1(mut p1_pos: i32, mut p2_pos: i32) {
  let mut p1_score = 0;
  let mut p2_score = 0;
  let mut die = 100;
  let mut rolls = 0;

  loop {
    for _ in 0..3 {
      die = die % 100 + 1;
      rolls += 1;
      p1_pos = (p1_pos + die - 1) % 10 + 1;
    }
    p1_score += p1_pos;
    if p1_score >= 1000 {
      break;
    }

    for _ in 0..3 {
      die = die % 100 + 1;
      rolls += 1;
      p2_pos = (p2_pos + die - 1) % 10 + 1;
    }
    p2_score += p2_pos;
    if p2_score >= 1000 {
      break;
    }
  }

  println!("{}", p1_score.min(p2_score) * rolls)
}
