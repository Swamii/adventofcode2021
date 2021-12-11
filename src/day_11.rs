use std::collections::HashSet;

struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn get_value(&self, coord: &Coordinate) -> u8 {
        return self.rows[coord.y][coord.x];
    }

    fn set_value(&mut self, coord: &Coordinate, value: u8) {
        self.rows[coord.y][coord.x] = value;
    }

    fn increment_value(&mut self, coord: &Coordinate) -> u8 {
        let val = self.get_value(coord);
        let new_val = if val == 9 { 0 } else { val + 1 };
        self.set_value(coord, new_val);
        return new_val;
    }

    fn get_column_len(&self) -> usize {
        return self.rows.len();
    }

    fn get_row_len(&self) -> usize {
        return self.rows[0].len();
    }

    fn get_coordinates(&self) -> Vec<Coordinate> {
        let mut coords: Vec<Coordinate> = Vec::new();
        for row in 0..self.get_column_len() {
            for column in 0..self.get_row_len() {
                coords.push(Coordinate { y: row, x: column });
            }
        }
        return coords;
    }

    fn get_adjacent_coordinates(&self, coord: &Coordinate) -> Vec<Coordinate> {
        let mut adjacent: Vec<Coordinate> = Vec::new();
        let max_y = (self.get_row_len() - 1) as isize;
        let max_x = (self.get_column_len() - 1) as isize;

        for x in -1 as isize..2 {
            for y in -1 as isize..2 {
                let y_d = y + coord.y as isize;
                let x_d = x + coord.x as isize;
                if (y == 0 && x == 0) || x_d < 0 || y_d < 0 || x_d > max_x || y_d > max_y {
                    continue;
                }
                adjacent.push(Coordinate {
                    x: x_d as usize,
                    y: y_d as usize,
                });
            }
        }

        return adjacent;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

pub fn run() {
    let contents = crate::utils::read_input("day11.txt");
    let rows: Vec<Vec<u8>> = contents
        .split("\n")
        .map(|line| {
            return line
                .chars()
                .map(|height_raw| height_raw.to_string().parse::<u8>().unwrap())
                .collect();
        })
        .collect();

    let mut map = Map { rows };
    let coords = &map.get_coordinates();
    let mut total_flashed = 0;
    let mut first_ultraflash = 0;

    for step in 0.. {
        let mut to_flash: Vec<Coordinate> = Vec::new();
        for coord in coords {
            let new_val = map.increment_value(&coord);
            if new_val == 0 {
                to_flash.push(coord.clone());
            }
        }

        let mut flashed: HashSet<Coordinate> = HashSet::new();
        while !to_flash.is_empty() {
            let flash_coord = to_flash.pop().unwrap();
            flashed.insert(flash_coord.clone());
            for inner_coord in &map.get_adjacent_coordinates(&flash_coord) {
                if !to_flash.contains(&inner_coord) && !flashed.contains(&inner_coord) {
                    let new_val = map.increment_value(&inner_coord);
                    if new_val == 0 {
                        to_flash.push(inner_coord.clone());
                    }
                }
            }
        }

        if step < 100 {
            total_flashed += flashed.len();
        }
        if flashed.len() == coords.len() {
            first_ultraflash = step + 1;
        }
        if step >= 100 && first_ultraflash != 0 {
            break;
        }
    }

    println!("Part1: {}", total_flashed);
    println!("Part2: {}", first_ultraflash);
}
