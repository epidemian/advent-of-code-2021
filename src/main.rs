use std::fs;

fn main() {
    // Part 1
    let input = fs::read_to_string("inputs/day01").unwrap();
    let numbers: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut last_num = 0;
    let mut increase_count = -1; // First number shouldn't count.
    for &num in numbers.iter() {
        if num > last_num {
            increase_count += 1;
        }
        last_num = num;
    }
    println!("{}", increase_count);

    // Part 2
    let mut sums = vec![];
    for i in 0..numbers.len() - 2 {
        sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }

    let mut last_num = 0;
    let mut increase_count = -1; // First number shouldn't count.
    for &num in sums.iter() {
        if num > last_num {
            increase_count += 1;
        }
        last_num = num;
    }

    println!("{}", increase_count);
}
