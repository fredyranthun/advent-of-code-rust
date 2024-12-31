advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let mut results = vec![];
    for (_, [gr1, gr2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((gr1.parse::<u64>().unwrap(), gr2.parse::<u64>().unwrap()));
    }

    let answer = results.iter().map(|(n1, n2)| n1 * n2).sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)|do\(\)|don't\(\)").unwrap();

    let results: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut on = true;

    print!("{:?}", results);

    // let mut operations = vec![];
    let mut s = String::new();

    for m in results.iter() {
        if *m == "do()" {
            on = true;
        } else if *m == "don't()" {
            on = false;
        } else if m.starts_with("mul(") && on {
            s.push_str(m);
        }
    }

    part_one(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167_090_022));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(89_823_704));
    }
}
