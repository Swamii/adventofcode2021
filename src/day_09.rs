use std::collections::HashSet;

const WALL_VALUE: u8 = 9;

struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn get_value(&self, coord: &Coordinate) -> u8 {
        return self.rows[coord.y][coord.x];
    }

    fn get_column_len(&self) -> usize {
        return self.rows.len();
    }

    fn get_row_len(&self) -> usize {
        return self.rows[0].len();
    }

    fn get_adjacent_coordinates(&self, coord: &Coordinate) -> Vec<Coordinate> {
        let mut adjacent: Vec<Coordinate> = Vec::new();
        if coord.y > 0 {
            adjacent.push(Coordinate {
                y: coord.y - 1,
                x: coord.x,
            });
        }
        if coord.y < self.get_row_len() - 1 {
            adjacent.push(Coordinate {
                y: coord.y + 1,
                x: coord.x,
            });
        }
        if coord.x > 0 {
            adjacent.push(Coordinate {
                y: coord.y,
                x: coord.x - 1,
            });
        }
        if coord.x < self.get_column_len() - 1 {
            adjacent.push(Coordinate {
                y: coord.y,
                x: coord.x + 1,
            });
        }
        return adjacent;
    }

    fn get_adjacent(&self, row: usize, column: usize) -> Vec<u8> {
        return self
            .get_adjacent_coordinates(&Coordinate { x: column, y: row })
            .iter()
            .map(|coord| self.rows[coord.y][coord.x])
            .collect();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn first_pass(map: &Map) -> usize {
    let mut sum: usize = 0;
    for y in 0..map.get_column_len() {
        for x in 0..map.get_row_len() {
            let value = map.rows[x][y];
            let adjacent = map.get_adjacent(x, y);
            let has_bigger_adjacent = adjacent.iter().any(|item| item <= &value);
            if !has_bigger_adjacent {
                sum += value as usize + 1;
            }
        }
    }
    return sum;
}

fn traverse(map: &Map, traversed: &mut HashSet<Coordinate>, coord: &Coordinate) -> Vec<Coordinate> {
    let adjacent = map.get_adjacent_coordinates(coord);
    let mut basin: Vec<Coordinate> = Vec::new();
    traversed.insert(coord.clone());
    basin.push(coord.clone());
    for inner_coord in &adjacent {
        if map.get_value(&inner_coord) == WALL_VALUE || traversed.contains(&inner_coord) {
            continue;
        }
        basin.extend(traverse(&map, traversed, &inner_coord));
    }
    return basin;
}

fn second_pass(map: &Map) -> usize {
    let traversed: &mut HashSet<Coordinate> = &mut HashSet::new();
    let mut basins: Vec<Vec<Coordinate>> = Vec::new();

    for y in 0..map.get_column_len() {
        for x in 0..map.get_row_len() {
            let coord = Coordinate { x, y };
            if map.get_value(&coord) == WALL_VALUE || traversed.contains(&coord) {
                continue;
            }
            basins.push(traverse(&map, traversed, &coord));
        }
    }

    basins.sort_by_key(|basin| -(basin.len() as isize));
    return basins
        .iter()
        .take(3)
        .fold(1, |result, basin| result * basin.len());
}

pub fn run() {
    let contents = crate::utils::read_input("day09.txt");
    let rows: Vec<Vec<u8>> = contents
        .split("\n")
        .map(|line| {
            return line
                .chars()
                .map(|height_raw| height_raw.to_string().parse::<u8>().unwrap())
                .collect();
        })
        .collect();

    let map = Map { rows };

    println!("Part1: {}", first_pass(&map));
    println!("Part2: {}", second_pass(&map));
}
