use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Grid {
    positions: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct GridErr {}
impl FromStr for Grid {
    type Err = GridErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Vec<u32>> = s
            .lines()
            .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        Ok(Grid { positions })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn find_points_next(pt: Point, value: u32, grid: &Grid) -> Vec<Point> {
    let directions = vec![
        (0, 1),  // Down
        (0, -1), // Up
        (1, 0),  // Right
        (-1, 0), // Left
    ];

    let mut next_points = vec![];

    for (dx, dy) in directions {
        let new_x = pt.0 as isize + dx;
        let new_y = pt.1 as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_x < grid.positions[0].len() && new_y < grid.positions.len() {
                if grid.positions[new_y][new_x] == value {
                    next_points.push(Point(new_x, new_y));
                }
            }
        }
    }

    next_points
}

fn get_final_points_multiple(pts: Vec<Point>, cur_value: u32, grid: &Grid) -> HashSet<Point> {
    let mut hs = HashSet::new();

    pts.iter().for_each(|pt| {
        let new_hs = get_final_points_single(*pt, cur_value, grid);
        hs = hs.union(&new_hs).cloned().collect();
    });

    hs
}

fn get_final_points_single(pt: Point, cur_value: u32, grid: &Grid) -> HashSet<Point> {
    if cur_value == 9 {
        let mut hash_set = HashSet::new();
        hash_set.insert(pt);
        return hash_set;
    }

    let next_pts = find_points_next(pt, cur_value + 1, grid);

    if next_pts.is_empty() {
        let hash_set = HashSet::new();
        return hash_set;
    }

    get_final_points_multiple(next_pts, cur_value + 1, grid)
}

fn count_trails_single(pt: Point, cur_value: u32, grid: &Grid) -> usize {
    if cur_value == 9 {
        return 1;
    }

    let next_pts = find_points_next(pt, cur_value + 1, grid);

    if next_pts.is_empty() {
        return 0;
    }

    count_trails_multiple(next_pts, cur_value + 1, grid)
}

fn count_trails_multiple(pts: Vec<Point>, cur_value: u32, grid: &Grid) -> usize {
    let mut sum = 0;

    pts.iter().for_each(|pt| {
        let inc = count_trails_single(*pt, cur_value, grid);
        sum += inc;
    });

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    let mut initial_pts = vec![];

    for (y, row) in grid.positions.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                initial_pts.push(Point(x, y));
            }
        }
    }

    let mut sum = 0;
    for pt in initial_pts {
        let final_pts = get_final_points_single(pt, 0, &grid);
        sum += final_pts.len();
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    let mut initial_pts = vec![];

    for (y, row) in grid.positions.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                initial_pts.push(Point(x, y));
            }
        }
    }

    let mut sum = 0;
    for pt in initial_pts {
        let count = count_trails_single(pt, 0, &grid);
        sum += count;
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
