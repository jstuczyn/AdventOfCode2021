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

use anyhow::Error;
use itertools::iproduct;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;
use utils::parsing::parse_raw_range;

#[derive(Debug, Clone)]
struct Cuboid {
    on: bool,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
}

impl FromStr for Cuboid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let on = s.starts_with("on");
        let mut ranges = if on {
            s.strip_prefix("on ")
                .ok_or_else(|| Error::msg("incomplete input"))?
                .split(',')
        } else {
            s.strip_prefix("off ")
                .ok_or_else(|| Error::msg("incomplete input"))?
                .split(',')
        };

        let x_range = parse_raw_range(
            ranges
                .next()
                .ok_or_else(|| Error::msg("incomplete input"))?,
        )?;
        let y_range = parse_raw_range(
            ranges
                .next()
                .ok_or_else(|| Error::msg("incomplete input"))?,
        )?;
        let z_range = parse_raw_range(
            ranges
                .next()
                .ok_or_else(|| Error::msg("incomplete input"))?,
        )?;

        Ok(Cuboid {
            on,
            x_range,
            y_range,
            z_range,
        })
    }
}

// impl Cuboid {
//     fn has_cube(&self, cube: Cube) -> bool {
//         self.x_range.contains(&cube.0)
//             && self.y_range.contains(&cube.1)
//             && self.z_range.contains(&cube.2)
//     }
// }

type Cube = (isize, isize, isize);

struct ReactorCore {
    active_cubes: HashSet<Cube>,
    initialization_area: Cuboid,
}

impl ReactorCore {
    fn new() -> Self {
        ReactorCore {
            active_cubes: Default::default(),
            initialization_area: Cuboid {
                on: false,
                x_range: RangeInclusive::new(-50, 50),
                y_range: RangeInclusive::new(-50, 50),
                z_range: RangeInclusive::new(-50, 50),
            },
        }
    }

    fn check_range(
        base: &RangeInclusive<isize>,
        limit: &RangeInclusive<isize>,
    ) -> Option<RangeInclusive<isize>> {
        // easy range checks:
        if base.start() > limit.end() {
            return None;
        }
        if base.end() < limit.start() {
            return None;
        }

        let start = max(base.start(), limit.start());
        let end = min(base.end(), limit.end());

        Some(RangeInclusive::new(*start, *end))
    }

    fn run_initialization_step(&mut self, step: &Cuboid) -> Option<()> {
        // filter out cuboids completely outside the area
        let limited_x = Self::check_range(&step.x_range, &self.initialization_area.x_range)?;
        let limited_y = Self::check_range(&step.y_range, &self.initialization_area.y_range)?;
        let limited_z = Self::check_range(&step.z_range, &self.initialization_area.z_range)?;

        for cube in iproduct!(limited_x, limited_y, limited_z) {
            if step.on {
                self.active_cubes.insert(cube);
            } else {
                self.active_cubes.remove(&cube);
            }
        }
        Some(())
    }
}

fn part1(input: &[Cuboid]) -> usize {
    let mut reactor_core = ReactorCore::new();
    for cube in input {
        reactor_core.run_initialization_step(cube);
    }
    reactor_core.active_cubes.len()
}

fn part2(input: &[Cuboid]) -> usize {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small_example() {
        let input = vec![
            "on x=10..12,y=10..12,z=10..12".parse().unwrap(),
            "on x=11..13,y=11..13,z=11..13".parse().unwrap(),
            "off x=9..11,y=9..11,z=9..11".parse().unwrap(),
            "on x=10..10,y=10..10,z=10..10".parse().unwrap(),
        ];

        let expected = 39;
        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "on x=-20..26,y=-36..17,z=-47..7".parse().unwrap(),
            "on x=-20..33,y=-21..23,z=-26..28".parse().unwrap(),
            "on x=-22..28,y=-29..23,z=-38..16".parse().unwrap(),
            "on x=-46..7,y=-6..46,z=-50..-1".parse().unwrap(),
            "on x=-49..1,y=-3..46,z=-24..28".parse().unwrap(),
            "on x=2..47,y=-22..22,z=-23..27".parse().unwrap(),
            "on x=-27..23,y=-28..26,z=-21..29".parse().unwrap(),
            "on x=-39..5,y=-6..47,z=-3..44".parse().unwrap(),
            "on x=-30..21,y=-8..43,z=-13..34".parse().unwrap(),
            "on x=-22..26,y=-27..20,z=-29..19".parse().unwrap(),
            "off x=-48..-32,y=26..41,z=-47..-37".parse().unwrap(),
            "on x=-12..35,y=6..50,z=-50..-2".parse().unwrap(),
            "off x=-48..-32,y=-32..-16,z=-15..-5".parse().unwrap(),
            "on x=-18..26,y=-33..15,z=-7..46".parse().unwrap(),
            "off x=-40..-22,y=-38..-28,z=23..41".parse().unwrap(),
            "on x=-16..35,y=-41..10,z=-47..6".parse().unwrap(),
            "off x=-32..-23,y=11..30,z=-14..3".parse().unwrap(),
            "on x=-49..-5,y=-3..45,z=-29..18".parse().unwrap(),
            "off x=18..30,y=-20..-8,z=-3..13".parse().unwrap(),
            "on x=-41..9,y=-7..43,z=-33..15".parse().unwrap(),
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877"
                .parse()
                .unwrap(),
            "on x=967..23432,y=45373..81175,z=27513..53682"
                .parse()
                .unwrap(),
        ];

        let expected = 590784;
        assert_eq!(expected, part1(&input))
    }
}
