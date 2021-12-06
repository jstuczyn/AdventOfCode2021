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
use utils::execute;
use utils::input_read::read_parsed_comma_separated_values;

fn naive_simulation(cycle_timers: &[usize], days: usize) -> usize {
    let mut timers: [usize; 9] = Default::default();
    for timer in cycle_timers {
        timers[*timer] += 1;
    }

    for _ in 0..days {
        let mut new_timers: [usize; 9] = Default::default();
        new_timers[6] = timers[0];
        new_timers[8] = timers[0];

        for (timer_value, fish_count) in timers.iter().enumerate().skip(1) {
            new_timers[timer_value - 1] += fish_count
        }
        timers = new_timers;
    }

    timers.iter().sum()
}

fn part1(input: &[usize]) -> usize {
    naive_simulation(input, 80)
}

fn part2(input: &[usize]) -> usize {
    naive_simulation(input, 256)
}

#[cfg(not(tarpaulin))]
fn main() {
    execute("input", read_parsed_comma_separated_values, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![3, 4, 3, 1, 2];

        let expected = 5934;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![3, 4, 3, 1, 2];

        let expected = 26984457539;

        assert_eq!(expected, part2(&input))
    }
}
