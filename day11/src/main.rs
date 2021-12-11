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

use std::collections::HashSet;
use std::ops::{Index, IndexMut};
use utils::execute;
use utils::input_read::read_input_lines;

#[derive(Debug)]
struct SquidGrid {
    inner: [[u8; 10]; 10],
}

impl Index<(usize, usize)> for SquidGrid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.inner[y][x]
    }
}

impl IndexMut<(usize, usize)> for SquidGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.inner[y][x]
    }
}

impl SquidGrid {
    fn parse(raw: &[String]) -> Self {
        let mut rows: [[u8; 10]; 10] = Default::default();
        for (i, line) in raw.iter().enumerate() {
            let mut row: [u8; 10] = Default::default();
            for (j, digit) in line.chars().enumerate() {
                row[j] = digit.to_digit(10).unwrap() as u8;
            }
            rows[i] = row;
        }

        SquidGrid { inner: rows }
    }

    fn flash(&mut self, octopus: (usize, usize), flashed: &mut HashSet<(usize, usize)>) {
        if flashed.contains(&octopus) {
            return;
        }
        flashed.insert(octopus);

        // (x - 1), (y - 1)
        // (x - 1), (y)
        // (x - 1), (y + 1)
        // (x), (y + 1)
        // (x), (y - 1)
        // (x + 1), (y - 1)
        // (x + 1), (y)
        // (x + 1), (y + 1)

        let x = octopus.0;
        let y = octopus.1;

        let x_minus_1 = if x > 0 { Some(x - 1) } else { None };
        let x_plus_1 = if x < 9 { Some(x + 1) } else { None };
        let y_minus_1 = if y > 0 { Some(y - 1) } else { None };
        let y_plus_1 = if y < 9 { Some(y + 1) } else { None };

        let adjacent = &[
            (x_minus_1, y_minus_1),
            (x_minus_1, Some(y)),
            (x_minus_1, y_plus_1),
            (Some(x), y_plus_1),
            (Some(x), y_minus_1),
            (x_plus_1, y_minus_1),
            (x_plus_1, Some(y)),
            (x_plus_1, y_plus_1),
        ];

        for (x, y) in adjacent {
            if let Some(x) = *x {
                if let Some(y) = *y {
                    self[(x, y)] += 1;

                    // if adjacent's energy went above 9 and it hasn't flashed during this step,
                    // it should flash
                    if self[(x, y)] > 9 && !flashed.contains(&(x, y)) {
                        self.flash((x, y), flashed);
                    }
                }
            }
        }
    }

    fn flash_all(&mut self, to_flash: Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut flashed = HashSet::new();

        for octopus in to_flash {
            self.flash(octopus, &mut flashed);
        }

        flashed
    }

    fn simulate_step(&mut self) -> usize {
        let mut to_flash = Vec::new();
        // First, the energy level of each octopus increases by 1.
        for (y, row) in self.inner.iter_mut().enumerate() {
            for (x, squid) in row.iter_mut().enumerate() {
                *squid += 1;

                if *squid > 9 {
                    to_flash.push((x, y));
                }
            }
        }

        // Then, any octopus with an energy level greater than 9 flashes.
        let flashed = self.flash_all(to_flash);

        let flashed_count = flashed.len();

        for (x, y) in flashed {
            // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
            self[(x, y)] = 0;
        }
        flashed_count
    }

    fn naive_simulation(&mut self, steps: usize) -> usize {
        let mut flashed = 0;

        for _ in 0..steps {
            flashed += self.simulate_step();
        }
        flashed
    }

    fn wait_for_sync(&mut self) -> usize {
        let mut step = 0;
        loop {
            step += 1;
            if self.simulate_step() == 100 {
                return step;
            }
        }
    }
}

fn part1(input: &[String]) -> usize {
    SquidGrid::parse(input).naive_simulation(100)
}

fn part2(input: &[String]) -> usize {
    SquidGrid::parse(input).wait_for_sync()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute("input", read_input_lines, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];

        let expected = 1656;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];

        let expected = 195;

        assert_eq!(expected, part2(&input))
    }
}
