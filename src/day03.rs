pub fn run() {
  part1();
  part2();
}

fn part1() {
  // Keep numbers as string. Easier random access than bit-fiddling.
  let nums: Vec<&str> = include_str!("inputs/day03").lines().collect();
  let mut counts = vec![0; nums.first().unwrap().len()];
  for num in nums.iter() {
    for (i, ch) in num.chars().enumerate() {
      counts[i] += if ch == '1' { 1 } else { -1 }
    }
  }
  let gamma = counts.iter().fold(0, |acc, &c| (acc << 1) | (c > 0) as i32);
  let epsilon = !gamma & (1 << counts.len()) - 1;
  println!("{}", gamma * epsilon);
}

fn part2() {
  let input = include_str!("inputs/day03");
  let nums: Vec<i32> = input
    .lines()
    .map(|s| i32::from_str_radix(s, 2).unwrap())
    .collect();
  let max_bit_index = input.lines().next().unwrap().len() - 1;

  let mut oxigen_nums = nums.clone();
  let mut index: usize = max_bit_index;
  loop {
    let value = most_common_bit_at(oxigen_nums.as_ref(), index);
    oxigen_nums.retain(|&n| bit_at(n, index) == value);
    if oxigen_nums.len() == 1 {
      break;
    }
    index -= 1;
  }
  let oxigen_generator_rating = oxigen_nums.first().unwrap();

  let mut co2_nums = nums.clone();
  let mut index: usize = max_bit_index;
  loop {
    let value = most_common_bit_at(co2_nums.as_ref(), index);
    co2_nums.retain(|&n| bit_at(n, index) != value);
    if co2_nums.len() == 1 {
      break;
    }
    index -= 1;
  }
  let co2_scrubber_rating = co2_nums.first().unwrap();

  println!("{}", oxigen_generator_rating * co2_scrubber_rating)
}

fn bit_at(n: i32, i: usize) -> i32 {
  (n >> i) & 1
}

fn most_common_bit_at(nums: &[i32], index: usize) -> i32 {
  let ones_count = nums.iter().filter(|&n| bit_at(*n, index) == 1).count();
  (ones_count * 2 >= nums.len()) as i32
}
