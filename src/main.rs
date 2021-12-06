use std::env;
mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Need to pass day number as first argument");
    let day = &args[1]
        .parse::<i32>()
        .expect("Day number needs to be an integer");

    match day {
        1 => day_one::run(),
        2 => day_two::run(),
        3 => day_three::run(),
        4 => day_four::run(),
        5 => day_five::run(),
        6 => day_six::run(),
        _ => println!("Day {} not implemented", day),
    }
}
