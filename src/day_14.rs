use std::collections::HashMap;

fn calculate(contents: &str, steps: usize) -> usize {
    let (template, rules_raw) = contents.split_once("\n\n").unwrap();
    let mut rules: HashMap<String, char> = HashMap::new();
    for rule in rules_raw.split("\n") {
        let (pair, insert) = rule.split_once(" -> ").unwrap();
        rules.insert(pair.to_string(), insert.chars().next().unwrap());
    }

    let mut results: HashMap<String, usize> = HashMap::new();
    let mut iter = template.chars().peekable();
    loop {
        let s1 = iter.next().unwrap();
        let s2 = match iter.peek() {
            Some(c) => c,
            None => break,
        };
        let s = format!("{}{}", s1, s2);
        results.insert(s, 1);
    }

    for _ in 0..steps {
        let mut new_results: HashMap<String, usize> = HashMap::new();
        for (pair, count) in results.iter() {
            let insert = rules[pair];
            *new_results
                .entry(format!("{}{}", pair.chars().next().unwrap(), insert))
                .or_insert(0) += count;
            *new_results
                .entry(format!("{}{}", insert, pair.chars().nth(1).unwrap()))
                .or_insert(0) += count;
        }
        results = new_results;
    }

    let mut position_counter: HashMap<char, usize> = HashMap::new();
    for (pair, count) in results {
        *position_counter
            .entry(pair.chars().next().unwrap())
            .or_insert(0) += count;
    }
    *position_counter
        .entry(template.chars().nth_back(0).unwrap())
        .or_insert(0) += 1;
    let max = position_counter.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let min = position_counter.iter().min_by_key(|(_, v)| *v).unwrap().1;
    return max - min;
}

pub fn run() {
    let contents = crate::utils::read_input("day14.txt");
    println!("Part1: {}", calculate(&contents, 10));
    println!("Part2: {}", calculate(&contents, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = &"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    const EXPECTED_PART1: usize = 1588;
    const EXPECTED_PART2: usize = 2188189693529;

    #[test]
    fn test_day_14_part1() {
        assert_eq!(calculate(INPUT.trim(), 10), EXPECTED_PART1);
    }

    #[test]
    fn test_day_14_part2() {
        assert_eq!(calculate(INPUT.trim(), 40), EXPECTED_PART2);
    }
}
