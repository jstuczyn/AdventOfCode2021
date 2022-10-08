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

use utils::execute_slice;
use utils::input_read::read_input_lines;

fn most_common_bit(input: &[u16], position: u8) -> u8 {
    let mut set_count = 0;
    for num in input {
        set_count += num >> position & 1;
    }

    let unset = input.len() as u16 - set_count;
    match set_count {
        set if set >= unset => 1,
        _ => 0,
    }
}

fn part1(input: &[String]) -> u32 {
    let num_bits = input[0].len() as u8;

    let input: Vec<_> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    let mut gamma_rate = 0;

    for bit in 0..num_bits {
        gamma_rate |= (most_common_bit(&input, bit) as u16) << bit;
    }

    let mask = (1 << num_bits) - 1;
    let epsilon = !gamma_rate & mask;

    gamma_rate as u32 * epsilon as u32
}

fn sieve(mut input: Vec<u16>, num_bits: u8, most_common: bool) -> u16 {
    // we need to work from the most significant bit
    for bit in (0..num_bits).rev() {
        if input.len() == 1 {
            return input[0];
        }

        let mut target_bit = most_common_bit(&input, bit);

        // least common is just reverse of most common
        if !most_common {
            target_bit = !target_bit & 1;
        }

        input.retain(|x| (x >> bit & 1) as u8 == target_bit)
    }

    if input.len() > 1 {
        panic!("we run out of numbers to sift through");
    } else {
        input[0]
    }
}

fn part2(input: &[String]) -> u32 {
    let num_bits = input[0].len() as u8;

    let input: Vec<_> = input
        .iter()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();

    let o2 = sieve(input.clone(), num_bits, true) as u32;
    let co2 = sieve(input, num_bits, false) as u32;

    o2 * co2
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_input_lines, part1, part2)
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

    #[test]
    fn part2_sample_input() {
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

        let expected = 230;

        assert_eq!(expected, part2(&input))
    }
}
