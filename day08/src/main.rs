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
use utils::execute;
use utils::input_read::read_input_lines;

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
            digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
        })
        .count()
}

fn contains_digit(checked: &str, against: &str) -> bool {
    for char in against.chars() {
        if !checked.contains(char) {
            return false;
        }
    }
    true
}

// basically just sort it
fn normalise_digit(raw: &str) -> String {
    let mut chars = raw.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn determine_substitutions(signal: &[String]) -> HashMap<String, usize> {
    let mut identified: [Option<String>; 10] = Default::default();
    let mut substitutions = HashMap::new();

    let mut normalised_signal = signal
        .iter()
        .map(|raw| normalise_digit(raw))
        .collect::<HashSet<_>>();

    // identify 1, 7, 4, 8
    for digit in normalised_signal.iter() {
        if digit.len() == 2 {
            identified[1] = Some(digit.clone());
            substitutions.insert(digit.clone(), 1);
        } else if digit.len() == 3 {
            identified[7] = Some(digit.clone());
            substitutions.insert(digit.clone(), 7);
        } else if digit.len() == 4 {
            identified[4] = Some(digit.clone());
            substitutions.insert(digit.clone(), 4);
        } else if digit.len() == 7 {
            identified[8] = Some(digit.clone());
            substitutions.insert(digit.clone(), 8);
        }
    }

    normalised_signal.remove(identified[1].as_ref().unwrap());
    normalised_signal.remove(identified[7].as_ref().unwrap());
    normalised_signal.remove(identified[4].as_ref().unwrap());
    normalised_signal.remove(identified[8].as_ref().unwrap());

    // identify 3, 9, 6, 0
    for digit in normalised_signal.iter() {
        if digit.len() == 5 {
            if contains_digit(digit, identified[1].as_ref().unwrap()) {
                identified[3] = Some(digit.clone());
                substitutions.insert(digit.clone(), 3);
            }
        } else if digit.len() == 6 {
            if contains_digit(digit, identified[4].as_ref().unwrap()) {
                identified[9] = Some(digit.clone());
                substitutions.insert(digit.clone(), 9);
            } else if !contains_digit(digit, identified[1].as_ref().unwrap()) {
                identified[6] = Some(digit.clone());
                substitutions.insert(digit.clone(), 6);
            } else {
                identified[0] = Some(digit.clone());
                substitutions.insert(digit.clone(), 0);
            }
        } else {
            panic!("invalid length")
        }
    }

    normalised_signal.remove(identified[3].as_ref().unwrap());
    normalised_signal.remove(identified[9].as_ref().unwrap());
    normalised_signal.remove(identified[6].as_ref().unwrap());
    normalised_signal.remove(identified[0].as_ref().unwrap());

    for digit in normalised_signal {
        // only 2 and 5 are left; 5 is subset of 9, while 2 is not.
        if contains_digit(identified[9].as_ref().unwrap(), &digit) {
            identified[5] = Some(digit.clone());
            substitutions.insert(digit.clone(), 5);
        } else {
            identified[2] = Some(digit.clone());
            substitutions.insert(digit.clone(), 2);
        }
    }

    substitutions
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
    input
        .iter()
        .map(|signal_display| {
            let (signal, display) = split_into_pattern_and_display(signal_display);
            let substitutions = determine_substitutions(&signal);
            let display_values = display
                .iter()
                .map(|digit| normalise_digit(digit))
                .map(|normalised| substitutions.get(&normalised).unwrap())
                .collect::<Vec<_>>();
            display_values[0] * 1000
                + display_values[1] * 100
                + display_values[2] * 10
                + display_values[3]
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

        let expected = 61229;

        assert_eq!(expected, part2(&input))
    }
}
