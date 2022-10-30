// Copyright 2022 Jedrzej Stuczynski
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

use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Add, Sub};
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_groups;

const OVERLAP_THRESHOLD: usize = 12;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl From<(isize, isize, isize)> for Position {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Position { x, y, z }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split
            .next()
            .ok_or_else(|| anyhow!("no x value present"))?
            .parse()?;
        let y = split
            .next()
            .ok_or_else(|| anyhow!("no y value present"))?
            .parse()?;
        let z = split
            .next()
            .ok_or_else(|| anyhow!("no z value present"))?
            .parse()?;
        Ok(Position { x, y, z })
    }
}

impl Position {
    #[inline]
    const fn origin() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }

    #[inline]
    const fn rot_90x(&self) -> Self {
        Position {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    #[inline]
    const fn rot_180x(&self) -> Self {
        Position {
            x: self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    #[inline]
    const fn rot_270x(&self) -> Self {
        Position {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    #[inline]
    const fn rot_90y(&self) -> Self {
        Position {
            x: self.z,
            y: self.y,
            z: -self.x,
        }
    }

    #[inline]
    const fn rot_180y(&self) -> Self {
        Position {
            x: -self.x,
            y: self.y,
            z: -self.z,
        }
    }

    #[inline]
    const fn rot_270y(&self) -> Self {
        Position {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    const fn rot_90z(&self) -> Self {
        Position {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    #[allow(unused)]
    const fn rot_180z(&self) -> Self {
        Position {
            x: -self.x,
            y: -self.y,
            z: self.z,
        }
    }

    #[inline]
    const fn rot_270z(&self) -> Self {
        Position {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    #[inline]
    const fn all_rotations(&self) -> [Self; 24] {
        [
            // x0:
            *self,
            self.rot_90y(),
            self.rot_180y(),
            self.rot_270y(),
            self.rot_90z(),
            self.rot_270z(),
            // x90:
            self.rot_90x(),
            self.rot_90x().rot_90y(),
            self.rot_90x().rot_180y(),
            self.rot_90x().rot_270y(),
            self.rot_90x().rot_90z(),
            self.rot_90x().rot_270z(),
            // x180:
            self.rot_180x(),
            self.rot_180x().rot_90y(),
            self.rot_180x().rot_180y(),
            self.rot_180x().rot_270y(),
            self.rot_180x().rot_90z(),
            self.rot_180x().rot_270z(),
            // x270:
            self.rot_270x(),
            self.rot_270x().rot_90y(),
            self.rot_270x().rot_180y(),
            self.rot_270x().rot_270y(),
            self.rot_270x().rot_90z(),
            self.rot_270x().rot_270z(),
        ]
    }

    #[inline]
    const fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    relative_position: Position,
    beacons: BTreeSet<Position>,
}

impl FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("empty input")
        }
        let mut lines = s.lines();
        let id_line = lines.next().ok_or_else(|| anyhow!("no id value present"))?;
        let prefix_stripped = id_line
            .strip_prefix("--- scanner ")
            .ok_or_else(|| anyhow!("invalid scanner id"))?;
        let id = prefix_stripped
            .strip_suffix(" ---")
            .ok_or_else(|| anyhow!("invalid scanner id"))?
            .parse()?;

        let beacons = lines
            .into_iter()
            .map(FromStr::from_str)
            .collect::<Result<BTreeSet<_>, _>>()?;

        Ok(Scanner {
            id,
            relative_position: Position::origin(),
            beacons,
        })
    }
}

impl Scanner {
    fn all_rotations(&self) -> [Scanner; 24] {
        let beacon_rotations = self
            .beacons
            .iter()
            .map(|b| b.all_rotations())
            .collect::<Vec<_>>();

        (0..24)
            .map(|i| Scanner {
                id: self.id,
                relative_position: self.relative_position,
                beacons: beacon_rotations.iter().map(|b| b[i]).collect(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn translate(&self, change: Position) -> Self {
        Scanner {
            id: self.id,
            relative_position: self.relative_position + change,
            beacons: self.beacons.iter().map(|&b| b + change).collect(),
        }
    }

    fn overlap_count(&self, other: &Self) -> usize {
        let mut count = 0;
        for other_beacon in &other.beacons {
            if self.beacons.contains(other_beacon) {
                count += 1;
            }
        }

        count
    }

    #[cfg(test)]
    fn overlapping_beacons(&self, other: &Self) -> Vec<Position> {
        let mut overlap = Vec::new();
        for other_beacon in &other.beacons {
            if self.beacons.contains(other_beacon) {
                overlap.push(*other_beacon)
            }
        }

        overlap
    }

    // we treat 'self' as the source of truth
    fn try_align_scanner(&self, other: &Self) -> Option<Scanner> {
        for &base in &self.beacons {
            for rotation in other.all_rotations() {
                for &beacon in &rotation.beacons {
                    let translation_candidate = base - beacon;

                    let translated_scanner = rotation.translate(translation_candidate);
                    if self.overlap_count(&translated_scanner) >= OVERLAP_THRESHOLD {
                        // we found it!
                        return Some(translated_scanner);
                    }
                }
            }
        }

        None
    }
}

fn try_align_relative_to<'a, I: Iterator<Item = &'a Scanner>>(
    base: &Scanner,
    unaligned: I,
) -> Vec<Scanner> {
    let mut aligned_scanners = Vec::new();
    for scanner in unaligned {
        if let Some(aligned) = base.try_align_scanner(scanner) {
            aligned_scanners.push(aligned)
        }
    }

    aligned_scanners
}

fn reconstruct_absolute_positions(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut unaligned = scanners
        .iter()
        .skip(1)
        .map(|s| (s.id, s.clone()))
        .collect::<HashMap<_, _>>();

    // we treat scanner 0 as the origin and attempt to align everything relative to it
    let mut aligned = vec![];

    // check leftover scanners only against any newly aligned entries
    let mut aligned_last_iter = vec![scanners[0].clone()];

    while !unaligned.is_empty() {
        let mut aligned_this_iter = Vec::new();

        for known in &aligned_last_iter {
            let new_aligned = try_align_relative_to(known, unaligned.values());
            for new_known in new_aligned {
                unaligned.remove(&new_known.id);
                aligned_this_iter.push(new_known);
            }
        }

        aligned.append(&mut aligned_last_iter);
        aligned_last_iter = aligned_this_iter;
    }
    aligned.append(&mut aligned_last_iter);

    aligned
}

fn part1(input: &[Scanner]) -> usize {
    let mut unique_beacons = HashSet::new();
    let aligned_scanners = reconstruct_absolute_positions(input);
    for scanner in aligned_scanners {
        for beacon in scanner.beacons {
            unique_beacons.insert(beacon);
        }
    }

    unique_beacons.len()
}

fn part2(input: &[Scanner]) -> usize {
    reconstruct_absolute_positions(input)
        .into_iter()
        .map(|s| s.relative_position)
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.manhattan_distance(&b))
        .max()
        .expect("failed to align the scanners!")
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_groups, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_positions() -> Vec<Position> {
        vec![
            Position {
                x: 230,
                y: 43,
                z: 780,
            },
            Position {
                x: -230,
                y: 43,
                z: 780,
            },
            Position {
                x: 230,
                y: -43,
                z: 780,
            },
            Position {
                x: 230,
                y: 43,
                z: -780,
            },
            Position {
                x: -230,
                y: -43,
                z: -780,
            },
            Position {
                x: 0,
                y: -43,
                z: 780,
            },
            Position {
                x: -230,
                y: 0,
                z: -780,
            },
            Position {
                x: -230,
                y: 43,
                z: 0,
            },
        ]
    }

    #[test]
    fn x_rotations() {
        for pos in fake_positions() {
            assert_eq!(pos.rot_90x().rot_90x(), pos.rot_180x());
            assert_eq!(pos.rot_90x().rot_90x().rot_90x(), pos.rot_270x());
            assert_eq!(pos.rot_180x().rot_90x(), pos.rot_270x());
        }
    }

    #[test]
    fn y_rotations() {
        for pos in fake_positions() {
            assert_eq!(pos.rot_90y().rot_90y(), pos.rot_180y());
            assert_eq!(pos.rot_90y().rot_90y().rot_90y(), pos.rot_270y());
            assert_eq!(pos.rot_180y().rot_90y(), pos.rot_270y());
        }
    }

    #[test]
    fn z_rotations() {
        for pos in fake_positions() {
            assert_eq!(pos.rot_90z().rot_90z(), pos.rot_180z());
            assert_eq!(pos.rot_90z().rot_90z().rot_90z(), pos.rot_270z());
            assert_eq!(pos.rot_180z().rot_90z(), pos.rot_270z());
        }
    }

    fn example_scanners() -> Vec<Scanner> {
        let scanner0 = Scanner {
            id: 0,
            relative_position: Position::origin(),
            beacons: vec![
                (404, -588, -901).into(),
                (528, -643, 409).into(),
                (-838, 591, 734).into(),
                (390, -675, -793).into(),
                (-537, -823, -458).into(),
                (-485, -357, 347).into(),
                (-345, -311, 381).into(),
                (-661, -816, -575).into(),
                (-876, 649, 763).into(),
                (-618, -824, -621).into(),
                (553, 345, -567).into(),
                (474, 580, 667).into(),
                (-447, -329, 318).into(),
                (-584, 868, -557).into(),
                (544, -627, -890).into(),
                (564, 392, -477).into(),
                (455, 729, 728).into(),
                (-892, 524, 684).into(),
                (-689, 845, -530).into(),
                (423, -701, 434).into(),
                (7, -33, -71).into(),
                (630, 319, -379).into(),
                (443, 580, 662).into(),
                (-789, 900, -551).into(),
                (459, -707, 401).into(),
            ]
            .into_iter()
            .collect(),
        };

        let scanner1 = Scanner {
            id: 1,
            relative_position: Position::origin(),
            beacons: vec![
                (686, 422, 578).into(),
                (605, 423, 415).into(),
                (515, 917, -361).into(),
                (-336, 658, 858).into(),
                (95, 138, 22).into(),
                (-476, 619, 847).into(),
                (-340, -569, -846).into(),
                (567, -361, 727).into(),
                (-460, 603, -452).into(),
                (669, -402, 600).into(),
                (729, 430, 532).into(),
                (-500, -761, 534).into(),
                (-322, 571, 750).into(),
                (-466, -666, -811).into(),
                (-429, -592, 574).into(),
                (-355, 545, -477).into(),
                (703, -491, -529).into(),
                (-328, -685, 520).into(),
                (413, 935, -424).into(),
                (-391, 539, -444).into(),
                (586, -435, 557).into(),
                (-364, -763, -893).into(),
                (807, -499, -711).into(),
                (755, -354, -619).into(),
                (553, 889, -390).into(),
            ]
            .into_iter()
            .collect(),
        };

        let scanner2 = Scanner {
            id: 2,
            relative_position: Position::origin(),
            beacons: vec![
                (649, 640, 665).into(),
                (682, -795, 504).into(),
                (-784, 533, -524).into(),
                (-644, 584, -595).into(),
                (-588, -843, 648).into(),
                (-30, 6, 44).into(),
                (-674, 560, 763).into(),
                (500, 723, -460).into(),
                (609, 671, -379).into(),
                (-555, -800, 653).into(),
                (-675, -892, -343).into(),
                (697, -426, -610).into(),
                (578, 704, 681).into(),
                (493, 664, -388).into(),
                (-671, -858, 530).into(),
                (-667, 343, 800).into(),
                (571, -461, -707).into(),
                (-138, -166, 112).into(),
                (-889, 563, -600).into(),
                (646, -828, 498).into(),
                (640, 759, 510).into(),
                (-630, 509, 768).into(),
                (-681, -892, -333).into(),
                (673, -379, -804).into(),
                (-742, -814, -386).into(),
                (577, -820, 562).into(),
            ]
            .into_iter()
            .collect(),
        };

        let scanner3 = Scanner {
            id: 3,
            relative_position: Position::origin(),
            beacons: vec![
                (-589, 542, 597).into(),
                (605, -692, 669).into(),
                (-500, 565, -823).into(),
                (-660, 373, 557).into(),
                (-458, -679, -417).into(),
                (-488, 449, 543).into(),
                (-626, 468, -788).into(),
                (338, -750, -386).into(),
                (528, -832, -391).into(),
                (562, -778, 733).into(),
                (-938, -730, 414).into(),
                (543, 643, -506).into(),
                (-524, 371, -870).into(),
                (407, 773, 750).into(),
                (-104, 29, 83).into(),
                (378, -903, -323).into(),
                (-778, -728, 485).into(),
                (426, 699, 580).into(),
                (-438, -605, -362).into(),
                (-469, -447, -387).into(),
                (509, 732, 623).into(),
                (647, 635, -688).into(),
                (-868, -804, 481).into(),
                (614, -800, 639).into(),
                (595, 780, -596).into(),
            ]
            .into_iter()
            .collect(),
        };

        let scanner4 = Scanner {
            id: 4,
            relative_position: Position::origin(),
            beacons: vec![
                (727, 592, 562).into(),
                (-293, -554, 779).into(),
                (441, 611, -461).into(),
                (-714, 465, -776).into(),
                (-743, 427, -804).into(),
                (-660, -479, -426).into(),
                (832, -632, 460).into(),
                (927, -485, -438).into(),
                (408, 393, -506).into(),
                (466, 436, -512).into(),
                (110, 16, 151).into(),
                (-258, -428, 682).into(),
                (-393, 719, 612).into(),
                (-211, -452, 876).into(),
                (808, -476, -593).into(),
                (-575, 615, 604).into(),
                (-485, 667, 467).into(),
                (-680, 325, -822).into(),
                (-627, -443, -432).into(),
                (872, -547, -609).into(),
                (833, 512, 582).into(),
                (807, 604, 487).into(),
                (839, -516, 451).into(),
                (891, -625, 532).into(),
                (-652, -548, -490).into(),
                (30, -46, -14).into(),
            ]
            .into_iter()
            .collect(),
        };

        vec![scanner0, scanner1, scanner2, scanner3, scanner4]
    }

    #[test]
    fn overlap_example() {
        let example_scanners = example_scanners();
        let scanner0 = &example_scanners[0];
        let scanner1 = &example_scanners[1];

        let expected: Vec<Position> = vec![
            (-618, -824, -621).into(),
            (-537, -823, -458).into(),
            (-447, -329, 318).into(),
            (404, -588, -901).into(),
            (544, -627, -890).into(),
            (528, -643, 409).into(),
            (-661, -816, -575).into(),
            (390, -675, -793).into(),
            (423, -701, 434).into(),
            (-345, -311, 381).into(),
            (459, -707, 401).into(),
            (-485, -357, 347).into(),
        ];

        let aligned = scanner0.try_align_scanner(scanner1).unwrap();
        assert_eq!(expected, scanner0.overlapping_beacons(&aligned))
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(79, part1(&example_scanners()))
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(3621, part2(&example_scanners()))
    }
}
