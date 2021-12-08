const UNIQUE_LENGTHS: [usize; 4] = [2, 3, 4, 7];

fn first_pass(entries: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut number_of_unique_entries = 0;

    for (_, output_value) in entries {
        for value in output_value {
            if UNIQUE_LENGTHS.contains(&value.len()) {
                number_of_unique_entries += 1;
            }
        }
    }
    return number_of_unique_entries;
}

fn second_pass(entries: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut sum = 0;

    for (signal_patterns, output_value) in entries.iter() {
        let mut patterns: [String; 10] = Default::default();
        patterns[1] = signal_patterns[0].clone();
        patterns[7] = signal_patterns[1].clone();
        patterns[4] = signal_patterns[2].clone();
        patterns[8] = signal_patterns[9].clone();

        for pattern in &signal_patterns[6..9] {
            if crate::utils::characters_overlap(&patterns[1], pattern)
                && crate::utils::characters_overlap(&patterns[4], pattern)
            {
                patterns[9] = pattern.clone();
            }
            if !crate::utils::characters_overlap(&patterns[1], pattern)
                && !crate::utils::characters_overlap(&patterns[4], pattern)
            {
                patterns[6] = pattern.clone();
            }
            if crate::utils::characters_overlap(&patterns[1], pattern)
                && !crate::utils::characters_overlap(&patterns[4], pattern)
            {
                patterns[0] = pattern.clone();
            }
        }

        let top_right_char = patterns[1]
            .chars()
            .filter(|c: &char| !patterns[6].contains(*c))
            .collect::<Vec<char>>()[0];

        for pattern in &signal_patterns[3..6] {
            if !pattern.contains(&top_right_char.to_string()) {
                patterns[5] = pattern.clone();
            }
            if crate::utils::characters_overlap(&patterns[1], pattern) {
                patterns[3] = pattern.clone();
            }
            if pattern.contains(&top_right_char.to_string())
                && !crate::utils::characters_overlap(&patterns[1], pattern)
            {
                patterns[2] = pattern.clone();
            }
        }

        let mut full_number = String::new();
        for output_pattern in output_value {
            for (number, pattern) in patterns.iter().enumerate() {
                if pattern == output_pattern {
                    full_number += &number.to_string();
                    break;
                }
            }
        }
        if full_number.len() != 4 {
            panic!("Bad number {}", full_number);
        }
        sum += full_number.parse::<usize>().unwrap();
    }
    return sum;
}

pub fn run() {
    let contents = crate::utils::read_input("day8.txt");
    let entries: &Vec<(Vec<String>, Vec<String>)> = &contents
        .split("\n")
        .map(|line| {
            let (signal_patterns_raw, output_value_raw) = line.split_once(" | ").unwrap();
            let sort_string = |item: &str| crate::utils::sort_string(item);
            let sort_by_len = |item: &String| item.len();

            let mut signal_patterns = signal_patterns_raw
                .split_whitespace()
                .map(sort_string)
                .collect::<Vec<String>>();
            signal_patterns.sort_by_key(sort_by_len);
            let output_value: Vec<String> = output_value_raw
                .split_whitespace()
                .map(sort_string)
                .collect();
            return (signal_patterns, output_value);
        })
        .collect();

    println!("Part1: {}", first_pass(entries));
    println!("Part2: {}", second_pass(entries));
}
