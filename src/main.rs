use std::env;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Need to pass day number as first argument");
    let day = &args[1]
        .parse::<i32>()
        .expect("Day number needs to be an integer");

    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        4 => day_04::run(),
        5 => day_05::run(),
        6 => day_06::run(),
        7 => day_07::run(),
        8 => day_08::run(),
        9 => day_09::run(),
        10 => day_10::run(),
        11 => day_11::run(),
        _ => println!("Day {} not implemented", day),
    }
}
