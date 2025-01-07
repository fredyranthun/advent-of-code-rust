use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn count_digits(mut n: u64) -> i32 {
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }

    d
}

fn split_digits(n: u64) -> (u64, u64) {
    let n_digits = count_digits(n);
    let half_digits = n_digits / 2;

    let divisor = 10_u64.pow(half_digits as u32);
    let first_part = n / divisor;
    let second_part = n % divisor;

    (first_part, second_part)
}

fn iterate_times(initial_stones: Vec<u64>, count: i32) -> Option<u64> {
    let mut map: HashMap<u64, usize> = HashMap::new();
    for stone in initial_stones {
        *map.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..count {
        let mut new_map = HashMap::new();
        for (stone, count) in map {
            if stone == 0 {
                *new_map.entry(1).or_insert(0) += count;
            } else if count_digits(stone) % 2 == 0 {
                let (a, b) = split_digits(stone);
                *new_map.entry(a).or_insert(0) += count;
                *new_map.entry(b).or_insert(0) += count;
            } else {
                *new_map.entry(stone * 2024).or_insert(0) += count;
            }
        }
        map = new_map;
    }

    let sum = map.iter().fold(0, |acc, (_, v)| acc + (*v));

    Some(sum as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = parse_input(input);
    let count = 25;

    iterate_times(initial_stones, count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = parse_input(input);
    let count = 75;

    iterate_times(initial_stones, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55_312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
