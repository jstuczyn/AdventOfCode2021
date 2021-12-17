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
use std::ops::RangeInclusive;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug)]
struct MalformedTarget;

#[derive(Debug, Clone)]
struct Target {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

fn parse_raw_range(raw: &str) -> Result<RangeInclusive<isize>, MalformedTarget> {
    let mut bounds = raw.split("=");
    let _axis = bounds.next().ok_or(MalformedTarget)?;
    let mut values = bounds.next().ok_or(MalformedTarget)?.split("..");

    let lower_bound = values
        .next()
        .ok_or(MalformedTarget)?
        .parse()
        .map_err(|_| MalformedTarget)?;
    let upper_bound = values
        .next()
        .ok_or(MalformedTarget)?
        .parse()
        .map_err(|_| MalformedTarget)?;

    Ok(RangeInclusive::new(lower_bound, upper_bound))
}

impl FromStr for Target {
    type Err = MalformedTarget;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("target area: ").ok_or(MalformedTarget)?;
        let mut ranges = stripped.split(", ");

        let x_range = parse_raw_range(ranges.next().ok_or(MalformedTarget)?)?;
        let y_range = parse_raw_range(ranges.next().ok_or(MalformedTarget)?)?;

        Ok(Target { x_range, y_range })
    }
}

impl Target {
    fn maximise_altitude(&self) -> usize {
        // only consider y acceleration, since probe's y position is independent of the x position
        // and we know there must exist *some* x acceleration for which this will work, otherwise
        // this task would have no solution

        // also note that since we're launching upwards, we will have to reach y = 0 again
        // and we're going to have Vy = -Vy_0 at that point
        // now, to maximise the altitude, we must maximise our launch velocity and therefore
        // also speed at which we cross y = 0
        // So to maintain the highest possible speed, we must therefore reach the bottom of the target
        // in a single step after reaching y = 0
        // so we must cross y = 0 at min y_pos of target + 1 (so that we would not miss it)

        // also:
        // y = Vy_0 * t - 1/2 t^2 + 1/2 t
        // y' = Vy_0 + 1/2 - t; y' = 0 <=> t = Vy0 + 1/2, so probe will reach its max attitude at t = Vy0 + 1/2
        // therefore we have to consider t = Vy0 and t = Vy0 + 1

        let vy_0 = (*self.y_range.start() + 1).abs() as usize;
        let y = |t: usize| vy_0 * t - t * t / 2 + t / 2;

        let t1 = vy_0;
        let t2 = vy_0 + 1;

        let y1 = y(t1);
        let y2 = y(t2);

        max(y1, y2)
    }
}

struct Velocity {
    dx: isize,
    dy: isize,
}

impl Velocity {
    #[allow(clippy::comparison_chain)]
    fn step(&mut self) {
        self.dy -= 1;

        if self.dx > 0 {
            self.dx -= 1
        } else if self.dx < 0 {
            self.dx += 1
        }
    }

    fn move_probe(&self, probe: &mut (isize, isize)) {
        probe.0 += self.dx;
        probe.1 += self.dy;
    }
}

fn part1(target: Target) -> usize {
    target.maximise_altitude()
}

fn part2(target: Target) -> usize {
    // unfortunately I'm running out of time now, so we're left to bruteforcing here : (
    let mut valid_velocities = 0;
    for dx in 0..*target.x_range.end() * 2 {
        for dy in *target.y_range.start()..target.y_range.start().abs() {
            let mut v = Velocity { dx, dy };
            let mut probe = (0, 0);
            loop {
                if target.x_range.contains(&probe.0) && target.y_range.contains(&probe.1) {
                    valid_velocities += 1;
                    break;
                }
                if probe.0 > *target.x_range.end() {
                    break;
                }
                if probe.1 < *target.y_range.start() {
                    break;
                }

                v.move_probe(&mut probe);
                v.step();
            }
        }
    }

    valid_velocities
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
        let target = "target area: x=20..30, y=-10..-5".parse().unwrap();

        let expected = 45;
        assert_eq!(expected, part1(target))
    }

    #[test]
    fn part2_sample_input() {
        let target = "target area: x=20..30, y=-10..-5".parse().unwrap();

        let expected = 112;
        assert_eq!(expected, part2(target))
    }
}
