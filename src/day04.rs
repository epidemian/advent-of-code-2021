type Board = Vec<(i32, bool)>;

pub fn run() {
  let mut input_parts = include_str!("inputs/day04").split("\n\n");
  let numbers: Vec<i32> = input_parts
    .next()
    .unwrap()
    .split(",")
    .map(|s| s.parse().unwrap())
    .collect();
  let mut boards: Vec<Board> = input_parts
    .map(|s| {
      s.split_whitespace()
        .map(|s| (s.parse().unwrap(), false))
        .collect()
    })
    .collect();
  let board_count = boards.len();
  let mut board_win_order = vec![];

  for number in numbers {
    for (board_index, board) in boards.iter_mut().enumerate() {
      if board_win_order.contains(&board_index) {
        continue;
      }
      for (num, marked) in board.iter_mut() {
        if *num == number {
          *marked = true;
        }
      }
      if has_complete_line(board) || has_complete_column(board) {
        board_win_order.push(board_index);

        if board_win_order.len() == 1 || board_win_order.len() == board_count {
          let unmarked_sum: i32 = board
            .iter()
            .filter(|(_, marked)| !marked)
            .map(|(n, _)| *n)
            .sum();
          println!("{}", unmarked_sum * number);
        }
      }
    }
  }
}

fn has_complete_line(board: &Board) -> bool {
  (0..5).any(|line| (0..5).all(|i| board[line * 5 + i].1))
}

fn has_complete_column(board: &Board) -> bool {
  (0..5).any(|col| (0..5).all(|i| board[col + 5 * i].1))
}
