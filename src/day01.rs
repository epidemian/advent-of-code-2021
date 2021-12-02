pub fn run() {
  let input = include_str!("inputs/day01");
  let numbers: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
  println!("{}", count_increments(&numbers));

  let mut sums = vec![];
  for i in 0..numbers.len() - 2 {
    sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
  }
  println!("{}", count_increments(&sums));
}

fn count_increments(numbers: &Vec<i32>) -> i32 {
  let mut result = 0;
  for (i, &num) in numbers.iter().enumerate() {
    if i > 0 && num > numbers[i - 1] {
      result += 1;
    }
  }
  return result;
}
