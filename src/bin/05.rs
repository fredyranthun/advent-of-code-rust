use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

advent_of_code::solution!(5);

type Update = Vec<u64>;
type Graph<T> = HashMap<T, HashSet<T>>;

pub struct State<T> {
    depends_on: Graph<T>,
    dependents: Graph<T>,
}

pub fn add_edge<T>(graph: &mut Graph<T>, from: T, to: T)
where
    T: Eq + Hash + Copy,
{
    graph
        .entry(from)
        .and_modify(|pointees| {
            pointees.insert(to);
        })
        .or_insert_with(|| {
            let mut s = HashSet::new();
            s.insert(to);
            s
        });
}

impl<T> State<T>
where
    T: Eq + std::hash::Hash,
{
    pub fn get_dependents(self: &Self, dependency: &T) -> Option<&HashSet<T>> {
        self.dependents.get(dependency)
    }

    pub fn is_resolved(self: &Self) -> bool {
        self.depends_on.is_empty()
    }
}

fn parse_input(input: &str) -> (Graph<u64>, Vec<Update>) {
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

    let mut graph: Graph<u64> = HashMap::new();

    rules.iter().for_each(|rule| {
        add_edge(&mut graph, rule[0], rule[1]);
    });

    (graph, updates)
}

fn valid_update(update: Update, graph: &Graph<u64>) -> bool {
    for i in 1..update.len() {
        if let Some(not_before_values) = graph.get(&update[i]) {
            if not_before_values
                .intersection(&update[..i].iter().cloned().collect::<HashSet<_>>())
                .count()
                != 0
            {
                return false;
            }
        }
    }
    true
}

fn sort_update(mut update: Update, graph: &Graph<u64>) -> Vec<u64> {
    update.sort_unstable_by(|a, b| {
        if let Some(not_before_values) = graph.get(a) {
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
