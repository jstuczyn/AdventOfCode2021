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

use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug, Clone, Copy)]
struct DiracDice {
    total_rolled: usize,
    last_roll: usize,
    player1_position: Position,
    player2_position: Position,

    player1_score: usize,
    player2_score: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position(usize);

impl Position {
    fn move_pawn(&mut self, val: usize) {
        self.0 += val;
        self.0 = (self.0 - 1) % 10 + 1
    }
}

impl FromStr for DiracDice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let p1 = lines
            .next()
            .unwrap()
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap();
        let p2 = lines
            .next()
            .unwrap()
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap();

        Ok(DiracDice {
            total_rolled: 0,
            last_roll: 0,
            player1_position: Position(p1),
            player2_position: Position(p2),
            player1_score: 0,
            player2_score: 0,
        })
    }
}

impl DiracDice {
    fn roll_deterministic_die_once(&mut self) -> usize {
        if self.last_roll == 100 {
            self.last_roll = 1;
        } else {
            self.last_roll += 1;
        }
        self.total_rolled += 1;
        self.last_roll
    }

    fn roll_deterministic_die_three_times(&mut self) -> usize {
        if self.last_roll <= 97 {
            let res = 3 * self.last_roll + 6;
            self.total_rolled += 3;
            self.last_roll += 3;
            res
        } else {
            self.roll_deterministic_die_once()
                + self.roll_deterministic_die_once()
                + self.roll_deterministic_die_once()
        }
    }

    fn play_round(&mut self, player: u8) -> bool {
        let throw = self.roll_deterministic_die_three_times();
        if player == 1 {
            self.player1_position.move_pawn(throw);
            self.player1_score += self.player1_position.0;
            if self.player1_score >= 1000 {
                return true;
            }
        } else if player == 2 {
            self.player2_position.move_pawn(throw);
            self.player2_score += self.player2_position.0;
            if self.player2_score >= 1000 {
                return true;
            }
        } else {
            unreachable!("invalid player")
        }
        false
    }
}

fn part1(mut game: DiracDice) -> usize {
    loop {
        if game.play_round(1) {
            return game.total_rolled * game.player2_score;
        }
        if game.play_round(2) {
            return game.total_rolled * game.player1_score;
        }
    }
}

fn part2(game: DiracDice) -> usize {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_struct("input", read_parsed, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_pawn() {
        let mut pos1 = Position(4);
        let mut pos2 = Position(8);

        pos1.move_pawn(1 + 2 + 3);
        assert_eq!(Position(10), pos1);

        pos2.move_pawn(4 + 5 + 6);
        assert_eq!(Position(3), pos2);

        pos1.move_pawn(7 + 8 + 9);
        assert_eq!(Position(4), pos1);

        pos2.move_pawn(10 + 11 + 12);
        assert_eq!(Position(6), pos2);

        pos1.move_pawn(13 + 14 + 15);
        assert_eq!(Position(6), pos1);

        pos2.move_pawn(16 + 17 + 18);
        assert_eq!(Position(7), pos2);

        pos1.move_pawn(19 + 20 + 21);
        assert_eq!(Position(6), pos1);

        pos2.move_pawn(22 + 23 + 24);
        assert_eq!(Position(6), pos2);
    }

    #[test]
    fn part1_sample_input() {
        let game = DiracDice {
            total_rolled: 0,
            last_roll: 0,
            player1_position: Position(4),
            player2_position: Position(8),
            player1_score: 0,
            player2_score: 0,
        };

        let expected = 739785;
        assert_eq!(expected, part1(game))
    }
}
