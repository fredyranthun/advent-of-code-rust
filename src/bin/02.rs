advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut parsed: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        let parsed_line: Vec<u64> = line
            .split_whitespace()
            .filter_map(|str| str.parse().ok())
            .collect();
        parsed.push(parsed_line);
    }

    parsed
}

fn all_increasing(values: Vec<u64>) -> bool {
    values
        .windows(2)
        .all(|w| w[1] > w[0] && w[1] - w[0] > 0 && w[1] - w[0] <= 3)
}

fn all_decreasing(values: Vec<u64>) -> bool {
    values
        .windows(2)
        .all(|w| w[0] > w[1] && w[0] - w[1] > 0 && w[0] - w[1] <= 3)
}

fn generate_variations(values: Vec<u64>) -> Vec<Vec<u64>> {
    let mut variations: Vec<Vec<u64>> = Vec::new();

    for i in 0..values.len() {
        let mut variation = values.clone();
        variation.remove(i);
        variations.push(variation);
    }

    variations.push(values);

    variations
}

fn all_decreasing_with_dampener(values: Vec<u64>) -> bool {
    let variations = generate_variations(values);
    variations
        .iter()
        .any(|v| all_increasing(v.to_vec()) || all_decreasing(v.to_vec()))
}

pub fn part_one(input: &str) -> Option<u64> {
    let p = parse_input(input);
    let n = p
        .iter()
        .filter(|v| all_increasing(v.to_vec()) || all_decreasing(v.to_vec()))
        .count();

    Some(n as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let p = parse_input(input);
    let n = p
        .iter()
        .filter(|v| all_decreasing_with_dampener(v.to_vec()))
        .count();

    Some(n as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(301));
    }
}
