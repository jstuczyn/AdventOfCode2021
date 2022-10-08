// Copyright 2021-2022 Jedrzej Stuczynski
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

use crate::intersection::Intersection;
use anyhow::Error;
use itertools::iproduct;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;
use utils::parsing::parse_raw_range;

mod intersection;

#[derive(Debug, Clone)]
struct Step {
    on: bool,
    cuboid: Cuboid,
}

impl FromStr for Step {
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

        Ok(Step {
            on,
            cuboid: Cuboid {
                x_range,
                y_range,
                z_range,
            },
        })
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cubes = self.clone().into_cubes();
        for cube in cubes {
            writeln!(f, "{}", cube)?;
        }

        Ok(())
    }
}

impl From<Cuboid> for Vec<Cube> {
    fn from(cuboid: Cuboid) -> Self {
        iproduct!(cuboid.x_range, cuboid.y_range, cuboid.z_range)
            .map(Into::into)
            .collect()
    }
}

impl Cuboid {
    fn into_cubes(self) -> Vec<Cube> {
        self.into()
    }

    fn size(&self) -> usize {
        let x_size = (self.x_range.end() - self.x_range.start()).unsigned_abs() + 1;
        let y_size = (self.y_range.end() - self.y_range.start()).unsigned_abs() + 1;
        let z_size = (self.z_range.end() - self.z_range.start()).unsigned_abs() + 1;

        x_size * y_size * z_size
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl From<(isize, isize, isize)> for Cube {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Cube { x, y, z }
    }
}

struct ReactorCore {
    additive_cuboids: Vec<Cuboid>,
    subtractive_cuboids: Vec<Cuboid>,
    initialization_area: Cuboid,
}

impl ReactorCore {
    fn new() -> Self {
        ReactorCore {
            additive_cuboids: vec![],
            subtractive_cuboids: vec![],
            initialization_area: Cuboid {
                x_range: RangeInclusive::new(-50, 50),
                y_range: RangeInclusive::new(-50, 50),
                z_range: RangeInclusive::new(-50, 50),
            },
        }
    }

    fn active_region_size(&self) -> usize {
        let positive_volume = self
            .additive_cuboids
            .iter()
            .map(|c| c.size())
            .sum::<usize>();

        let negative_volume = self
            .subtractive_cuboids
            .iter()
            .map(|c| c.size())
            .sum::<usize>();

        debug_assert!(positive_volume >= negative_volume);
        positive_volume - negative_volume
    }

    fn run_initialization_step(&mut self, cuboid: Cuboid, on: bool) {
        // since our input consists only of a double digit of cuboids, this naive approach is more than sufficient
        let mut new_subs = Vec::new();
        for add in &self.additive_cuboids {
            if let Some(intersection) = cuboid.intersection(add) {
                new_subs.push(intersection)
            }
        }

        for sub in &self.subtractive_cuboids {
            if let Some(intersection) = cuboid.intersection(sub) {
                self.additive_cuboids.push(intersection)
            }
        }

        self.subtractive_cuboids.append(&mut new_subs);

        if on {
            self.additive_cuboids.push(cuboid)
        }
    }

    fn run_part1_initialization_step(&mut self, step: &Step) {
        // filter out cuboids completely outside the area
        if let Some(restricted) = self.initialization_area.intersection(&step.cuboid) {
            self.run_initialization_step(restricted, step.on)
        }
    }

    // same as part 1 but without the area restriction
    fn run_part2_initialization_step(&mut self, step: &Step) {
        self.run_initialization_step(step.cuboid.clone(), step.on)
    }
}

fn part1(input: &[Step]) -> usize {
    let mut reactor_core = ReactorCore::new();
    for step in input {
        reactor_core.run_part1_initialization_step(step);
    }

    reactor_core.active_region_size()
}

fn part2(input: &[Step]) -> usize {
    let mut reactor_core = ReactorCore::new();
    for step in input {
        reactor_core.run_part2_initialization_step(step);
    }

    reactor_core.active_region_size()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuboid_size() {
        assert_eq!(
            Cuboid {
                x_range: 1..=1,
                y_range: 1..=1,
                z_range: 1..=1
            }
            .size(),
            1
        );

        assert_eq!(
            Cuboid {
                x_range: 1..=10,
                y_range: 1..=10,
                z_range: 1..=10
            }
            .size(),
            1000
        );

        assert_eq!(
            Cuboid {
                x_range: -10..=-1,
                y_range: -10..=-1,
                z_range: -10..=-1
            }
            .size(),
            1000
        );
    }

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

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "on x=-5..47,y=-31..22,z=-19..33".parse().unwrap(),
            "on x=-44..5,y=-27..21,z=-14..35".parse().unwrap(),
            "on x=-49..-1,y=-11..42,z=-10..38".parse().unwrap(),
            "on x=-20..34,y=-40..6,z=-44..1".parse().unwrap(),
            "off x=26..39,y=40..50,z=-2..11".parse().unwrap(),
            "on x=-41..5,y=-41..6,z=-36..8".parse().unwrap(),
            "off x=-43..-33,y=-45..-28,z=7..25".parse().unwrap(),
            "on x=-33..15,y=-32..19,z=-34..11".parse().unwrap(),
            "off x=35..47,y=-46..-34,z=-11..5".parse().unwrap(),
            "on x=-14..36,y=-6..44,z=-16..29".parse().unwrap(),
            "on x=-57795..-6158,y=29564..72030,z=20435..90618"
                .parse()
                .unwrap(),
            "on x=36731..105352,y=-21140..28532,z=16094..90401"
                .parse()
                .unwrap(),
            "on x=30999..107136,y=-53464..15513,z=8553..71215"
                .parse()
                .unwrap(),
            "on x=13528..83982,y=-99403..-27377,z=-24141..23996"
                .parse()
                .unwrap(),
            "on x=-72682..-12347,y=18159..111354,z=7391..80950"
                .parse()
                .unwrap(),
            "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709"
                .parse()
                .unwrap(),
            "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856"
                .parse()
                .unwrap(),
            "on x=-52752..22273,y=-49450..9096,z=54442..119054"
                .parse()
                .unwrap(),
            "on x=-29982..40483,y=-108474..-28371,z=-24328..38471"
                .parse()
                .unwrap(),
            "on x=-4958..62750,y=40422..118853,z=-7672..65583"
                .parse()
                .unwrap(),
            "on x=55694..108686,y=-43367..46958,z=-26781..48729"
                .parse()
                .unwrap(),
            "on x=-98497..-18186,y=-63569..3412,z=1232..88485"
                .parse()
                .unwrap(),
            "on x=-726..56291,y=-62629..13224,z=18033..85226"
                .parse()
                .unwrap(),
            "on x=-110886..-34664,y=-81338..-8658,z=8914..63723"
                .parse()
                .unwrap(),
            "on x=-55829..24974,y=-16897..54165,z=-121762..-28058"
                .parse()
                .unwrap(),
            "on x=-65152..-11147,y=22489..91432,z=-58782..1780"
                .parse()
                .unwrap(),
            "on x=-120100..-32970,y=-46592..27473,z=-11695..61039"
                .parse()
                .unwrap(),
            "on x=-18631..37533,y=-124565..-50804,z=-35667..28308"
                .parse()
                .unwrap(),
            "on x=-57817..18248,y=49321..117703,z=5745..55881"
                .parse()
                .unwrap(),
            "on x=14781..98692,y=-1341..70827,z=15753..70151"
                .parse()
                .unwrap(),
            "on x=-34419..55919,y=-19626..40991,z=39015..114138"
                .parse()
                .unwrap(),
            "on x=-60785..11593,y=-56135..2999,z=-95368..-26915"
                .parse()
                .unwrap(),
            "on x=-32178..58085,y=17647..101866,z=-91405..-8878"
                .parse()
                .unwrap(),
            "on x=-53655..12091,y=50097..105568,z=-75335..-4862"
                .parse()
                .unwrap(),
            "on x=-111166..-40997,y=-71714..2688,z=5609..50954"
                .parse()
                .unwrap(),
            "on x=-16602..70118,y=-98693..-44401,z=5197..76897"
                .parse()
                .unwrap(),
            "on x=16383..101554,y=4615..83635,z=-44907..18747"
                .parse()
                .unwrap(),
            "off x=-95822..-15171,y=-19987..48940,z=10804..104439"
                .parse()
                .unwrap(),
            "on x=-89813..-14614,y=16069..88491,z=-3297..45228"
                .parse()
                .unwrap(),
            "on x=41075..99376,y=-20427..49978,z=-52012..13762"
                .parse()
                .unwrap(),
            "on x=-21330..50085,y=-17944..62733,z=-112280..-30197"
                .parse()
                .unwrap(),
            "on x=-16478..35915,y=36008..118594,z=-7885..47086"
                .parse()
                .unwrap(),
            "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456"
                .parse()
                .unwrap(),
            "off x=2032..69770,y=-71013..4824,z=7471..94418"
                .parse()
                .unwrap(),
            "on x=43670..120875,y=-42068..12382,z=-24787..38892"
                .parse()
                .unwrap(),
            "off x=37514..111226,y=-45862..25743,z=-16714..54663"
                .parse()
                .unwrap(),
            "off x=25699..97951,y=-30668..59918,z=-15349..69697"
                .parse()
                .unwrap(),
            "off x=-44271..17935,y=-9516..60759,z=49131..112598"
                .parse()
                .unwrap(),
            "on x=-61695..-5813,y=40978..94975,z=8655..80240"
                .parse()
                .unwrap(),
            "off x=-101086..-9439,y=-7088..67543,z=33935..83858"
                .parse()
                .unwrap(),
            "off x=18020..114017,y=-48931..32606,z=21474..89843"
                .parse()
                .unwrap(),
            "off x=-77139..10506,y=-89994..-18797,z=-80..59318"
                .parse()
                .unwrap(),
            "off x=8476..79288,y=-75520..11602,z=-96624..-24783"
                .parse()
                .unwrap(),
            "on x=-47488..-1262,y=24338..100707,z=16292..72967"
                .parse()
                .unwrap(),
            "off x=-84341..13987,y=2429..92914,z=-90671..-1318"
                .parse()
                .unwrap(),
            "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188"
                .parse()
                .unwrap(),
            "off x=-27365..46395,y=31009..98017,z=15428..76570"
                .parse()
                .unwrap(),
            "off x=-70369..-16548,y=22648..78696,z=-1892..86821"
                .parse()
                .unwrap(),
            "on x=-53470..21291,y=-120233..-33476,z=-44150..38147"
                .parse()
                .unwrap(),
            "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"
                .parse()
                .unwrap(),
        ];

        let expected = 2758514936282235;
        assert_eq!(expected, part2(&input))
    }
}
