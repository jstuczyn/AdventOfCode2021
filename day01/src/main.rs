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

use itertools::Itertools;
use utils::{execute_slice, input_read};

fn part1(input: &[usize]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", input_read::read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;

        assert_eq!(expected, part2(&input))
    }
}
