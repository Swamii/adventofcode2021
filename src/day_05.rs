use std::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: u32,
    y: u32,
}
impl Coordinate {
    fn from(string: &str) -> Coordinate {
        let (x, y) = string.split_once(",").unwrap();
        return Coordinate {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        };
    }
}

#[derive(Debug)]
struct Line {
    from: Coordinate,
    to: Coordinate,
}
impl Line {
    fn from(string: &str) -> Line {
        let (from_coord, to_coord) = string.split_once(" -> ").unwrap();
        return Line {
            from: Coordinate::from(from_coord),
            to: Coordinate::from(to_coord),
        };
    }

    pub fn coordinates(&self, vertical_only: bool) -> Vec<Coordinate> {
        let mut coordinates: Vec<Coordinate> = Vec::new();
        let min_x = cmp::min(self.from.x, self.to.x);
        let max_x = cmp::max(self.from.x, self.to.x);
        let min_y = cmp::min(self.from.y, self.to.y);
        let max_y = cmp::max(self.from.y, self.to.y);
        if self.from.x == self.to.x || self.from.y == self.to.y {
            for x in min_x..max_x + 1 {
                for y in min_y..max_y + 1 {
                    coordinates.push(Coordinate { x, y });
                }
            }
        } else if !vertical_only {
            let x_iter = if self.from.x <= self.to.x {
                (min_x..max_x + 1).collect::<Vec<u32>>()
            } else {
                (min_x..max_x + 1).rev().collect::<Vec<u32>>()
            };
            let y_iter = if self.from.y <= self.to.y {
                (min_y..max_y + 1).collect::<Vec<u32>>()
            } else {
                (min_y..max_y + 1).rev().collect::<Vec<u32>>()
            };
            for (x, y) in x_iter.iter().zip(y_iter) {
                coordinates.push(Coordinate { x: *x, y: y });
            }
        }
        return coordinates;
    }
}

fn count_coords(lines: &Vec<Coordinate>) -> HashMap<&Coordinate, u32> {
    let mut position_counter: HashMap<&Coordinate, u32> = HashMap::new();
    for line in lines {
        *position_counter.entry(line).or_insert(0) += 1;
    }
    return position_counter;
}

fn first_pass(raw_lines: &Vec<Line>) -> u32 {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    for line in raw_lines.iter() {
        for coord in line.coordinates(true) {
            coordinates.push(coord);
        }
    }

    return count_coords(&coordinates)
        .iter()
        .filter(|(_, count)| *count > &1)
        .fold(0, |result, _| result + 1);
}

fn second_pass(raw_lines: &Vec<Line>) -> u32 {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    for line in raw_lines.iter() {
        for coord in line.coordinates(false) {
            coordinates.push(coord);
        }
    }
    return count_coords(&coordinates)
        .iter()
        .filter(|(_, count)| *count > &1)
        .fold(0, |result, _| result + 1);
}

pub fn run() {
    let contents = crate::utils::read_input("day05.txt");
    let raw_lines: Vec<Line> = contents.split("\n").map(|line| Line::from(line)).collect();

    let count_overlapping_first = first_pass(&raw_lines);
    println!(
        "Part1: {} count with more than 2 overlapping coordinates",
        count_overlapping_first
    );

    let count_overlapping_second = second_pass(&raw_lines);
    println!(
        "Part2: {} count with more than 2 overlapping coordinates",
        count_overlapping_second
    );
}
