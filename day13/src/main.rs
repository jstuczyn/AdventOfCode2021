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

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use utils::execute;
use utils::input_read::read_into_string_groups;

#[derive(Debug)]
struct MalformedFold;

#[derive(Debug)]
struct MalformedPoint;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = MalformedPoint;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split
            .next()
            .ok_or(MalformedPoint)?
            .parse()
            .map_err(|_| MalformedPoint)?;
        let y = split
            .next()
            .ok_or(MalformedPoint)?
            .parse()
            .map_err(|_| MalformedPoint)?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    at: usize,
}

impl FromStr for Fold {
    type Err = MalformedFold;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("fold along ").ok_or(MalformedFold)?;
        let mut split = stripped.split('=');
        let axis = match split.next().ok_or(MalformedFold)? {
            c if c == "x" => Axis::X,
            c if c == "y" => Axis::Y,
            _ => return Err(MalformedFold),
        };
        let at = split
            .next()
            .ok_or(MalformedFold)?
            .parse()
            .map_err(|_| MalformedFold)?;

        Ok(Fold { axis, at })
    }
}

#[derive(Debug)]
struct Manual {
    points: HashSet<Point>,
    folds: VecDeque<Fold>,
}

impl Manual {
    fn from_raw(raw: &[String]) -> Manual {
        let points = raw[0].lines().map(|s| s.parse().unwrap()).collect();
        let folds = raw[1].lines().map(|s| s.parse().unwrap()).collect();

        Manual { points, folds }
    }

    fn fold_at_y_axis(&mut self, at: usize) {
        let mut new_points: HashSet<Point> = self
            .points
            .iter()
            .filter(|point| point.y < at)
            .copied()
            .collect();
        for point in &self.points {
            if point.y > at {
                new_points.insert(Point {
                    x: point.x,
                    y: 2 * at - point.y,
                });
            }
        }

        self.points = new_points
    }

    fn fold_at_x_axis(&mut self, at: usize) {
        let mut new_points: HashSet<Point> = self
            .points
            .iter()
            .filter(|point| point.x < at)
            .copied()
            .collect();
        for point in &self.points {
            if point.x > at {
                new_points.insert(Point {
                    x: 2 * at - point.x,
                    y: point.y,
                });
            }
        }

        self.points = new_points
    }

    fn fold(&mut self) -> bool {
        if let Some(fold) = self.folds.pop_front() {
            if fold.axis == Axis::Y {
                self.fold_at_y_axis(fold.at)
            } else {
                self.fold_at_x_axis(fold.at)
            }
            true
        } else {
            false
        }
    }

    fn final_manual(&self) -> String {
        let max_x = self.points.iter().max_by_key(|point| point.x).unwrap().x;
        let max_y = self.points.iter().max_by_key(|point| point.y).unwrap().y;
        let mut out = vec![String::new()];
        for y in 0..=max_y {
            let mut row = Vec::with_capacity(max_x);
            for x in 0..=max_x {
                if self.points.contains(&Point { x, y }) {
                    row.push('■');
                } else {
                    row.push('□')
                }
            }
            out.push(row.into_iter().collect::<String>())
        }
        out.join("\n")
    }
}

fn part1(input: &[String]) -> String {
    let mut manual = Manual::from_raw(input);
    manual.fold();
    manual.points.len().to_string()
}

fn part2(input: &[String]) -> String {
    let mut manual = Manual::from_raw(input);
    while manual.fold() {}
    manual.final_manual()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute("input", read_into_string_groups, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"
            .to_string(),
            "fold along y=7
fold along x=5"
                .to_string(),
        ];

        let expected = "17";

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"
            .to_string(),
            "fold along y=7
fold along x=5"
                .to_string(),
        ];

        let expected = r#"
■■■■■
■□□□■
■□□□■
■□□□■
■■■■■"#;

        assert_eq!(expected, part2(&input))
    }
}
