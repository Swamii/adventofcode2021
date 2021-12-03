use std::collections::HashMap;

const LINE_LEN: usize = 12;

fn count(lines: &Vec<&str>) -> Vec<HashMap<char, i32>> {
    let mut counters: Vec<HashMap<char, i32>> = Vec::new();
    for _ in 0..LINE_LEN {
        counters.push(HashMap::<char, i32>::new());
    }

    for &line in lines.iter() {
        for (index, char) in line.chars().enumerate() {
            let position_counter = &mut counters[index];
            *position_counter.entry(char).or_insert(0) += 1;
        }
    }
    return counters;
}

fn first_pass(lines: &Vec<&str>) -> (String, String) {
    let counters = count(&lines);
    let mut most_common = String::new();
    let mut least_common = String::new();

    for counter in counters.iter() {
        most_common.push(*counter.into_iter().max_by_key(|pair| pair.1).unwrap().0);
        least_common.push(*counter.into_iter().min_by_key(|pair| pair.1).unwrap().0);
    }
    return (least_common, most_common);
}

fn second_pass(lines: &Vec<&str>, prefer_low: bool) -> Option<String> {
    let mut rated_lines = lines.to_vec();
    let preferred_on_stalemate = if prefer_low { &'0' } else { &'1' };

    for index in 0..LINE_LEN {
        let remaining_counters = count(&rated_lines);
        let (most_common, most_common_occurence) = (&remaining_counters[index])
            .into_iter()
            .max_by_key(|pair| pair.1)
            .unwrap();
        let (least_common, least_common_occurence) = (&remaining_counters[index])
            .into_iter()
            .min_by_key(|pair| pair.1)
            .unwrap();

        let char_to_find = if most_common_occurence == least_common_occurence {
            preferred_on_stalemate
        } else if prefer_low {
            least_common
        } else {
            most_common
        };

        rated_lines = rated_lines
            .iter()
            .filter(|line| line.chars().nth(index) == Some(*char_to_find))
            .map(|line| *line)
            .collect();
        if rated_lines.len() == 1 {
            return Some(rated_lines[0].to_string());
        }
    }
    return None;
}

pub fn run() {
    let contents = crate::utils::read_input("day3.txt");
    let lines: &Vec<&str> = &contents.split("\n").collect();

    let (least_common, most_common) = first_pass(&lines);
    let gamma_rate = isize::from_str_radix(&most_common, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&least_common, 2).unwrap();
    println!(
        "Part1: Consumption {} * {} = {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let low = second_pass(&lines, true).unwrap();
    let high = second_pass(&lines, false).unwrap();
    let oxygen_rate = isize::from_str_radix(&high, 2).unwrap();
    let scrubber_rate = isize::from_str_radix(&low, 2).unwrap();
    println!(
        "Part2: Life support rating {} * {} = {}",
        oxygen_rate,
        scrubber_rate,
        oxygen_rate * scrubber_rate
    );
}
