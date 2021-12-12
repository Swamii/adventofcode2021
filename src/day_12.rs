use std::collections::HashMap;

const PATH_DELIMITER: &str = ",";

struct Graph {
    graph: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from(lines: &Vec<(&str, &str)>) -> Graph {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for (start, end) in lines {
            if *start != "end" {
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
        return Graph { graph };
    }
}

fn first_pass(graph: &Graph) -> usize {
    let mut todo: Vec<(String, String)> = vec![(String::from("start"), String::new())];
    let mut done: Vec<String> = Vec::new();

    while !todo.is_empty() {
        let (next, path) = todo.pop().unwrap();
        let current_path = if path.is_empty() {
            next.to_string()
        } else {
            path + PATH_DELIMITER + &next
        };
        if next == "end" {
            done.push(current_path.to_string());
            continue;
        }
        let neighbours = graph.graph.get(&next);

        for neighbour in neighbours.unwrap_or(&Vec::new()) {
            if *neighbour == neighbour.to_lowercase() && current_path.contains(neighbour) {
                continue;
            }
            todo.push((neighbour.to_string(), current_path.to_string()));
        }
    }
    return done.len();
}

fn has_more_than_one_ocurrence_of_cave(string: &str, cave: &str) -> bool {
    let mut ocurrences: HashMap<String, usize> = HashMap::new();
    for chars in string.split(PATH_DELIMITER) {
        if chars == chars.to_lowercase() {
            *ocurrences.entry(chars.to_string()).or_insert(0) += 1;
        }
    }
    return ocurrences.iter().any(|(_, count)| *count > 1) && ocurrences.contains_key(cave);
}

fn second_pass(graph: &Graph) -> usize {
    let mut todo: Vec<(String, String)> = vec![(String::from("start"), String::new())];
    let mut done: Vec<String> = Vec::new();

    while !todo.is_empty() {
        let (next, path) = todo.pop().unwrap();
        let current_path = if path.is_empty() {
            next.to_string()
        } else {
            path + PATH_DELIMITER + &next
        };
        if next == "end" {
            done.push(current_path.to_string());
            continue;
        }
        let neighbours = match graph.graph.get(&next) {
            Some(n) => n,
            None => continue,
        };

        for neighbour in neighbours {
            if *neighbour == "start"
                || (*neighbour == neighbour.to_lowercase()
                    && neighbour != "end"
                    && has_more_than_one_ocurrence_of_cave(&current_path, neighbour))
            {
                continue;
            }
            todo.push((neighbour.to_string(), current_path.to_string()));
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
    let graph = Graph::from(&lines);
    println!("Part1 {}", first_pass(&graph));
    println!("Part2 {}", second_pass(&graph));
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

    fn create_graph() -> Graph {
        let lines: Vec<(&str, &str)> = INPUT
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| line.trim().split_once("-").unwrap())
            .collect();
        return Graph::from(&lines);
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
