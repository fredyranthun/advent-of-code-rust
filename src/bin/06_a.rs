use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Block {
    NotVisited,
    Visited,
    Obstacle,
    GuardFacingUp,
    GuardFacingDown,
    GuardFacingRight,
    GuardFacingLeft,
}

#[derive(Debug)]
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
                        '^' => Block::GuardFacingUp,
                        _ => Block::NotVisited,
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
                if *block == Block::GuardFacingUp {
                    return Some(Point(col_idx, row_idx));
                }
            }
        }
        None
    }

    fn get_block(&self, point: &Point) -> &Block {
        &self.positions[point.1][point.0]
    }

    fn set_block(&mut self, point: &Point, block: Block) {
        self.positions[point.1][point.0] = block;
    }

    fn move_guard_dir(&mut self, point: &Point, direction: Direction) -> Option<Point> {
        let (next_point, obstacle_block, visited_block, guard_block) = match direction {
            Direction::Up => (
                point.up(self),
                Block::GuardFacingRight,
                Block::GuardFacingUp,
                point.up(self),
            ),
            Direction::Down => (
                point.down(self),
                Block::GuardFacingLeft,
                Block::GuardFacingDown,
                point.down(self),
            ),
            Direction::Left => (
                point.left(self),
                Block::GuardFacingUp,
                Block::GuardFacingLeft,
                point.left(self),
            ),
            Direction::Right => (
                point.right(self),
                Block::GuardFacingDown,
                Block::GuardFacingRight,
                point.right(self),
            ),
        };

        if let Some(pt) = next_point {
            if self.get_block(&pt) == &Block::Obstacle {
                self.set_block(point, obstacle_block);
                Some(Point(point.0, point.1))
            } else {
                self.set_block(point, Block::Visited);
                self.set_block(&pt, visited_block);
                guard_block
            }
        } else {
            None
        }
    }

    fn move_guard(&mut self, point: Point) -> Option<Point> {
        let guard = self.get_block(&point);
        match guard {
            Block::GuardFacingUp => self.move_guard_dir(&point, Direction::Up),
            Block::GuardFacingDown => self.move_guard_dir(&point, Direction::Down),
            Block::GuardFacingLeft => self.move_guard_dir(&point, Direction::Left),
            Block::GuardFacingRight => self.move_guard_dir(&point, Direction::Right),
            _ => None,
        }
    }

    fn count_visited(&self) -> u64 {
        let mut count = 0;
        for row in self.positions.iter() {
            for block in row.iter() {
                if *block == Block::Visited {
                    count += 1;
                }
            }
        }

        // Consider the guard position
        count + 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

impl Point {
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
    let mut grid = Grid::from_str(input).unwrap();
    let mut guard_position = grid.find_guard();

    while let Some(pt) = guard_position {
        guard_position = grid.move_guard(pt);
    }

    Some(grid.count_visited())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
