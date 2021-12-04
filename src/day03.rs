pub fn run() {
  part1();
  part2();
}

fn part1() {
  let nums = get_input_numbers();
  let bit_size = get_bit_size();

  let mut gamma = 0;
  for i in 0..bit_size {
    gamma |= most_common_bit_at(nums.as_ref(), i) << i;
  }
  let epsilon = !gamma & (1 << bit_size) - 1;
  println!("{}", gamma * epsilon);
}

fn part2() {
  let mut oxigen_nums = get_input_numbers();
  let mut index = get_bit_size() - 1;
  loop {
    let value = most_common_bit_at(oxigen_nums.as_ref(), index);
    oxigen_nums.retain(|&n| bit_at(n, index) == value);
    if oxigen_nums.len() == 1 {
      break;
    }
    index -= 1;
  }

  let mut co2_nums = get_input_numbers();
  let mut index = get_bit_size() - 1;
  loop {
    let value = most_common_bit_at(co2_nums.as_ref(), index);
    co2_nums.retain(|&n| bit_at(n, index) != value);
    if co2_nums.len() == 1 {
      break;
    }
    index -= 1;
  }

  println!("{}", oxigen_nums[0] * co2_nums[0])
}

static INPUT: &str = include_str!("inputs/day03");

fn get_input_numbers() -> Vec<i32> {
  INPUT
    .lines()
    .map(|s| i32::from_str_radix(s, 2).unwrap())
    .collect()
}

fn get_bit_size() -> usize {
  INPUT.lines().next().unwrap().len()
}

fn bit_at(n: i32, i: usize) -> i32 {
  (n >> i) & 1
}

fn most_common_bit_at(nums: &[i32], index: usize) -> i32 {
  let ones_count = nums.iter().filter(|&n| bit_at(*n, index) == 1).count();
  (ones_count * 2 >= nums.len()) as i32
}
