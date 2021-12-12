use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
    ];
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day_num: usize = args[1].parse().unwrap();
        days[day_num - 1]()
    } else {
        for (i, fun) in days.iter().enumerate() {
            println!("day {}", i + 1);
            fun()
        }
    }
}
