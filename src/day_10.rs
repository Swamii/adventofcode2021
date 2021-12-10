use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

const NEWLINE: &char = &'\n';

fn first_pass(chars: &Vec<char>) -> usize {
    let closers = HashMap::<_, _>::from_iter(IntoIter::new([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]));
    let scores = HashMap::<_, _>::from_iter(IntoIter::new([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]));

    let mut expected: Vec<char> = Vec::new();
    let mut read_until_next_line = false;
    let mut score = 0;

    for char in chars {
        if char == NEWLINE {
            read_until_next_line = false;
            expected.clear();
        }
        if read_until_next_line || char == NEWLINE {
            continue;
        }
        if let Some(closer_char) = closers.get(&char) {
            expected.push(*closer_char);
        } else {
            let exp = expected.pop().unwrap();

            if char != &exp {
                read_until_next_line = true;
                score += scores[&char];
            }
        }
    }
    return score;
}

fn second_pass(chars: &Vec<char>) -> usize {
    let closers = HashMap::<_, _>::from_iter(IntoIter::new([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]));
    let score_map =
        HashMap::<_, _>::from_iter(IntoIter::new([(')', 1), (']', 2), ('}', 3), ('>', 4)]));

    let mut expected: Vec<char> = Vec::new();
    let mut read_until_next_line = false;
    let mut scores: Vec<usize> = Vec::new();

    for char in chars {
        if char == NEWLINE {
            read_until_next_line = false;
            let mut score = 0;
            while !expected.is_empty() {
                let next = expected.pop().unwrap();
                score = score * 5 + score_map[&next];
            }
            if score > 0 {
                scores.push(score);
            }
        }
        if read_until_next_line || char == NEWLINE {
            continue;
        }
        if let Some(closer_char) = closers.get(&char) {
            expected.push(*closer_char);
        } else {
            let exp = expected.pop().unwrap();

            if char != &exp {
                expected.clear();
                read_until_next_line = true;
            }
        }
    }

    scores.sort();
    return scores[scores.len() / 2];
}

pub fn run() {
    let contents = crate::utils::read_input("day10.txt");
    let chars: Vec<char> = contents.chars().collect();

    println!("Part1: {}", first_pass(&chars));
    println!("Part2: {}", second_pass(&chars));
}
