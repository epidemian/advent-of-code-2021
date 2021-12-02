use std::env;
mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = args.last().unwrap();
    match day_num.as_ref() {
        "1" => day01::run(),
        "2" => day02::run(),
        _ => println!("invalid day number {}", day_num),
    }
}
