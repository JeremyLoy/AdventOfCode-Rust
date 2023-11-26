use itertools::Itertools;
use std::collections::HashSet;
use std::convert::identity;

#[derive(Debug)]
pub struct BingoBoard([[BingoCell; 5]; 5]);

#[derive(Debug, Copy, Clone)]
pub enum BingoCell {
    Marked(i32),
    Unmarked(i32),
}
impl BingoBoard {
    // Extracting cell parsing logic to a separate function
    fn parse_cell(number_str: &str) -> Option<BingoCell> {
        let number = number_str.parse::<i32>().ok()?;
        Some(BingoCell::Unmarked(number))
    }
    pub fn parse(input: impl Iterator<Item = String>) -> Option<Self> {
        let mut board = [[BingoCell::Unmarked(0); 5]; 5];
        for (i, line) in input.enumerate() {
            for (j, number_str) in line.split_whitespace().enumerate() {
                board[i][j] = Self::parse_cell(number_str)?;
            }
        }
        Some(BingoBoard(board))
    }

    pub fn parse_batch(lines: impl Iterator<Item = String>) -> Vec<Self> {
        lines
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .chunks(5)
            .into_iter()
            .map(BingoBoard::parse)
            .filter_map(identity)
            .collect()
    }

    pub fn calculate_score(&self, last_call: i32) -> i32 {
        let mut score = 0;
        for row in self.0.iter() {
            for cell in row.iter() {
                if let BingoCell::Unmarked(value) = cell {
                    score += value;
                }
            }
        }
        score * last_call
    }

    pub fn mark(&mut self, number: i32) {
        for row in self.0.iter_mut() {
            for cell in row.iter_mut() {
                if let BingoCell::Unmarked(value) = cell {
                    if *value == number {
                        *cell = BingoCell::Marked(number);
                    }
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for row in self.0.iter() {
            if row.iter().all(|&cell| matches!(cell, BingoCell::Marked(_))) {
                return true;
            }
        }
        for col in 0..5 {
            if self
                .0
                .iter()
                .all(|row| matches!(row[col], BingoCell::Marked(_)))
            {
                return true;
            }
        }
        false
    }
}

pub fn parse_calls_and_bingo_boards(
    mut lines: impl Iterator<Item = String>,
) -> (Vec<i32>, Vec<BingoBoard>) {
    let calls = lines.next().unwrap_or_default();
    let calls = calls
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let boards = BingoBoard::parse_batch(lines);
    (calls, boards)
}

pub fn play_bingo(calls: Vec<i32>, mut boards: Vec<BingoBoard>) -> Vec<i32> {
    let mut winning_scores = Vec::new();
    let mut past_winners = HashSet::new();

    for call in calls {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(call);
            if board.is_winner() {
                winning_scores.push(board.calculate_score(call));
                past_winners.insert(i);
            }
        }
        let mut i: usize = 0;
        boards.retain(|_| {
            let keep = !past_winners.contains(&i);
            i += 1;
            keep
        });
        past_winners.clear()
    }

    winning_scores
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::{to_lines, Input::*};
    #[test]
    fn test_4_1_sample() {
        let input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.first().unwrap(), 4_512);
    }

    #[test]
    fn test_4_1() {
        let input = to_lines(Path("input/2021/4.txt"));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.first().unwrap(), 8_136);
    }

    #[test]
    fn test_4_2_sample() {
        let input = to_lines(Raw("
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.last().unwrap(), 1_924);
    }

    #[test]
    fn test_4_2() {
        let input = to_lines(Path("input/2021/4.txt"));

        let (calls, boards) = parse_calls_and_bingo_boards(input);

        let winning_scores = play_bingo(calls, boards);

        assert_eq!(*winning_scores.last().unwrap(), 12_738);
    }
}
