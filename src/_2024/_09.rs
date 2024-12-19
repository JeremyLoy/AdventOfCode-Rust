use anyhow::Result;
use itertools::Itertools;

pub struct Disk {
    pub blocks: Vec<Block>,
    pub blocks_grouped: Vec<Vec<Block>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Block {
    File(u64),
    Empty,
}

pub fn parse(input: &str) -> Result<Disk> {
    let blocks_grouped: Vec<Vec<Block>> = input
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(id, chunk)| {
            let id = id as u64;
            let chunk_vec: Vec<u64> = chunk.map(|c| u64::from(c.to_digit(10).unwrap())).collect();
            if chunk_vec.len() == 2 {
                let (file_length, empty_length) = (chunk_vec[0], chunk_vec[1]);
                let mut result = vec![vec![Block::File(id); file_length as usize]];
                if empty_length > 0 {
                    result.push(vec![Block::Empty; empty_length as usize]);
                }
                result
            } else {
                let length = chunk_vec[0];
                vec![vec![Block::File(id); length as usize]]
            }
        })
        .collect();

    let blocks: Vec<Block> = blocks_grouped.iter().flatten().copied().collect();

    Ok(Disk {
        blocks,
        blocks_grouped,
    })
}

impl Disk {
    pub fn defrag_blocks(&mut self) -> u64 {
        let mut front = 0;
        let mut back = self.blocks.len() - 1;

        while front < back {
            while front < back && self.blocks[front] != Block::Empty {
                front += 1;
            }
            while front < back && self.blocks[back] == Block::Empty {
                back -= 1;
            }
            if front < back {
                self.blocks.swap(front, back);
            }
        }

        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                Block::File(id) => id * i as u64,
                Block::Empty => 0,
            })
            .sum()
    }

    pub fn defrag_groups(&mut self) -> u64 {
        for back_idx in (0..self.blocks_grouped.len()).rev() {
            let back = self.blocks_grouped.get(back_idx).unwrap();
            if back.iter().any(|&b| b == Block::Empty) {
                continue;
            }
            let mut front_idx = 0;
            while front_idx < back_idx {
                let front = self.blocks_grouped.get(front_idx).unwrap();
                let count_empty = front.iter().filter(|&b| *b == Block::Empty).count();
                if count_empty < back.len() {
                    front_idx += 1;
                    continue;
                }
                let idx_empty = front.iter().position(|&b| b == Block::Empty).unwrap();
                let back_len = back.len();
                let tmp = *back.first().unwrap();
                self.blocks_grouped
                    .get_mut(back_idx)
                    .unwrap()
                    .iter_mut()
                    .for_each(|b| *b = Block::Empty);
                let front = self.blocks_grouped.get_mut(front_idx).unwrap();
                front[idx_empty..idx_empty + back_len]
                    .iter_mut()
                    .for_each(|b| *b = tmp);
                break;
            }
        }

        self.blocks_grouped
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, b)| match b {
                Block::File(id) => id * i as u64,
                Block::Empty => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";
    const INPUT: &str = include_str!("../../input/2024/09.txt");

    #[test]
    fn test_1_sample() {
        let mut disk = parse(SAMPLE).unwrap();

        assert_eq!(disk.defrag_blocks(), 1_928);
    }

    #[test]
    fn test_1() {
        let mut disk = parse(INPUT).unwrap();

        assert_eq!(disk.defrag_blocks(), 6_201_130_364_722);
    }

    #[test]
    fn test_2_sample() {
        let mut disk = parse(SAMPLE).unwrap();

        assert_eq!(disk.defrag_groups(), 2_858);
    }

    #[test]
    fn test_2() {
        let mut disk = parse(INPUT).unwrap();

        assert_eq!(disk.defrag_groups(), 6_221_662_795_602);
    }
}
