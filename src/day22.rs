pub fn run() {
  let all_instructions: Vec<Instruction> = include_str!("inputs/day22")
    .lines()
    .map(parse_instruction)
    .collect();

  let init_cuboid = Cuboid {
    start: (-50, -50, -50),
    end: (50, 50, 50),
  };
  let init_instructions: Vec<Instruction> = all_instructions
    .iter()
    .filter_map(|(v, c)| c.intersect(&init_cuboid).map(|i| (*v, i)))
    .collect();

  println!("{}", run_instructions(init_instructions));
  println!("{}", run_instructions(all_instructions));
}

fn run_instructions(instructions: Vec<Instruction>) -> i64 {
  let mut cuboids: Vec<Instruction> = vec![];

  for (value, cuboid) in instructions {
    let intersections: Vec<Instruction> = cuboids
      .iter()
      .filter_map(|(v, c)| c.intersect(&cuboid).map(|i| (-v, i)))
      .collect();
    cuboids.extend(intersections);
    if value == ON {
      cuboids.push((value, cuboid))
    }
  }

  cuboids.iter().map(|(val, c)| *val as i64 * c.size()).sum()
}

type Instruction = (i32, Cuboid);
type Point = (i32, i32, i32);

#[derive(Debug)]
struct Cuboid {
  start: Point,
  end: Point,
}

const ON: i32 = 1;
const OFF: i32 = -1;

fn parse_instruction(s: &str) -> Instruction {
  let (value, coords) = s.split_once(" ").unwrap();
  let coords: Vec<_> = coords
    .split(",")
    .map(|s| {
      let (start, end) = s.split_once("=").unwrap().1.split_once("..").unwrap();
      (start.parse().unwrap(), end.parse().unwrap())
    })
    .collect();

  let value = if value == "on" { ON } else { OFF };
  let cuboid = Cuboid {
    start: (coords[0].0, coords[1].0, coords[2].0),
    end: (coords[0].1, coords[1].1, coords[2].1),
  };
  (value, cuboid)
}

fn intersect_ranges(range_a: (i32, i32), range_b: (i32, i32)) -> Option<(i32, i32)> {
  let start = range_a.0.max(range_b.0);
  let end = range_a.1.min(range_b.1);
  if start <= end {
    Some((start, end))
  } else {
    None
  }
}

impl Cuboid {
  fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
    let (sx, sy, sz) = self.start;
    let (ex, ey, ez) = self.end;
    let (other_sx, other_sy, other_sz) = other.start;
    let (other_ex, other_ey, other_ez) = other.end;
    match (
      intersect_ranges((sx, ex), (other_sx, other_ex)),
      intersect_ranges((sy, ey), (other_sy, other_ey)),
      intersect_ranges((sz, ez), (other_sz, other_ez)),
    ) {
      (Some((sx, ex)), Some((sy, ey)), Some((sz, ez))) => Some(Cuboid {
        start: (sx, sy, sz),
        end: (ex, ey, ez),
      }),
      _ => None,
    }
  }

  fn size(&self) -> i64 {
    let (sx, sy, sz) = self.start;
    let (ex, ey, ez) = self.end;
    let dx = (ex - sx + 1) as i64;
    let dy = (ey - sy + 1) as i64;
    let dz = (ez - sz + 1) as i64;
    dx * dy * dz
  }
}
