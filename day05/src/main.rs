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

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;

#[derive(Debug)]
struct MalformedVentLine;

#[derive(Debug)]
struct VentLine {
    start: (i32, i32),
    end: (i32, i32),
}

impl Display for VentLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

impl FromStr for VentLine {
    type Err = MalformedVentLine;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(" -> ");
        let start = coords.next().ok_or(MalformedVentLine)?;
        let mut x_y1 = start.split(',');
        let x1 = x_y1
            .next()
            .ok_or(MalformedVentLine)?
            .parse()
            .map_err(|_| MalformedVentLine)?;
        let y1 = x_y1
            .next()
            .ok_or(MalformedVentLine)?
            .parse()
            .map_err(|_| MalformedVentLine)?;

        let end = coords.next().ok_or(MalformedVentLine)?;
        let mut x_y2 = end.split(',');
        let x2 = x_y2
            .next()
            .ok_or(MalformedVentLine)?
            .parse()
            .map_err(|_| MalformedVentLine)?;
        let y2 = x_y2
            .next()
            .ok_or(MalformedVentLine)?
            .parse()
            .map_err(|_| MalformedVentLine)?;

        Ok(VentLine {
            start: (x1, y1),
            end: (x2, y2),
        })
    }
}

impl VentLine {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    // in the case of this task and our input, all slopes are guaranteed to be integers
    fn slope(&self) -> Option<i32> {
        let dx = self.end.0 - self.start.0;
        if dx == 0 {
            return None;
        }
        let dy = self.end.1 - self.start.1;
        Some(dy / dx)
    }

    fn interception(&self, slope: i32) -> i32 {
        self.start.1 - slope * self.start.0
    }

    fn covered_points(&self) -> Vec<(i32, i32)> {
        match self.slope() {
            Some(m) => {
                let b = self.interception(m);
                if self.start.0 > self.end.0 {
                    (self.end.0..=self.start.0)
                        .map(|x| (x, m * x + b))
                        .rev()
                        .collect()
                } else {
                    (self.start.0..=self.end.0)
                        .map(|x| (x, m * x + b))
                        .collect()
                }
            }
            None => {
                if self.start.1 > self.end.1 {
                    (self.end.1..=self.start.1)
                        .map(|y| (self.start.0, y))
                        .rev()
                        .collect()
                } else {
                    (self.start.1..=self.end.1)
                        .map(|y| (self.start.0, y))
                        .collect()
                }
            }
        }
    }
}

fn part1(input: &[VentLine]) -> usize {
    let mut coverage: HashMap<_, i32> = HashMap::new();

    input
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .for_each(|line| {
            for covered_point in line.covered_points() {
                *coverage.entry(covered_point).or_default() += 1i32;
            }
        });

    coverage.values().filter(|&&count| count >= 2).count()
}

fn part2(input: &[VentLine]) -> usize {
    let mut coverage: HashMap<_, i32> = HashMap::new();

    input.iter().for_each(|line| {
        for covered_point in line.covered_points() {
            *coverage.entry(covered_point).or_default() += 1i32;
        }
    });

    coverage.values().filter(|&&count| count >= 2).count()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_cover() {
        let line1 = VentLine {
            start: (1, 1),
            end: (1, 3),
        };
        assert_eq!(vec![(1, 1), (1, 2), (1, 3)], line1.covered_points());

        let line2 = VentLine {
            start: (9, 7),
            end: (7, 7),
        };
        assert_eq!(vec![(9, 7), (8, 7), (7, 7)], line2.covered_points());
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "0,9 -> 5,9".parse().unwrap(),
            "8,0 -> 0,8".parse().unwrap(),
            "9,4 -> 3,4".parse().unwrap(),
            "2,2 -> 2,1".parse().unwrap(),
            "7,0 -> 7,4".parse().unwrap(),
            "6,4 -> 2,0".parse().unwrap(),
            "0,9 -> 2,9".parse().unwrap(),
            "3,4 -> 1,4".parse().unwrap(),
            "0,0 -> 8,8".parse().unwrap(),
            "5,5 -> 8,2".parse().unwrap(),
        ];

        let expected = 5;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "0,9 -> 5,9".parse().unwrap(),
            "8,0 -> 0,8".parse().unwrap(),
            "9,4 -> 3,4".parse().unwrap(),
            "2,2 -> 2,1".parse().unwrap(),
            "7,0 -> 7,4".parse().unwrap(),
            "6,4 -> 2,0".parse().unwrap(),
            "0,9 -> 2,9".parse().unwrap(),
            "3,4 -> 1,4".parse().unwrap(),
            "0,0 -> 8,8".parse().unwrap(),
            "5,5 -> 8,2".parse().unwrap(),
        ];

        let expected = 12;

        assert_eq!(expected, part2(&input))
    }
}
