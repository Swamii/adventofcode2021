use std::collections::VecDeque;

const BOARD_LENGTH: usize = 5;

#[derive(Debug)]
struct BingoBoard {
    id: usize,
    rows: Vec<Vec<u8>>,
}

impl BingoBoard {
    pub fn from(id: usize, board: &str) -> Self {
        let mut rows: Vec<Vec<u8>> = Vec::new();

        for row_raw in board.split("\n") {
            let mut row = Vec::new();
            for column in row_raw.split_whitespace() {
                row.push(column.trim().parse::<u8>().unwrap());
            }
            rows.push(row);
        }

        return BingoBoard { id, rows: rows };
    }

    fn columns(&self) -> Vec<Vec<u8>> {
        let mut columns: Vec<Vec<u8>> = Vec::new();
        for _ in 0..BOARD_LENGTH {
            columns.push(Vec::new());
        }
        for row in &self.rows {
            for (column_index, column_value) in row.iter().enumerate() {
                columns[column_index].push(*column_value);
            }
        }

        return columns;
    }
    fn unchecked_values(&self, numbers: &[u8]) -> Vec<u8> {
        let mut unchecked: Vec<u8> = Vec::new();
        for row in &self.rows {
            for value in row {
                if !numbers.contains(value) {
                    unchecked.push(*value);
                }
            }
        }
        return unchecked;
    }

    pub fn check(&self, numbers: &[u8]) -> Option<Vec<u8>> {
        for direction in [&self.rows, &self.columns()] {
            for values in direction {
                if values
                    .iter()
                    .filter(|val| numbers.contains(val))
                    .collect::<Vec<&u8>>()
                    .len()
                    == BOARD_LENGTH
                {
                    return Some(self.unchecked_values(&numbers));
                }
            }
        }
        return None;
    }
}

struct WinningBoard {
    unchecked_sum: usize,
    deciding_number: u8,
    board_id: usize,
}

fn check_board(boards: &Vec<BingoBoard>, numbers: &Vec<u8>) -> Vec<WinningBoard> {
    let mut winners: Vec<WinningBoard> = Vec::new();

    for index in BOARD_LENGTH..numbers.len() {
        for board in boards {
            if winners
                .iter()
                .map(|w| w.board_id)
                .collect::<Vec<usize>>()
                .contains(&board.id)
            {
                continue;
            }
            let current_numbers = &numbers[..index];
            match board.check(current_numbers) {
                Some(unchecked_values) => {
                    let deciding_number = &numbers[index - 1];
                    let unchecked_sum: usize = unchecked_values.iter().map(|v| *v as usize).sum();
                    winners.push(WinningBoard {
                        unchecked_sum: unchecked_sum,
                        deciding_number: *deciding_number,
                        board_id: board.id,
                    });
                }
                None => (),
            };
        }
    }
    return winners;
}

pub fn run() {
    let contents = crate::utils::read_input("day4.txt");
    let mut blocks: VecDeque<&str> = contents.split("\n\n").collect();
    let numbers: Vec<u8> = blocks
        .pop_front()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = blocks
        .iter()
        .enumerate()
        .map(|(index, block)| BingoBoard::from(index, block))
        .collect();

    let winners = check_board(&boards, &numbers);
    let first_winner = &winners[0];
    println!(
        "Part1: {} * {} = {}",
        first_winner.deciding_number,
        first_winner.unchecked_sum,
        (first_winner.deciding_number as usize) * first_winner.unchecked_sum
    );
    let last_winner = winners.last().unwrap();
    println!(
        "Part2: {} * {} = {}",
        last_winner.deciding_number,
        last_winner.unchecked_sum,
        (last_winner.deciding_number as usize) * last_winner.unchecked_sum
    );
}
