// Copyright 2021 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_into_string_groups;

const GRID_SIZE: usize = 5;

#[derive(Debug)]
struct MalformedBingoCard;

#[derive(Debug, Default)]
struct BingoField {
    value: u8,
    marked: bool,
}

impl Display for BingoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "[{:>2}]", self.value)
        } else {
            write!(f, " {:>2} ", self.value)
        }
    }
}

impl BingoField {
    fn new(value: u8) -> Self {
        BingoField {
            value,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

// card is defined to be a 5x5 grid
#[derive(Debug)]
struct BingoBoard {
    rows: [[BingoField; GRID_SIZE]; GRID_SIZE],
}

impl FromStr for BingoBoard {
    type Err = MalformedBingoCard;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [[BingoField; GRID_SIZE]; GRID_SIZE] = Default::default();
        for (i, row) in s.lines().enumerate() {
            for (j, val) in row.split_ascii_whitespace().enumerate() {
                let val = val.parse().map_err(|_| MalformedBingoCard)?;
                rows[i][j] = BingoField::new(val);
            }
        }

        Ok(BingoBoard { rows })
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for value in row {
                write!(f, "{}", value)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl BingoBoard {
    fn check_win_condition(&self) -> bool {
        for i in 0..GRID_SIZE {
            if self.check_row(i) {
                return true;
            }
            if self.check_column(i) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, row: usize) -> bool {
        self.rows[row].iter().all(|field| field.is_marked())
    }

    fn check_column(&self, column: usize) -> bool {
        for row in &self.rows {
            if !row[column].is_marked() {
                return false;
            }
        }
        true
    }

    fn mark_value(&mut self, value: u8) {
        for row in self.rows.iter_mut() {
            for field in row.iter_mut() {
                if field.value == value {
                    field.mark();
                    return;
                }
            }
        }
    }

    fn calculate_score(&self) -> usize {
        let mut score = 0;
        for row in self.rows.iter() {
            for field in row.iter() {
                if !field.is_marked() {
                    score += field.value as usize
                }
            }
        }
        score
    }
}

#[derive(Debug)]
struct BingoGame {
    currently_played: usize,
    drawn_numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn from_raw(input: &[String]) -> Self {
        assert!(input.len() > 2);
        let drawn_numbers = input[0]
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect();
        let boards = input
            .iter()
            .skip(1)
            .map(|val| val.parse().unwrap())
            .collect();

        BingoGame {
            currently_played: 0,
            drawn_numbers,
            boards,
        }
    }

    fn play_round(&mut self, drawn: u8) -> Option<usize> {
        for board in self.boards.iter_mut() {
            board.mark_value(drawn);
            if board.check_win_condition() {
                return Some(board.calculate_score() * drawn as usize);
            }
        }

        None
    }

    fn play_round_with_removal(&mut self, drawn: u8) -> Option<usize> {
        let mut to_remove = Vec::new();
        let boards = self.boards.len();
        for (i, board) in self.boards.iter_mut().enumerate().rev() {
            board.mark_value(drawn);
            if board.check_win_condition() {
                if boards == 1 {
                    return Some(board.calculate_score() * drawn as usize);
                } else {
                    to_remove.push(i)
                }
            }
        }

        for remove in to_remove {
            self.boards.remove(remove);
        }

        None
    }

    fn draw_number(&mut self) -> u8 {
        let value = self
            .drawn_numbers
            .get(self.currently_played)
            .expect("run out of values to draw");
        self.currently_played += 1;
        *value
    }

    fn play(&mut self) -> usize {
        loop {
            let drawn = self.draw_number();
            if let Some(winning_score) = self.play_round(drawn) {
                return winning_score;
            }
        }
    }

    fn play_until_final_board(&mut self) -> usize {
        loop {
            let drawn = self.draw_number();
            if let Some(winning_score) = self.play_round_with_removal(drawn) {
                return winning_score;
            }
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut game = BingoGame::from_raw(input);
    game.play()
}

fn part2(input: &[String]) -> usize {
    let mut game = BingoGame::from_raw(input);
    game.play_until_final_board()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_into_string_groups, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            r#"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19"#
                .to_string(),
            r#"3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#
                .to_string(),
            r#"14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#
                .to_string(),
        ];

        let expected = 4512;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            r#"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19"#
                .to_string(),
            r#"3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#
                .to_string(),
            r#"14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#
                .to_string(),
        ];

        let expected = 1924;

        assert_eq!(expected, part2(&input))
    }
}
