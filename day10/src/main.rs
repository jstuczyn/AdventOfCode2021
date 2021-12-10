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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Bracket {
    typ: BracketType,
    opening: bool,
}

impl From<char> for Bracket {
    fn from(c: char) -> Self {
        match c {
            '(' => Bracket::new(BracketType::Parentheses, true),
            ')' => Bracket::new(BracketType::Parentheses, false),
            '[' => Bracket::new(BracketType::Square, true),
            ']' => Bracket::new(BracketType::Square, false),
            '{' => Bracket::new(BracketType::Curly, true),
            '}' => Bracket::new(BracketType::Curly, false),
            '<' => Bracket::new(BracketType::Angle, true),
            '>' => Bracket::new(BracketType::Angle, false),
            n => panic!("invalid bracket type found - {}", n),
        }
    }
}

impl Bracket {
    fn new(typ: BracketType, opening: bool) -> Self {
        Bracket { typ, opening }
    }

    fn is_opening(&self) -> bool {
        self.opening
    }

    fn inverse(&self) -> Bracket {
        Bracket {
            typ: self.typ,
            opening: !self.opening,
        }
    }

    fn score(&self) -> usize {
        self.typ.score()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum BracketType {
    Square,
    Curly,
    Angle,
    Parentheses,
}

impl BracketType {
    fn score(&self) -> usize {
        match self {
            BracketType::Parentheses => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
}

#[derive(Debug)]
enum LineError {
    Incomplete,
    Corrupted(Bracket),
}

impl LineError {
    fn is_incomplete(&self) -> bool {
        matches!(self, LineError::Incomplete)
    }
}

fn validate_line(line: &str) -> Result<(), LineError> {
    let mut stack = Vec::new();

    for bracket in line.chars().map(Bracket::from) {
        if bracket.is_opening() {
            stack.push(bracket)
        } else {
            let popped = match stack.pop() {
                None => return Err(LineError::Incomplete),
                Some(bracket) => bracket,
            };
            if popped.inverse() != bracket {
                return Err(LineError::Corrupted(bracket));
            }
        }
    }

    Ok(())
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| match validate_line(line) {
            Err(LineError::Corrupted(bracket)) => bracket.score(),
            _ => 0,
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
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];

        let expected = 26397;

        assert_eq!(expected, part1(&input))
    }
}
