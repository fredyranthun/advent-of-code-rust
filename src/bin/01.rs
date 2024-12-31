use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (left, right) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            left.push(line[0]);
            right.push(line[1]);

            (left, right)
        });

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut first_column, mut second_column) = parse_input(input);

    first_column.sort_unstable();
    second_column.sort_unstable();

    let answer = first_column
        .into_iter()
        .zip(second_column)
        .map(|(num1, num2)| num1.abs_diff(num2))
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (first_column, second_column) = parse_input(input);
    let mut map: HashMap<u64, u64> = HashMap::new();
    second_column
        .iter()
        .for_each(|k| *map.entry(*k).or_insert(0) += 1);

    let answer: u64 = first_column
        .iter()
        .map(|k| k * (*map.entry(*k).or_default() as u64))
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_879_048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21_024_792));
    }
}
