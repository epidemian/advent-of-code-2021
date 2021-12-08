use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = args.last().unwrap();
    match day_num.as_ref() {
        "1" => day01::run(),
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        "5" => day05::run(),
        _ => println!("invalid day number {}", day_num),
    }
}
