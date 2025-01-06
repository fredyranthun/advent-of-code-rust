use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
enum Block {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Debug, Clone)]
struct Grid {
    positions: Vec<Vec<Block>>,
}

#[derive(Debug)]
struct GridErr {}
impl FromStr for Grid {
    type Err = GridErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Vec<Block>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Block::Obstacle,
                        '^' => Block::Guard,
                        _ => Block::Empty,
                    })
                    .collect()
            })
            .collect();

        Ok(Grid { positions })
    }
}

impl Grid {
    fn find_guard(&self) -> Option<Point> {
        for (row_idx, row) in self.positions.iter().enumerate() {
            for (col_idx, block) in row.iter().enumerate() {
                if *block == Block::Guard {
                    return Some(Point(col_idx, row_idx));
                }
            }
        }
        None
    }

    fn generate_new_with_obstacle(&self, point: Point) -> Self {
        let mut new_grid = self.clone();
        new_grid.set_block(&point, Block::Obstacle);

        new_grid
    }

    fn get_visited(&self) -> HashSet<Point> {
        let mut guard_position = self.find_guard().unwrap();
        let mut visited: HashSet<Point> = HashSet::new();

        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut direction_iter = directions.iter().cycle();

        loop {
            if let Some(&direction) = direction_iter.next() {
                let (pts, left_grid) =
                    self.move_guard_dir_until_obstacle(&guard_position, direction);
                let pts_set: HashSet<Point> = pts.iter().cloned().collect();
                visited = visited.union(&pts_set).cloned().collect();
                if left_grid {
                    break;
                }
                guard_position = pts.last().cloned().unwrap();
            }
        }

        visited
    }

    fn get_visited_except_guard_position(&self) -> HashSet<Point> {
        let g = self.find_guard().unwrap();
        let mut visited = self.get_visited();
        visited.remove(&g);
        visited
    }

    fn get_caught_in_a_loop(&self) -> bool {
        let mut guard_position = self.find_guard().unwrap();
        let mut visited: HashMap<Point, Vec<Direction>> = HashMap::new();

        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut direction_iter = directions.iter().cycle();

        loop {
            if let Some(&direction) = direction_iter.next() {
                let (pts, left_grid) =
                    self.move_guard_dir_until_obstacle(&guard_position, direction);
                for pt in &pts {
                    let pt_visited = visited.entry(*pt).or_insert(vec![]);
                    if pt_visited.contains(&direction) {
                        return true;
                    } else {
                        pt_visited.push(direction);
                    }
                }
                if left_grid {
                    return false;
                }
                guard_position = pts.clone().last().cloned().unwrap();
            }
        }
    }

    fn get_block(&self, point: &Point) -> &Block {
        &self.positions[point.1][point.0]
    }

    fn set_block(&mut self, point: &Point, block: Block) {
        self.positions[point.1][point.0] = block;
    }

    fn move_guard_dir_until_obstacle(
        &self,
        point: &Point,
        direction: Direction,
    ) -> (Vec<Point>, bool) {
        let mut visited_points = vec![];
        let mut left_grid = false;
        let mut curr_point = point.clone();
        let mut found_obstacle = false;

        while !left_grid && !found_obstacle {
            if let Some(pt) = curr_point.muv(self, &direction) {
                if self.get_block(&pt) == &Block::Obstacle {
                    found_obstacle = true;
                    visited_points.push(curr_point.clone());
                } else {
                    visited_points.push(curr_point.clone());
                    curr_point = pt.clone();
                }
            } else {
                left_grid = true;
                visited_points.push(curr_point.clone());
            }
        }

        (visited_points, left_grid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn muv(&self, grid: &Grid, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Up => self.up(grid),
            Direction::Down => self.down(grid),
            Direction::Right => self.right(grid),
            Direction::Left => self.left(grid),
        }
    }

    fn up(&self, _grid: &Grid) -> Option<Point> {
        if self.1 == 0 {
            None
        } else {
            Some(Point(self.0, self.1 - 1))
        }
    }

    fn down(&self, grid: &Grid) -> Option<Point> {
        if self.1 == grid.positions.len() - 1 {
            None
        } else {
            Some(Point(self.0, self.1 + 1))
        }
    }

    fn left(&self, _grid: &Grid) -> Option<Point> {
        if self.0 == 0 {
            None
        } else {
            Some(Point(self.0 - 1, self.1))
        }
    }

    fn right(&self, grid: &Grid) -> Option<Point> {
        if self.0 == grid.positions[0].len() - 1 {
            None
        } else {
            Some(Point(self.0 + 1, self.1))
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    Some(grid.get_visited().len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();
    let visited: Vec<Point> = grid
        .get_visited_except_guard_position()
        .into_iter()
        .collect();

    let count = visited
        .par_iter()
        .filter(|pt| {
            let new_grid = grid.generate_new_with_obstacle(**pt);
            new_grid.get_caught_in_a_loop()
        })
        .count();

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
