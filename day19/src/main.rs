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
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_groups;

#[derive(Debug)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
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

#[derive(Debug)]
pub struct Scanner {
    id: usize,
    beacons: Vec<Position>,
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
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Scanner { id, beacons })
    }
}

fn part1(_input: &[Scanner]) -> usize {
    todo!()
}

fn part2(_input: &[Scanner]) -> usize {
    todo!()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_groups, part1, part2)
}
