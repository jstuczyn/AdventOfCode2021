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

use std::cmp::Reverse;
use std::collections::HashSet;
use utils::execute;
use utils::input_read::read_input_lines;

#[derive(Debug)]
struct Basin {
    points: HashSet<Point>,
}

impl Basin {
    fn size(&self) -> usize {
        self.points.len()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn new(x: usize, y: usize, height: usize) -> Self {
        Point { x, y, height }
    }

    fn risk_level(&self) -> usize {
        self.height + 1
    }
}

#[derive(Debug)]
struct HeightMap {
    rows: Vec<Vec<usize>>,
}

impl HeightMap {
    fn from_raw_rows(raw: &[String]) -> Self {
        let rows = raw
            .iter()
            .map(|raw_row| {
                raw_row
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        HeightMap { rows }
    }

    fn check_low_point(&self, x: usize, y: usize, value: usize) -> bool {
        // left
        if x > 0 && self.rows[y][x - 1] <= value {
            return false;
        }

        // top
        if y > 0 && self.rows[y - 1][x] <= value {
            return false;
        }

        // right
        if let Some(&right) = self.rows[y].get(x + 1) {
            if right <= value {
                return false;
            }
        }

        // down
        if let Some(down_row) = self.rows.get(y + 1) {
            if down_row[x] <= value {
                return false;
            }
        }

        true
    }

    fn low_points(&self) -> Vec<Point> {
        let mut low_points = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if self.check_low_point(x, y, *value) {
                    low_points.push(Point::new(x, y, *value))
                }
            }
        }
        low_points
    }

    fn check_surrounding_points_for_common_basin(&self, point: Point) -> Vec<Point> {
        let mut new_basin_members = Vec::with_capacity(4);

        // left
        if point.x > 0 {
            let left_value = self.rows[point.y][point.x - 1];
            if left_value != 9 {
                new_basin_members.push(Point::new(point.x - 1, point.y, left_value))
            }
        }

        // top
        if point.y > 0 {
            let top_value = self.rows[point.y - 1][point.x];
            if top_value != 9 {
                new_basin_members.push(Point::new(point.x, point.y - 1, top_value))
            }
        }

        // right
        if let Some(&right_value) = self.rows[point.y].get(point.x + 1) {
            if right_value != 9 {
                new_basin_members.push(Point::new(point.x + 1, point.y, right_value))
            }
        }

        // down
        if let Some(down_row) = self.rows.get(point.y + 1) {
            let down_value = down_row[point.x];
            if down_value != 9 {
                new_basin_members.push(Point::new(point.x, point.y + 1, down_value))
            }
        }

        new_basin_members
    }

    fn basin_around(&self, point: Point) -> Basin {
        let mut basin_points = HashSet::new();
        basin_points.insert(point);
        let mut unchecked_points = vec![point];

        loop {
            let mut new_unchecked = Vec::new();
            for unchecked in &unchecked_points {
                for new_point in self.check_surrounding_points_for_common_basin(*unchecked) {
                    if !basin_points.contains(&new_point) {
                        basin_points.insert(new_point);
                        new_unchecked.push(new_point);
                    }
                }
            }

            unchecked_points = new_unchecked;
            if unchecked_points.is_empty() {
                break;
            }
        }

        Basin {
            points: basin_points,
        }
    }
}

fn part1(input: &[String]) -> usize {
    HeightMap::from_raw_rows(input)
        .low_points()
        .into_iter()
        .map(|point| point.risk_level())
        .sum()
}

fn part2(input: &[String]) -> usize {
    let height_map = HeightMap::from_raw_rows(input);
    let low_points = height_map.low_points();

    let mut basins = low_points
        .into_iter()
        .map(|point| height_map.basin_around(point))
        .collect::<Vec<_>>();
    basins.sort_by_key(|b| Reverse(b.size()));

    basins.iter().take(3).map(|basin| basin.size()).product()
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
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        let expected = 15;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        let expected = 1134;

        assert_eq!(expected, part2(&input))
    }
}
