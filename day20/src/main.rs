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
use std::convert::TryInto;
use std::ops::RangeInclusive;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug, Clone)]
struct TrenchMap {
    enhancement_algorithm: [bool; 512],
    image: HashSet<(isize, isize)>,
    infinity: bool,
    image_boundary: (RangeInclusive<isize>, RangeInclusive<isize>),
}

impl FromStr for TrenchMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let algo = lines
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(|c| c == '#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        lines.next(); // empty line

        let mut image = HashSet::new();
        for (y, line) in lines.enumerate() {
            for (x, pixel) in line.chars().enumerate() {
                if pixel == '#' {
                    image.insert((x as isize, y as isize));
                }
            }
        }

        let mut map = TrenchMap {
            enhancement_algorithm: algo,
            image,
            infinity: false,
            image_boundary: (RangeInclusive::new(0, 0), RangeInclusive::new(0, 0)),
        };
        map.update_image_boundary();

        Ok(map)
    }
}

impl TrenchMap {
    fn update_image_boundary(&mut self) {
        let mut max_x = 0;
        let mut min_x = 0;
        let mut max_y = 0;
        let mut min_y = 0;
        for (x, y) in &self.image {
            if *x > max_x {
                max_x = *x;
            }
            if *x < min_x {
                min_x = *x
            }
            if *y > max_y {
                max_y = *y;
            }
            if *y < min_y {
                min_y = *y
            }
        }

        self.image_boundary = (
            RangeInclusive::new(min_x, max_x),
            RangeInclusive::new(min_y, max_y),
        );
    }

    fn lookup_pixel(&self, pos: (isize, isize)) -> bool {
        let (x, y) = pos;

        if !self.image_boundary.0.contains(&x) || !self.image_boundary.1.contains(&y) {
            self.infinity
        } else {
            self.image.contains(&pos)
        }
    }

    fn enhance_pixel(&self, pos: (isize, isize)) -> bool {
        let mut lookup = 0;

        // TL
        if self.lookup_pixel((pos.0 - 1, pos.1 - 1)) {
            lookup += 1 << 8;
        }

        // T
        if self.lookup_pixel((pos.0, pos.1 - 1)) {
            lookup += 1 << 7;
        }

        // TR
        if self.lookup_pixel((pos.0 + 1, pos.1 - 1)) {
            lookup += 1 << 6;
        }

        // L
        if self.lookup_pixel((pos.0 - 1, pos.1)) {
            lookup += 1 << 5;
        }

        // M
        if self.lookup_pixel((pos.0, pos.1)) {
            lookup += 1 << 4;
        }

        // MR
        if self.lookup_pixel((pos.0 + 1, pos.1)) {
            lookup += 1 << 3;
        }

        // BL
        if self.lookup_pixel((pos.0 - 1, pos.1 + 1)) {
            lookup += 1 << 2;
        }

        // B
        if self.lookup_pixel((pos.0, pos.1 + 1)) {
            lookup += 1 << 1;
        }

        // BR
        if self.lookup_pixel((pos.0 + 1, pos.1 + 1)) {
            lookup += 1 << 0;
        }

        self.enhancement_algorithm[lookup]
    }

    fn enhance(&mut self) {
        let mut new_image = HashSet::new();
        let (x_range, y_range) = &self.image_boundary;
        let min_x = x_range.start();
        let max_x = x_range.end();
        let min_y = y_range.start();
        let max_y = y_range.end();

        for x in min_x - 3..max_x + 3 {
            for y in min_y - 3..max_y + 3 {
                if self.enhance_pixel((x, y)) {
                    new_image.insert((x, y));
                }
            }
        }

        if self.infinity {
            self.infinity = self.enhancement_algorithm[511];
        } else {
            self.infinity = self.enhancement_algorithm[0]
        }

        self.image = new_image;
        self.update_image_boundary();
    }
}

fn part1(mut map: TrenchMap) -> usize {
    map.enhance();
    map.enhance();
    map.image.len()
}

fn part2(map: TrenchMap) -> usize {
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
    fn part1_sample_input() {
        let map = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            .parse()
            .unwrap();

        let expected = 35;
        assert_eq!(expected, part1(map));
    }
}
