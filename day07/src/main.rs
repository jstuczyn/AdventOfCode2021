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

use std::cmp::min;
use utils::execute_slice;
use utils::input_read::read_parsed_comma_separated_values;

fn abs_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).unsigned_abs()
}

fn part1(input: &[usize]) -> usize {
    let mut owned_input = input.to_vec();
    let idx = input.len() / 2;
    let (_, median, _) = owned_input.select_nth_unstable(idx);

    input.iter().map(|&x| abs_diff(x, *median)).sum()
}

fn part2(input: &[usize]) -> usize {
    fn fuel_cost(from: usize, to: usize) -> usize {
        (1..=abs_diff(from, to)).sum()
    }

    // so apparently we can't use just mean since its minimises distance^2
    // and we need to minimise (distance * (distance + 1)) / 2.
    // so rather than just doing a big binary search, just try 2 values closest
    // to minimised d^2 and choose the smaller one
    let sum: usize = input.iter().sum();
    let mean_f = (sum as f32 / input.len() as f32).floor() as usize;
    let mean_c = (sum as f32 / input.len() as f32).ceil() as usize;

    let min_f = input.iter().map(|&x| fuel_cost(x, mean_f)).sum();
    let min_c = input.iter().map(|&x| fuel_cost(x, mean_c)).sum();

    min(min_f, min_c)
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_comma_separated_values, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 37;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 168;

        assert_eq!(expected, part2(&input))
    }
}
