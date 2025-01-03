use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

type Update = Vec<u64>;
type PageOrderingRules = HashMap<u64, Vec<u64>>;

fn parse_input(input: &str) -> (PageOrderingRules, Vec<Update>) {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    assert!(blocks.len() == 2);

    let rules: Vec<Vec<u64>> = blocks[0]
        .lines()
        .map(|line| line.split("|").map(|s| s.parse::<u64>().unwrap()).collect())
        .collect();
    let updates: Vec<Update> = blocks[1]
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<u64>().unwrap()).collect())
        .collect();

    let mut rules_map: PageOrderingRules = HashMap::new();

    rules.iter().for_each(|rule| {
        rules_map.entry(rule[0]).or_insert(vec![]).push(rule[1]);
    });

    (rules_map, updates)
}

fn valid_update(update: Update, rules_map: &PageOrderingRules) -> bool {
    for i in 1..update.len() {
        if let Some(not_before_values) = rules_map.get(&update[i]) {
            if not_before_values.iter().any(|&v| update[..i].contains(&v)) {
                return false;
            }
        }
    }
    true
}

fn sort_update(mut update: Update, rules_map: &PageOrderingRules) -> Vec<u64> {
    update.sort_unstable_by(|a, b| {
        if let Some(not_before_values) = rules_map.get(a) {
            if not_before_values.contains(b) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    });

    update
}

pub fn part_one(input: &str) -> Option<u64> {
    let (page_ordering_rules, updates) = parse_input(input);

    let sum: u64 = updates
        .iter()
        .filter(|&update| valid_update(update.clone(), &page_ordering_rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (page_ordering_rules, updates) = parse_input(input);

    let sum: u64 = updates
        .iter()
        .filter(|&update| !valid_update(update.clone(), &page_ordering_rules))
        .map(|update| sort_update(update.clone(), &page_ordering_rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5_391));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6142));
    }
}
