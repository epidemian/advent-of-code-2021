pub fn run() {
  let all_instructions: Vec<Instruction> = include_str!("inputs/day22")
    .lines()
    .map(parse_instruction)
    .collect();

  let init_cuboid = Cuboid {
    xs: Range::new(-50, 50),
    ys: Range::new(-50, 50),
    zs: Range::new(-50, 50),
  };
  let init_instructions: Vec<Instruction> = all_instructions
    .iter()
    .filter_map(|(v, c)| c.intersect(&init_cuboid).map(|i| (*v, i)))
    .collect();

  println!("{}", run_instructions(init_instructions));
  println!("{}", run_instructions(all_instructions));
}

fn run_instructions(instructions: Vec<Instruction>) -> i64 {
  let mut processed: Vec<Instruction> = vec![];

  for (value, cuboid) in instructions {
    let negated_intersections: Vec<Instruction> = processed
      .iter()
      .filter_map(|(v, c)| c.intersect(&cuboid).map(|i| (-v, i)))
      .collect();
    // Avoid counting intersections multiple times by adding their negative values.
    processed.extend(negated_intersections);
    if value == ON {
      processed.push((value, cuboid))
    }
  }

  processed.iter().map(|(val, c)| *val * c.size()).sum()
}

const ON: i64 = 1;
const OFF: i64 = -1;

type Instruction = (i64, Cuboid);

struct Cuboid {
  xs: Range,
  ys: Range,
  zs: Range,
}

struct Range {
  start: i32,
  end: i32,
}

fn parse_instruction(s: &str) -> Instruction {
  let (value, coords) = s.split_once(" ").unwrap();
  let value = if value == "on" { ON } else { OFF };
  (value, Cuboid::parse(coords))
}

impl Cuboid {
  fn parse(s: &str) -> Cuboid {
    let mut ranges = s
      .split(",")
      .map(|s| Range::parse(s.split_once("=").unwrap().1));
    Cuboid {
      xs: ranges.next().unwrap(),
      ys: ranges.next().unwrap(),
      zs: ranges.next().unwrap(),
    }
  }

  fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
    match (
      self.xs.intersect(&other.xs),
      self.ys.intersect(&other.ys),
      self.zs.intersect(&other.zs),
    ) {
      (Some(xs), Some(ys), Some(zs)) => Some(Cuboid { xs, ys, zs }),
      _ => None,
    }
  }

  fn size(&self) -> i64 {
    self.xs.size() as i64 * self.ys.size() as i64 * self.zs.size() as i64
  }
}

impl Range {
  fn parse(s: &str) -> Range {
    let (start, end) = s.split_once("..").unwrap();
    Range::new(start.parse().unwrap(), end.parse().unwrap())
  }

  fn new(start: i32, end: i32) -> Range {
    Range { start, end }
  }

  fn intersect(&self, other: &Range) -> Option<Range> {
    let start = self.start.max(other.start);
    let end = self.end.min(other.end);
    if start <= end {
      Some(Range { start, end })
    } else {
      None
    }
  }

  fn size(&self) -> i32 {
    self.end - self.start + 1
  }
}
