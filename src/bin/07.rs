use std::collections::HashMap;

advent_of_code::solution!(7);
#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Add,
    Concatenate,
}

impl Operation {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Multiply => a * b,
            Operation::Add => a + b,
            Operation::Concatenate => concatenate_numbers(a, b),
        }
    }
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    let mut b_copy = b;
    let mut digits = 0;
    while b_copy > 0 {
        b_copy /= 10;
        digits += 1;
    }
    a * 10_u64.pow(digits) + b
}

fn parse_input(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut parsed: HashMap<u64, Vec<u64>> = HashMap::new();
    input.lines().for_each(|l| {
        let mut parts = l.split(":");
        let result = parts.next().unwrap().parse::<u64>().ok().unwrap();
        let operands: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|op| op.parse::<u64>().ok().unwrap())
            .collect();
        parsed.insert(result, operands);
    });

    parsed
}

fn check_results(result: u64, operands: Vec<u64>, operations: Vec<Operation>) -> bool {
    let results: Vec<u64> = operands.iter().fold(vec![], |acc, &x| {
        if acc.len() == 0 {
            return vec![x];
        }
        operations
            .iter()
            .flat_map(|op| {
                acc.iter().filter_map(move |&y| {
                    let r = op.apply(y, x);
                    if r > result {
                        None
                    } else {
                        Some(r)
                    }
                })
            })
            .collect()
    });

    results.contains(&&result)
}

pub fn part_one(input: &str) -> Option<u64> {
    let functions = parse_input(input);
    let operations = vec![Operation::Multiply, Operation::Add];
    let mut total = 0;

    for (result, operands) in functions {
        if check_results(result, operands, operations.clone()) {
            total += result
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let functions = parse_input(input);
    let operations = vec![Operation::Multiply, Operation::Add, Operation::Concatenate];
    let mut total = 0;

    for (result, operands) in functions {
        if check_results(result, operands, operations.clone()) {
            total += result
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3_749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11_387));
    }
}
