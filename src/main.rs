use std::env;
mod day_one;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Need to pass day number as first argument");
    let day = &args[1]
        .parse::<i32>()
        .expect("Day number needs to be an integer");

    if *day == 1 {
        println!("Running first day");
        day_one::run();
    }
}
