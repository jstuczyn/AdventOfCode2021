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

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use std::fmt::Formatter;
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

fn try_get_substitutions(
    known_substitutions: &mut HashMap<char, Vec<char>>,
    digits: &[String],
) -> (HashMap<char, char>, Vec<char>) {
    let mut matched = HashMap::new();
    let mut remaining = Vec::new();
    for digit in digits {}

    (matched, remaining)
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

fn substitution_round() {}
//
// #[derive(Debug)]
// struct PossibleSubstitutions {
//     inner: HashMap<char, HashSet<char>>,
// }
//
// impl PossibleSubstitutions {
//     fn new() -> Self {
//         let mut inner = HashMap::new();
//         let all_possible = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
//             .into_iter()
//             .collect::<HashSet<_>>();
//         inner.insert('a', all_possible.clone());
//         inner.insert('b', all_possible.clone());
//         inner.insert('c', all_possible.clone());
//         inner.insert('d', all_possible.clone());
//         inner.insert('e', all_possible.clone());
//         inner.insert('f', all_possible.clone());
//         inner.insert('g', all_possible);
//
//         PossibleSubstitutions { inner }
//     }
//
//     fn thin_out_substitutions(&mut self) {
//         let mut subs = self.inner.iter().collect::<Vec<_>>();
//         subs.sort_by_key(|x| x.1.len());
//         // if we have, for example a: {c, d} and b: {c, d}, it means its IMPOSSIBLE for say e to be {c, d}
//
//         println!("{:?}", subs);
//     }
//
//     fn insert_segment_constraint(&mut self, segment: char, possibilities: &str) -> bool {
//         let possible_substitutions = self.inner.get_mut(&segment).unwrap();
//
//         // TODO: check if segment fails to get inserted to all but one - it means we have definite match
//         // (consider inserting 1 and then 7. we must get a match on top of 7)
//
//         let mut present_count = 0;
//
//         let mut constrained = HashSet::new();
//         for possible_substitution in possible_substitutions.iter() {
//             if possibilities.contains(*possible_substitution) {
//                 constrained.insert(*possible_substitution);
//             }
//         }
//
//         let constrained_cpy = constrained.clone();
//         *possible_substitutions = constrained;
//
//         // check if for constraint of length N whether there exist N-1 constraints containing all but one character
//         let mut found = 0;
//         let mut chars = Vec::new();
//         'out: for (seg, subs) in &self.inner {
//             if subs.len() == constrained_cpy.len() - 1 {
//                 for sub in subs {
//                     if !constrained_cpy.contains(sub) {
//                         continue 'out;
//                     }
//                 }
//                 found += 1;
//                 chars.push(*seg);
//             }
//         }
//
//         if found == constrained_cpy.len() {
//             println!("we found something")
//         }
//
//         println!("present count: {}", present_count);
//
//         if present_count == 1 {
//             println!("we have definite match! {} in {}", segment, possibilities)
//         }
//
//         false
//     }
//
//     fn insert_digit_constraint(&mut self, encoded_digit: &str, expected_digit_segments: &str) {
//         for segment in expected_digit_segments.chars() {
//             println!("seg: {}", segment);
//             self.insert_segment_constraint(segment, encoded_digit);
//         }
//     }
//
//     fn insert_digits_constraint(&mut self, encoded_digit: &str, possible_digits: &[&str]) {
//         let joined: String = possible_digits.join("");
//         let foo: String = joined.chars().unique().collect();
//         self.insert_digit_constraint(encoded_digit, &foo);
//     }
// }

#[derive(Debug)]
struct PossibleSubstitutions {
    inner: HashMap<char, HashSet<char>>,
}

impl PossibleSubstitutions {
    fn new() -> Self {
        PossibleSubstitutions {
            inner: HashMap::new(),
        }
    }

    fn insert_segment_possibilities(&mut self, segment: char, posibilities: &str) {}

    fn insert_encoded_digit(&mut self, encoded_digit: &str, expected_digit_segments: &str) {
        for segment in expected_digit_segments.chars() {
            self.insert_segment_possibilities(segment, encoded_digit)
        }
    }
}

// note: EACH line of input seems to contain at least single ONE, SEVEN and FOUR
fn determine_substitutions(signal: &[String]) -> HashMap<char, char> {
    let mut possible_matches = PossibleSubstitutions::new();

    // helper sorting, we could definitely go without it, but, egh...
    let mut signal = signal.to_vec();
    signal.sort_by_key(|x| x.len());

    loop {
        for digit in &signal {
            if digit.len() == ONE.len() {
                println!("ONE");
                possible_matches.insert_digit_constraint(digit, ONE)
            }
            if digit.len() == FOUR.len() {
                println!("FOUR");
                possible_matches.insert_digit_constraint(digit, FOUR)
            }
            if digit.len() == SEVEN.len() {
                println!("SEVEN");
                possible_matches.insert_digit_constraint(digit, SEVEN)
            }

            // // also covers 3 and 5
            // if digit.len() == TWO.len() {
            //     possible_matches.insert_digits_constraint(digit, &[TWO, THREE, FIVE])
            // }
            //
            // // also covers 6 and 9
            // if digit.len() == ZERO.len() {
            //     possible_matches.insert_digits_constraint(digit, &[ZERO, SIX, NINE])
            // }

            if digit.len() == EIGHT.len() {
                // eight is useless
            }
        }
        possible_matches.thin_out_substitutions();

        println!("first round");
        println!("{:#?}", possible_matches);

        panic!("done");
    }

    HashMap::new()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|signal_display| {
            let (signal, display) = split_into_pattern_and_display(signal_display);
            let all_digits = signal
                .into_iter()
                .chain(display.iter().cloned())
                .collect::<Vec<_>>();

            let subs = determine_substitutions(&all_digits);
            count_uniques(&display)
        })
        .sum()
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

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            // "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            // "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            // "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            // "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            // "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            // "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            // "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            // "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            // "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        let expected = 61229;

        assert_eq!(expected, part2(&input))
    }
}
