fn first_pass(contents: &str) -> usize {
    return 32;
}

pub fn run() {
    let contents = crate::utils::read_input("day14.txt");
    println!("Part1: {}", first_pass(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = &"";
    const EXPECTED_PART1: usize = 0;

    #[test]
    fn test_day_x_part1() {
        assert_eq!(first_pass(INPUT), EXPECTED_PART1);
    }
}
