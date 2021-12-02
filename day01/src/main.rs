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
use std::time::Instant;
use utils::input_read;

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
    let input = input_read::read_line_input("input").expect("failed to read input file");
    let start = Instant::now();

    let part1_result = part1(&input);
    let p1_end = Instant::now();
    let p1_time_taken = p1_end - start;

    println!(
        "Part 1 result is {}. It took {:?} to compute",
        part1_result, p1_time_taken
    );

    let part2_result = part2(&input);
    let p2_time_taken = Instant::now() - p1_end;

    println!(
        "Part 2 result is {}. It took {:?} to compute",
        part2_result, p2_time_taken
    );
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
