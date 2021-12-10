fn first_pass(positions_: &Vec<usize>) -> usize {
    let mut positions = (*positions_).clone();
    positions.sort();
    let lowest_cost_position = positions[positions.len() / 2];
    let mut cost: usize = 0;
    for position in positions {
        cost += (lowest_cost_position as isize - position as isize).abs() as usize;
    }
    return cost;
}

fn second_pass(positions: &Vec<usize>) -> usize {
    let mut lowest_cost: usize = 0;
    for test_position in positions {
        let mut cost: usize = 0;
        for position in positions {
            let distance = (*position as isize - *test_position as isize).abs() as usize;
            cost += distance * (distance + 1) / 2; // Same as summing distance..distance + 1
        }
        if lowest_cost == 0 || cost < lowest_cost {
            lowest_cost = cost;
        }
    }
    return lowest_cost;
}

pub fn run() {
    let contents = crate::utils::read_input("day07.txt");
    let positions: &Vec<usize> = &contents
        .split(",")
        .map(|pos| pos.parse::<usize>().unwrap())
        .collect();

    println!("Part1: {:?}", first_pass(positions));
    println!("Part2: {:?}", second_pass(positions));
}
