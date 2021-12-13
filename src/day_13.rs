use std::cmp;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn fold_it(contents: &str, first_fold_only: bool) -> Vec<Vec<bool>> {
    let (coords_raw, folding_raw) = contents.split_once("\n\n").unwrap();
    let coords: Vec<Coordinate> = coords_raw
        .split("\n")
        .map(|coord| {
            let (x, y) = coord.split_once(",").unwrap();
            return Coordinate {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            };
        })
        .collect();
    let folding: Vec<(&str, usize)> = folding_raw
        .split("\n")
        .map(|fold| {
            let (direction, position) = fold
                .split(" ")
                .skip(2)
                .next()
                .unwrap()
                .split_once("=")
                .unwrap();
            return (direction, position.parse::<usize>().unwrap());
        })
        .collect();

    // We can't just use the largest coord as the end. We also need to make sure
    // that there's enough width/height for the largest fold (which can be larger).
    let max_x = cmp::max(
        coords.iter().max_by_key(|c| c.x).unwrap().x,
        folding
            .iter()
            .filter(|(dir, _)| *dir == "x")
            .max_by_key(|(_, position)| position)
            .unwrap()
            .1
            * 2,
    );
    let max_y = cmp::max(
        coords.iter().max_by_key(|c| c.y).unwrap().y,
        folding
            .iter()
            .filter(|(dir, _)| *dir == "y")
            .max_by_key(|(_, position)| position)
            .unwrap()
            .1
            * 2,
    );
    let mut rows: Vec<Vec<bool>> = Vec::new();
    for y in 0..max_y + 1 {
        let mut row = Vec::new();
        for x in 0..max_x + 1 {
            row.push(coords.contains(&Coordinate { x, y }));
        }
        rows.push(row);
    }
    let mut last_folded: Vec<Vec<bool>> = rows.to_vec();

    for (fold_direction, fold_length) in folding.iter() {
        let mut folded: Vec<Vec<bool>> = Vec::new();
        if *fold_direction == "x" {
            for row in &last_folded {
                let mut new_row = Vec::new();
                for (c1, c2) in row[0..*fold_length]
                    .iter()
                    .zip(row[*fold_length + 1..row.len()].iter().rev())
                {
                    new_row.push(*c1 || *c2);
                }
                folded.push(new_row);
            }
        } else {
            for (first_row, second_row) in last_folded[0..*fold_length].iter().zip(
                last_folded[*fold_length + 1..last_folded.len()]
                    .iter()
                    .rev(),
            ) {
                let mut new_row = Vec::new();
                for (r1, r2) in first_row.iter().zip(second_row.iter()) {
                    new_row.push(*r1 || *r2);
                }
                folded.push(new_row);
            }
        }
        last_folded = folded;
        if first_fold_only {
            break;
        }
    }
    return last_folded;
}

fn first_pass(contents: &str) -> usize {
    let last_folded = fold_it(contents, true);
    return last_folded.iter().flatten().filter(|v| **v).count();
}

fn display(rows: &Vec<Vec<bool>>) -> String {
    let mut numbers: String = String::new();
    for row in rows {
        numbers = numbers
            + &row
                .iter()
                .map(|r| match r {
                    true => '#',
                    false => ' ',
                })
                .collect::<String>()
            + "\n";
    }
    return numbers.to_string().trim().to_string();
}

fn second_pass(contents: &str) -> String {
    let last_folded = fold_it(contents, false);
    return display(&last_folded);
}

pub fn run() {
    let contents = crate::utils::read_input("day13.txt");
    println!("Part1: {}", first_pass(&contents));
    println!("Part2:\n{}", second_pass(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = &"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    const EXPECTED_PART1: usize = 17;
    const EXPECTED_PART2: &str = "#####
#   #
#   #
#   #
#####
     
     ";

    #[test]
    fn test_day_13_part1() {
        assert_eq!(first_pass(INPUT), EXPECTED_PART1);
    }

    #[test]
    fn test_day_13_part2() {
        assert_eq!(second_pass(INPUT), EXPECTED_PART2);
    }
}
