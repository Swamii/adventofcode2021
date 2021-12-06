use std::collections::VecDeque;

const RESET_STATE: usize = 6;

fn simulate(initial_states: &VecDeque<u64>, number_of_days: usize) -> u64 {
    let mut states = initial_states.to_owned();
    for _ in 0..number_of_days {
        let recreating = states.pop_front().unwrap();
        // Reset all the ones previously at 0 to 6
        states[RESET_STATE] += recreating;
        // Add the same amount to the back (8 value)
        states.push_back(recreating);
    }
    return states.iter().sum();
}

pub fn run() {
    let contents = crate::utils::read_input("day6.txt");
    let mut timers = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for state in contents
        .split(",")
        .map(|state| state.parse::<usize>().unwrap())
    {
        timers[state] += 1;
    }

    println!("Part1: {}", simulate(&timers, 80));
    println!("Part2: {}", simulate(&timers, 256));
}
