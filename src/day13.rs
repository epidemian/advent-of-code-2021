use std::collections::HashSet;

pub fn run() {
  let (dots_part, folds_part) = include_str!("inputs/day13").split_once("\n\n").unwrap();
  let mut dots: Dots = parse_dots(dots_part);
  let folds: Vec<Fold> = parse_folds(folds_part);

  for (i, fold) in folds.iter().enumerate() {
    dots = fold_paper(&dots, fold);
    if i == 0 {
      println!("{}", dots.len());
    }
  }

  print_dots(&dots);
}

type Dots = HashSet<(i32, i32)>;
type Fold = (char, i32);

fn parse_dots(s: &str) -> Dots {
  s.lines()
    .map(|l| {
      let (x, y) = l.split_once(",").unwrap();
      (x.parse().unwrap(), y.parse().unwrap())
    })
    .collect()
}

fn parse_folds(s: &str) -> Vec<Fold> {
  s.lines()
    .map(|l| {
      let (lhs, rhs) = l.split_once("=").unwrap();
      (lhs.chars().last().unwrap(), rhs.parse().unwrap())
    })
    .collect()
}

fn fold_paper(dots: &Dots, fold: &Fold) -> Dots {
  dots
    .iter()
    .map(|&(x, y)| match fold {
      &('x', fx) => (if x < fx { x } else { fx - (x - fx) }, y),
      &('y', fy) => (x, if y < fy { y } else { fy - (y - fy) }),
      _ => unreachable!(),
    })
    .collect()
}

fn print_dots(dots: &Dots) {
  let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
  let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();

  for y in 0..=max_y {
    for x in 0..=max_x {
      print!("{}", if dots.contains(&(x, y)) { "#" } else { " " })
    }
    println!()
  }
}
