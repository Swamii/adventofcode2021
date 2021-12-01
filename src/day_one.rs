const FILLED_WINDOW_LEN: usize = 3;

fn first_pass(lines: &Vec<i32>) -> i32 {
    let mut increases = 0;
    let mut previous_value: Option<i32> = None;
    for &number in lines {
        if let Some(prev) = previous_value {
            if number > prev {
                increases += 1;
            }
        }
        previous_value = Some(number);
    }
    return increases;
}

fn second_pass(lines: &Vec<i32>) -> i32 {
    let mut sliding_windows: Vec<Vec<i32>> = Vec::new();
    let mut increases = 0;

    for &number in lines {
        let previous_filled_window = sliding_windows
            .iter()
            .rev()
            .find(|x| x.len() == FILLED_WINDOW_LEN);
        let previous_filled_sum = match previous_filled_window {
            Some(prev) => Some(prev.iter().sum::<i32>()),
            None => None,
        };
        for window in sliding_windows
            .iter_mut()
            .filter(|window| window.len() < FILLED_WINDOW_LEN)
            .collect::<Vec<&mut Vec<i32>>>()
        {
            window.push(number);
            if let Some(previous_sum) = previous_filled_sum {
                if window.len() == FILLED_WINDOW_LEN && window.iter().sum::<i32>() > previous_sum {
                    increases += 1;
                }
            }
        }
        sliding_windows.push(vec![number]);
    }
    return increases;
}

pub fn run() {
    let contents = crate::utils::read_input("day1.txt");
    let lines: &Vec<i32> = &contents
        .split("\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let first_pass_result = first_pass(lines);
    println!("Number of increases: {}", first_pass_result);
    let second_pass_result = second_pass(lines);
    println!("Number of increased windows: {}", second_pass_result);
}
