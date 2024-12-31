use std::{str::FromStr, vec};

advent_of_code::solution!(4);

#[derive(Debug)]
struct WordSearchGrid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
struct WordSearchGridErr {}
impl FromStr for WordSearchGrid {
    type Err = WordSearchGridErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| line.chars().collect()).collect();
        Ok(WordSearchGrid { grid })
    }
}

impl WordSearchGrid {
    fn count_occurrences(&self) -> usize {
        self.grid.iter().fold(0, |acc, line| {
            acc + line
                .windows(4)
                .map(|v| v.iter().collect::<String>())
                .filter(|s| s == "XMAS" || s == "SAMX")
                .count()
        })
    }

    fn count_occurrences_columns(&self) -> usize {
        let transposed = self.transpose();
        transposed.count_occurrences()
    }

    fn count_occurrences_diagonals_45(&self) -> usize {
        let diagonals = self.diagonals_45();
        diagonals.count_occurrences()
    }

    fn count_occurrences_diagonals_135(&self) -> usize {
        let diagonals = self.diagonals_135();
        diagonals.count_occurrences()
    }

    fn is_x_mas(&self, point: Point) -> bool {
        if point.0 == 0
            || point.1 == 0
            || point.0 == self.grid[0].len() - 1
            || point.1 == self.grid.len() - 1
        {
            return false;
        }

        let first_diagonal = (self.grid[point.1 - 1][point.0 - 1] == 'M'
            && self.grid[point.1 + 1][point.0 + 1] == 'S')
            || (self.grid[point.1 - 1][point.0 - 1] == 'S'
                && self.grid[point.1 + 1][point.0 + 1] == 'M');

        let second_diagonal = (self.grid[point.1 - 1][point.0 + 1] == 'M'
            && self.grid[point.1 + 1][point.0 - 1] == 'S')
            || (self.grid[point.1 - 1][point.0 + 1] == 'S'
                && self.grid[point.1 + 1][point.0 - 1] == 'M');

        first_diagonal && second_diagonal
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if *ch == 'A' && self.is_x_mas(Point(j, i)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn transpose(&self) -> Self {
        // assert!(!self.grid.is_empty());
        if self.grid.len() == 0 {
            return WordSearchGrid { grid: vec![] };
        }
        let len = self.grid[0].len();
        let grid = (0..len)
            .map(|i| {
                self.grid
                    .iter()
                    .map(|inner| inner[i])
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        WordSearchGrid { grid }
    }

    fn diagonals_45(&self) -> Self {
        if self.grid.len() == 0 {
            return WordSearchGrid { grid: vec![] };
        }
        let diagonals_number = self.grid[0].len() * 2 - 1;
        //   [0][1][2]
        //[0] 1  2  3
        //[1] 4  5  6
        //[2] 7  8  9
        // [[1] [2, 4] [3, 5, 7] [6, 8] [9]]
        let grid = (0..diagonals_number)
            .map(|d: usize| {
                self.grid
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, row)| {
                        if d < idx || d - idx >= row.len() {
                            None
                        } else {
                            Some(row[d - idx])
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        WordSearchGrid { grid }
    }

    fn diagonals_135(&self) -> Self {
        if self.grid.len() == 0 {
            return WordSearchGrid { grid: vec![] };
        }
        let diagonals_number = self.grid[0].len() * 2 - 1;
        //   [0][1][2]
        //[0] 1  2  3
        //[1] 4  5  6
        //[2] 7  8  9
        // [[3] [2, 6] [1, 5, 9] [4, 8] [7]]
        let grid = (0..diagonals_number)
            .map(|d: usize| {
                self.grid
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, row)| {
                        let r = row.len();
                        if d < idx || d - idx >= r {
                            None
                        } else {
                            Some(row[r - 1 - (d - idx)])
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        WordSearchGrid { grid }
    }
}

struct Point(usize, usize);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = WordSearchGrid::from_str(input).unwrap();
    let count = grid.count_occurrences()
        + grid.count_occurrences_columns()
        + grid.count_occurrences_diagonals_45()
        + grid.count_occurrences_diagonals_135();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = WordSearchGrid::from_str(input).unwrap();

    Some(grid.count_x_mas() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2_685));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2048));
    }
}
