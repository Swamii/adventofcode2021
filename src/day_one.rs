pub fn run() {
    let contents = crate::utils::read_input("day1.txt");
    let mut previous_value: Option<i32> = None;
    let mut increases = 0;
    for line in contents.split("\n") {
        let number = line.parse::<i32>().unwrap();
        if let Some(prev) = previous_value {
            if number > prev {
                increases += 1;
            }
        }
        previous_value = Some(number);
    }
    println!("Number of increases: {}", increases);
    increases = 0;
    let mut sliding_windows: Vec<Vec<i32>> = Vec::new();

    for line in contents.split("\n") {
        let number = line.parse::<i32>().unwrap();
        let previous_filled_window = sliding_windows.iter().clone().rev().find(|x| x.len() == 3);
        let previous_filled_sum = match previous_filled_window {
            Some(prev) => Some(prev.iter().sum::<i32>()),
            None => None,
        };
        for window in sliding_windows.iter_mut() {
            let current_len = window.len();
            if current_len >= 3 {
                continue;
            }
            if current_len == 2 {
                // We will fill this window
                let newly_filled_sum = number + window.iter().sum::<i32>();
                if let Some(previous_sum) = previous_filled_sum {
                    if newly_filled_sum > previous_sum {
                        increases += 1;
                    }
                }
            }
            window.push(number);
        }
        sliding_windows.push(vec![number]);
    }
    println!("Number of increased windows: {}", increases);
}
