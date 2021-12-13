use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn fold_it(contents: &str, first_fold_only: bool) -> HashSet<Coordinate> {
    let (coords_raw, folding_raw) = contents.split_once("\n\n").unwrap();
    let mut coords: HashSet<Coordinate> = coords_raw
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

    for (fold_direction, fold_length) in folding.iter() {
        if *fold_direction == "x" {
            coords = coords
                .iter()
                .map(|coord| Coordinate {
                    x: if coord.x < *fold_length {
                        coord.x
                    } else {
                        fold_length - (coord.x - fold_length)
                    },
                    y: coord.y,
                })
                .collect();
        } else {
            coords = coords
                .iter()
                .map(|coord| Coordinate {
                    x: coord.x,
                    y: if coord.y < *fold_length {
                        coord.y
                    } else {
                        fold_length - (coord.y - fold_length)
                    },
                })
                .collect();
        }
        if first_fold_only {
            break;
        }
    }
    return coords;
}

fn first_pass(contents: &str) -> usize {
    let last_folded = fold_it(contents, true);
    return last_folded.len();
}

fn display(rows: &HashSet<Coordinate>) -> String {
    let mut numbers: String = String::new();
    for y in 0..rows.iter().max_by_key(|c| c.y).unwrap().y + 1 {
        for x in 0..rows.iter().max_by_key(|c| c.x).unwrap().x + 1 {
            numbers += if rows.contains(&Coordinate { x, y }) {
                "#"
            } else {
                " "
            }
        }
        numbers += "\n";
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
#####";

    #[test]
    fn test_day_13_part1() {
        assert_eq!(first_pass(INPUT), EXPECTED_PART1);
    }

    #[test]
    fn test_day_13_part2() {
        assert_eq!(second_pass(INPUT), EXPECTED_PART2);
    }
}
