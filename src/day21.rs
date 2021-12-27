pub fn run() {
  // Doesn't make sense to parse the input on this one really.
  let p1_pos = 6;
  let p2_pos = 7;

  part1(p1_pos, p2_pos);
  part2(p1_pos, p2_pos);
}

fn part1(p1_pos: i32, p2_pos: i32) {
  let mut positions = [p1_pos, p2_pos];
  let mut scores = [0, 0];
  let mut player = 0;
  let mut roll = 1;

  loop {
    for _ in 0..3 {
      positions[player] = (positions[player] + roll - 1) % 10 + 1;
      roll += 1;
    }
    scores[player] += positions[player];
    if scores[player] >= 1000 {
      break;
    }

    player = (player + 1) % 2;
  }

  println!("{}", scores[0].min(scores[1]) * (roll - 1))
}

fn part2(p1_pos: i32, p2_pos: i32) {
  let (p1_wins, p2_wins) = dirac_dice_wins(p1_pos, p2_pos, 0, 0);
  println!("{}", p1_wins.max(p2_wins));
}

// For 3 rolls of a Dirac dice there is only one chance for them to add up to 3,
// 3 chances of adding up to 4, and so on...
const DIRAC_DICE_3_ROLL_CHANCES: [(i32, usize); 7] =
  [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

// Calculates the number of universes in which player 1 and 2 win.
fn dirac_dice_wins(p1_pos: i32, p2_pos: i32, p1_score: i32, p2_score: i32) -> (usize, usize) {
  let (mut p1_wins, mut p2_wins) = (0, 0);
  for (die_roll, chance) in DIRAC_DICE_3_ROLL_CHANCES {
    let p1_sub_pos = (p1_pos + die_roll - 1) % 10 + 1;
    let p1_sub_score = p1_score + p1_sub_pos;
    if p1_sub_score >= 21 {
      p1_wins += chance
    } else {
      let (p2_sub_wins, p1_sub_wins) = dirac_dice_wins(p2_pos, p1_sub_pos, p2_score, p1_sub_score);
      p1_wins += chance * p1_sub_wins;
      p2_wins += chance * p2_sub_wins;
    }
  }
  (p1_wins, p2_wins)
}
