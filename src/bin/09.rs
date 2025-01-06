advent_of_code::solution!(9);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Block {
    File(u32),
    FreeSpace,
}

#[derive(Debug, PartialEq, Eq)]
enum DiskBlocks {
    File(u32, u32),
    FreeSpace(u32),
}

fn compress(disk: &mut Vec<Block>) -> &Vec<Block> {
    let mut i = 0;
    let mut j = disk.len() - 1;

    while i < j {
        if disk[i] != Block::FreeSpace {
            i += 1;
        } else {
            while disk[j] == Block::FreeSpace {
                j -= 1;
            }
            disk.swap(i, j);
        }
    }

    disk
}

fn compress_and_transform(disk: &mut Vec<DiskBlocks>) -> Vec<Block> {
    let mut j = disk.len() - 1;

    while j > 0 {
        match disk[j] {
            DiskBlocks::FreeSpace(_) => {
                j -= 1;
                continue;
            }
            DiskBlocks::File(_, size_j) => {
                let mut i = 0;
                while i < j {
                    match disk[i] {
                        DiskBlocks::File(_, _) => {
                            i += 1;
                            continue;
                        }
                        DiskBlocks::FreeSpace(size_i) => {
                            if size_i < size_j {
                                i += 1;
                                continue;
                            } else {
                                let remaining_space = DiskBlocks::FreeSpace(size_i - size_j);
                                let space_to_move = DiskBlocks::FreeSpace(size_j);
                                disk.swap(i, j);
                                disk[j] = space_to_move;
                                disk.insert(i + 1, remaining_space);
                                break;
                            }
                        }
                    }
                }
                j -= 1;
            }
        }
    }

    disk.iter()
        .flat_map(|db| match db {
            DiskBlocks::File(id, size) => vec![Block::File(*id); *size as usize],
            DiskBlocks::FreeSpace(size) => vec![Block::FreeSpace; *size as usize],
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut file = true; // either is file or blank space;
    let mut id = 0;
    let mut disk: Vec<Block> = vec![];
    for ch in input.chars() {
        if let Some(mut digit) = ch.to_digit(10) {
            if file {
                while digit > 0 {
                    disk.push(Block::File(id));
                    digit -= 1;
                }
                id += 1;
            } else {
                while digit > 0 {
                    disk.push(Block::FreeSpace);
                    digit -= 1;
                }
            }
            file = !file;
        }
    }

    compress(&mut disk);
    let sum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            Block::File(n) => Some((*n as usize) * i),
            Block::FreeSpace => None,
        })
        .sum();
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut file = true; // either is file or blank space;
    let mut id = 0;
    let mut disk: Vec<DiskBlocks> = vec![];
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            if file {
                disk.push(DiskBlocks::File(id, digit));
                id += 1;
            } else {
                disk.push(DiskBlocks::FreeSpace(digit));
            }
            file = !file;
        }
    }

    let new_disk = compress_and_transform(&mut disk);
    let sum: usize = new_disk
        .iter()
        .enumerate()
        .filter_map(|(i, block)| match block {
            Block::File(n) => Some((*n as usize) * i),
            Block::FreeSpace => None,
        })
        .sum();
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
