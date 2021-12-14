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
use std::collections::HashMap;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

type Pair = (char, char);

#[derive(Debug)]
struct MalformedRule;

#[derive(Debug, Clone)]
struct Rule {
    pair: Pair,
    insertion: char,
}

impl FromStr for Rule {
    type Err = MalformedRule;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let mut pair_raw = split.next().ok_or(MalformedRule)?.chars();
        let pair = (
            pair_raw.next().ok_or(MalformedRule)?,
            pair_raw.next().ok_or(MalformedRule)?,
        );

        let insertion = split
            .next()
            .ok_or(MalformedRule)?
            .to_owned()
            .chars()
            .next()
            .ok_or(MalformedRule)?;

        Ok(Rule { pair, insertion })
    }
}

impl Rule {
    fn apply(&self) -> (Pair, Pair) {
        ((self.pair.0, self.insertion), (self.insertion, self.pair.1))
    }
}

#[derive(Debug, Clone)]
struct Manual {
    front: char,
    pairs: HashMap<Pair, usize>,
    rules: Vec<Rule>,
}

impl FromStr for Manual {
    type Err = MalformedRule;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .replace("\r\n", "\n") // Windows fix
            .split("\n\n")
            .map(|split| split.to_owned())
            .collect::<Vec<_>>();

        let mut pairs: HashMap<Pair, usize> = HashMap::new();

        let mut front = 'Z';
        for (i, pair) in lines[0].chars().tuple_windows().enumerate() {
            *pairs.entry(pair).or_default() += 1;
            if i == 0 {
                front = pair.0;
            }
        }

        // let points = lines[0].lines().map(|s| s.parse().unwrap()).collect();
        let mut rules = Vec::new();
        for rule in lines[1].lines() {
            rules.push(rule.parse()?)
        }

        Ok(Manual {
            front,
            pairs,
            rules,
        })
    }
}

impl Manual {
    fn step(&mut self) {
        let mut new_pairs = self.pairs.clone();
        for rule in &self.rules {
            if let Some(count) = self.pairs.remove(&rule.pair) {
                let inserted = rule.apply();

                *new_pairs.entry(rule.pair).or_default() -= count;
                *new_pairs.entry(inserted.0).or_default() += count;
                *new_pairs.entry(inserted.1).or_default() += count;
            }
        }

        self.pairs = new_pairs
            .into_iter()
            .filter(|(_, count)| *count != 0)
            .collect();
    }

    fn apply_steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step()
        }
    }

    fn element_count(&self) -> HashMap<char, usize> {
        let mut count = HashMap::new();
        for (pair, occurrences) in self.pairs.iter() {
            *count.entry(pair.1).or_default() += occurrences;
        }
        *count.entry(self.front).or_default() += 1;
        count
    }

    fn max_frequency_difference(&self) -> usize {
        let count = self.element_count();

        count.iter().max_by_key(|(_, &count)| count).unwrap().1
            - count.iter().min_by_key(|(_, &count)| count).unwrap().1
    }
}

fn part1(mut manual: Manual) -> usize {
    manual.apply_steps(10);
    manual.max_frequency_difference()
}

fn part2(mut manual: Manual) -> usize {
    manual.apply_steps(40);
    manual.max_frequency_difference()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_struct("input", read_parsed, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            .to_string();

        let manual = input.parse().unwrap();
        let expected = 1588;

        assert_eq!(expected, part1(manual));
    }

    #[test]
    fn part2_sample_input() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            .to_string();

        let manual = input.parse().unwrap();
        let expected = 2188189693529;

        assert_eq!(expected, part2(manual));
    }
}
