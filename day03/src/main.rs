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
use utils::input_read::read_input_lines;

fn part1(input: &[String]) -> u32 {
    let num_bits = input[0].len() as u16;
    let input_size = input.len() as u16;

    let input: Vec<_> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    let mut gamma_rate = 0;

    for bit in 0..num_bits {
        let mut set_count = 0;
        for num in &input {
            set_count += num >> bit & 1;
        }

        if set_count > (input_size / 2) {
            gamma_rate |= 1 << bit;
        }
    }

    let mask = (1 << num_bits) - 1;
    let epsilon = !gamma_rate & mask;

    gamma_rate as u32 * epsilon as u32
}

fn part2(_input: &[String]) -> u32 {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    execute("input", read_input_lines, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let expected = 198;

        assert_eq!(expected, part1(&input))
    }
}
