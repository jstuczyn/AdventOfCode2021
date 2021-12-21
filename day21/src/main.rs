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

use std::cmp::max;
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug, Copy, Clone)]
enum Player {
    One,
    Two,
}

#[derive(Debug, Clone, Copy)]
struct DiracDice {
    total_rolled: usize,
    last_roll: usize,
    player1_position: Position,
    player2_position: Position,

    player1_score: usize,
    player2_score: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    fn into_quantum(self) -> QuantumDiracDice {
        let mut game = QuantumDiracDice {
            simulated_universes: Default::default(),
            p1_wins: 0,
            p2_wins: 0,
        };
        game.simulated_universes.insert(
            UniverseState {
                player1_position: self.player1_position,
                player1_score: self.player1_score,
                player2_position: self.player2_position,
                player2_score: self.player2_score,
            },
            1,
        );

        game
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct UniverseState {
    player1_position: Position,
    player1_score: usize,
    player2_position: Position,
    player2_score: usize,
}

impl UniverseState {
    fn add_throw(&mut self, throw: usize, player: Player) -> bool {
        match player {
            Player::One => {
                self.player1_position.move_pawn(throw);
                self.player1_score += self.player1_position.0;
                if self.player1_score >= 21 {
                    return true;
                }
            }
            Player::Two => {
                self.player2_position.move_pawn(throw);
                self.player2_score += self.player2_position.0;
                if self.player2_score >= 21 {
                    return true;
                }
            }
        }
        false
    }
}

struct QuantumDiracDice {
    simulated_universes: HashMap<UniverseState, usize>,

    p1_wins: usize,
    p2_wins: usize,
}

impl QuantumDiracDice {
    // possible outcomes of dice roll:
    // 1-1-1 = 3
    // 1-1-2 = 4
    // 1-1-3 = 5
    // 1-2-1 = 4
    // 1-2-2 = 5
    // 1-2-3 = 6
    // 1-3-1 = 5
    // 1-3-2 = 6
    // 1-3-3 = 7
    // 2-1-1 = 4
    // 2-1-2 = 5
    // 2-1-3 = 6
    // 2-2-1 = 5
    // 2-2-2 = 6
    // 2-2-3 = 7
    // 2-3-1 = 6
    // 2-3-2 = 7
    // 2-3-3 = 8
    // 3-1-1 = 5
    // 3-1-2 = 6
    // 3-1-3 = 7
    // 3-2-1 = 6
    // 3-2-2 = 7
    // 3-2-3 = 8
    // 3-3-1 = 7
    // 3-3-2 = 8
    // 3-3-3 = 9

    // so each 3 rolls produces:
    // 1 universe with sum 3
    // 3 universes with sum 4
    // 6 universes with sum 5
    // 7 universes with sum 6
    // 6 universes with sum 7
    // 3 universes with sum 8
    // 1 universe with sum 9

    fn add_wins(&mut self, count: usize, player: Player) {
        match player {
            Player::One => self.p1_wins += count,
            Player::Two => self.p2_wins += count,
        }
    }

    fn play_round(&mut self, player: Player) -> bool {
        for (universe_state, count) in mem::take(&mut self.simulated_universes) {
            let mut sum3 = universe_state;
            if sum3.add_throw(3, player) {
                self.add_wins(count, player);
            } else {
                *self.simulated_universes.entry(sum3).or_default() += count
            }

            let mut sum4 = universe_state;
            if sum4.add_throw(4, player) {
                self.add_wins(3 * count, player);
            } else {
                *self.simulated_universes.entry(sum4).or_default() += 3 * count
            }

            let mut sum5 = universe_state;
            if sum5.add_throw(5, player) {
                self.add_wins(6 * count, player);
            } else {
                *self.simulated_universes.entry(sum5).or_default() += 6 * count
            }

            let mut sum6 = universe_state;
            if sum6.add_throw(6, player) {
                self.add_wins(7 * count, player);
            } else {
                *self.simulated_universes.entry(sum6).or_default() += 7 * count
            }

            let mut sum7 = universe_state;
            if sum7.add_throw(7, player) {
                self.add_wins(6 * count, player);
            } else {
                *self.simulated_universes.entry(sum7).or_default() += 6 * count
            }

            let mut sum8 = universe_state;
            if sum8.add_throw(8, player) {
                self.add_wins(3 * count, player);
            } else {
                *self.simulated_universes.entry(sum8).or_default() += 3 * count
            }

            let mut sum9 = universe_state;
            if sum9.add_throw(9, player) {
                self.add_wins(count, player);
            } else {
                *self.simulated_universes.entry(sum9).or_default() += count
            }
        }

        self.simulated_universes.is_empty()
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
    let mut quantum_game = game.into_quantum();
    loop {
        if quantum_game.play_round(Player::One) {
            return max(quantum_game.p1_wins, quantum_game.p2_wins);
        }
        if quantum_game.play_round(Player::Two) {
            return max(quantum_game.p1_wins, quantum_game.p2_wins);
        }
    }
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

    #[test]
    fn part2_sample_input() {
        let game = DiracDice {
            total_rolled: 0,
            last_roll: 0,
            player1_position: Position(4),
            player2_position: Position(8),
            player1_score: 0,
            player2_score: 0,
        };

        let expected = 444356092776315;
        assert_eq!(expected, part2(game))
    }
}
