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

use utils::execute;
use utils::input_read::read_parsed_comma_separated_values;

fn part1(input: &[usize]) -> usize {
    let mut owned_input = input.to_vec();
    let idx = input.len() / 2;
    let (_, median, _) = owned_input.select_nth_unstable(idx);

    input
        .iter()
        .map(|&x| (x as isize - *median as isize).abs() as usize)
        .sum()
}

fn part2(input: &[usize]) -> usize {
    0
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
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 37;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {}
}
