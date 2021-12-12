use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn make_graph(lines: &Vec<(&str, &str)>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (start, end) in lines {
        if *start != "end" && *end != "start" {
            graph
                .entry(String::from(*start))
                .or_insert(Vec::new())
                .push(String::from(*end));
        }
        if *start != "start" && *end != "end" {
            graph
                .entry(String::from(*end))
                .or_insert(Vec::new())
                .push(String::from(*start));
        }
    }
    return graph;
}

fn first_pass(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut todo: VecDeque<Vec<String>> = VecDeque::from([vec![String::from("start")]]);
    let mut done: Vec<Vec<String>> = Vec::new();

    while !todo.is_empty() {
        let path = todo.pop_front().unwrap();
        let last = path.last().unwrap();
        if *last == "end" {
            done.push(path);
            continue;
        }
        let neighbours = graph.get(&last.to_string());

        for neighbour in neighbours.unwrap_or(&Vec::new()) {
            if *neighbour == neighbour.to_uppercase() || !path.contains(&neighbour) {
                let mut new_path = path.to_vec();
                new_path.push(neighbour.to_string());
                todo.push_back(new_path);
            }
        }
    }
    return done.len();
}

fn second_pass(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut todo: VecDeque<(Vec<String>, bool)> =
        VecDeque::from([(vec![String::from("start")], false)]);
    let mut done: Vec<Vec<String>> = Vec::new();

    while !todo.is_empty() {
        let (path, is_double_small) = todo.pop_front().unwrap();
        let last = path.last().unwrap();
        if *last == "end" {
            done.push(path);
            continue;
        }
        let neighbours = graph.get(&last.to_string());
        for neighbour in neighbours.unwrap_or(&Vec::new()) {
            if *neighbour == neighbour.to_uppercase() || !path.contains(neighbour) {
                let mut new_path = path.to_vec();
                new_path.push(neighbour.to_string());
                todo.push_back((new_path, is_double_small));
            } else if !is_double_small && path.iter().filter(|p| *p == neighbour).count() == 1 {
                let mut new_path = path.to_vec();
                new_path.push(neighbour.to_string());
                todo.push_back((new_path, true));
            }
        }
    }
    return done.len();
}

pub fn run() {
    let input = crate::utils::read_input("day12.txt");
    let lines: Vec<(&str, &str)> = input
        .split("\n")
        .map(|line| line.trim().split_once("-").unwrap())
        .collect();
    let start = Instant::now();
    let graph = make_graph(&lines);
    println!("Graph setup {:?}", start.elapsed());
    println!("Part1 {} ({:?})", first_pass(&graph), start.elapsed());
    println!("Part2 {} ({:?})", second_pass(&graph), start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";
    const EXPECTED: usize = 226;
    const EXPECTED_PART2: usize = 3509;

    fn create_graph() -> HashMap<String, Vec<String>> {
        let lines: Vec<(&str, &str)> = INPUT
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| line.trim().split_once("-").unwrap())
            .collect();
        return make_graph(&lines);
    }

    #[test]
    fn test_day_12_part1() {
        let graph = create_graph();
        assert_eq!(first_pass(&graph), EXPECTED);
    }

    #[test]
    fn test_day_12_part2() {
        let graph = create_graph();
        assert_eq!(second_pass(&graph), EXPECTED_PART2);
    }
}
