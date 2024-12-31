advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut first_column: Vec<u64> = Vec::new();
    let mut second_column: Vec<u64> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                first_column.push(num1);
                second_column.push(num2);
            }
        }
    }

    (first_column, second_column)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut first_column, mut second_column) = parse_input(input);

    first_column.sort();
    second_column.sort();

    let answer = first_column
        .into_iter()
        .zip(second_column)
        .map(|(num1, num2)| num1.abs_diff(num2))
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut first_column, mut second_column) = parse_input(input);

    first_column.sort();
    second_column.sort();

    let answer = first_column
        .iter()
        .map(|n| (second_column.iter().filter(|n2| *n2 == n).count() as u64) * n)
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
