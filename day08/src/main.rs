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
use utils::input_read::read_input_lines;

const ZERO: &str = "abcefg"; // 6
const ONE: &str = "cf"; // *2*
const TWO: &str = "acdeg"; // 5
const THREE: &str = "acdfg"; // 5
const FOUR: &str = "bcdf"; // *4*
const FIVE: &str = "abdfg"; // 5
const SIX: &str = "abdefg"; // 6
const SEVEN: &str = "acf"; // *3*
const EIGHT: &str = "abcdefg"; // *8*
const NINE: &str = "abcdfg"; // 6

struct Segment {
    position: char,
    possible_substitution: Vec<char>,
}

struct RawDigit {
    raw: String,
}

fn unscramble_signal_pattern(pattern: &[String]) -> HashMap<char, char> {
    let mut substitutions = HashMap::new();
    // let mut possibilities =
    println!("tryin to unscralbe {:?}", pattern);

    substitutions
}

fn split_into_pattern_and_display(raw: &str) -> (Vec<String>, Vec<String>) {
    let mut split = raw.split(" | ");
    (
        split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect(),
        split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect(),
    )
}

fn count_uniques(source: &[String]) -> usize {
    source
        .iter()
        .filter(|digit| {
            digit.len() == ONE.len()
                || digit.len() == FOUR.len()
                || digit.len() == SEVEN.len()
                || digit.len() == EIGHT.len()
        })
        .count()
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|signal_display| {
            let (_, display) = split_into_pattern_and_display(signal_display);
            count_uniques(&display)
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
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
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        let expected = 26;

        assert_eq!(expected, part1(&input))
    }
}
