use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Grid {
    positions: Vec<Vec<char>>,
}

#[derive(Debug)]
struct GridErr {}
impl FromStr for Grid {
    type Err = GridErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        Ok(Grid { positions })
    }
}

impl Grid {
    fn width(&self) -> usize {
        if self.positions.is_empty() {
            return 0;
        }

        self.positions[0].len()
    }

    fn height(&self) -> usize {
        self.positions.len()
    }

    fn get_antennas(&self) -> HashMap<char, Vec<Point>> {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

        for (y, row) in self.positions.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if *ch != '.' {
                    antennas
                        .entry(*ch)
                        .or_insert(vec![])
                        .push(Point(x as isize, y as isize));
                }
            }
        }

        antennas
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

fn generate_antinodes(a1: Point, a2: Point) -> (Point, Point) {
    (
        Point(a2.0 * 2 - a1.0, a2.1 * 2 - a1.1),
        Point(a1.0 * 2 - a2.0, a1.1 * 2 - a2.1),
    )
}

fn generate_multiple_antinodes(
    a1: Point,
    a2: Point,
    width: usize,
    height: usize,
) -> HashSet<Point> {
    let delta_x: usize = (a2.0 - a1.0).abs() as usize;
    let delta_y: usize = (a2.1 - a1.1).abs() as usize;

    let gcd = gcd(delta_x, delta_y);

    let dx = (a2.0 - a1.0) / (gcd as isize);
    let dy = (a2.1 - a1.1) / (gcd as isize);

    let mut antinodes = HashSet::new();
    let mut new_pt = a1.clone();

    while is_in_grid(new_pt, width, height) {
        antinodes.insert(new_pt.clone());
        new_pt.0 = new_pt.0 + dx;
        new_pt.1 = new_pt.1 + dy;
    }

    let mut new_pt = a2.clone();

    while is_in_grid(new_pt, width, height) {
        antinodes.insert(new_pt.clone());
        new_pt.0 = new_pt.0 - dx;
        new_pt.1 = new_pt.1 - dy;
    }

    antinodes
}

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn is_in_grid(pt: Point, width: usize, height: usize) -> bool {
    pt.0 >= 0 && pt.0 < (width as isize) && pt.1 >= 0 && pt.1 < (height as isize)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    let width = grid.width();
    let height = grid.height();
    let antennas = grid.get_antennas();
    let mut antinodes: HashSet<_> = HashSet::new();

    // generate the antinodes and save them in a set
    for (_, points) in antennas {
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                let (antinode1, antinode2) = generate_antinodes(points[i], points[j]);
                if is_in_grid(antinode1, width, height) {
                    antinodes.insert(antinode1);
                }

                if is_in_grid(antinode2, width, height) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    let width = grid.width();
    let height = grid.height();
    let antennas = grid.get_antennas();
    let mut antinodes: HashSet<_> = HashSet::new();

    // generate the antinodes and save them in a set
    for (_, points) in antennas {
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                let new_antinodes =
                    generate_multiple_antinodes(points[i], points[j], width, height);
                antinodes.extend(new_antinodes);
            }
        }
    }

    Some(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
