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

struct Stack<T> {
    inner: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            inner: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Clone> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Stack {
            inner: self.inner.clone(),
            size: self.size,
        }
    }
}

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
            n => panic!("invalid bracket type found - {n}"),
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

    fn error_score(&self) -> usize {
        self.typ.error_score()
    }

    fn completion_score(&self) -> usize {
        self.typ.completion_score()
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
    fn error_score(&self) -> usize {
        match self {
            BracketType::Parentheses => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }

    fn completion_score(&self) -> usize {
        match self {
            BracketType::Parentheses => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
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
    let mut stack = Stack::new();

    for bracket in line.chars().map(Bracket::from) {
        if bracket.is_opening() {
            stack.push(bracket)
        } else {
            let popped = match stack.pop() {
                None => return Err(LineError::Corrupted(bracket)),
                Some(bracket) => bracket,
            };
            if popped.inverse() != bracket {
                return Err(LineError::Corrupted(bracket));
            }
        }
    }

    if !stack.is_empty() {
        Err(LineError::Incomplete)
    } else {
        Ok(())
    }
}

fn complete_line(incomplete_line: &str) -> Vec<Bracket> {
    let mut stack = Stack::new();

    // first, fill up the stack with available characters
    for bracket in incomplete_line.chars().map(Bracket::from) {
        if bracket.is_opening() {
            stack.push(bracket)
        } else {
            stack.pop();
        }
    }

    let mut completion_brackets = Vec::new();

    while let Some(popped) = stack.pop() {
        completion_brackets.push(popped.inverse())
    }

    completion_brackets
}

fn calculate_completion_score(completion_brackets: Vec<Bracket>) -> usize {
    let mut score = 0;

    for bracket in completion_brackets {
        score *= 5;
        score += bracket.completion_score()
    }

    score
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| match validate_line(line) {
            Err(LineError::Corrupted(bracket)) => bracket.error_score(),
            _ => 0,
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut scores = input
        .iter()
        .filter(|line| match validate_line(line) {
            Err(err) => err.is_incomplete(),
            _ => false,
        })
        .map(|incomplete_line| calculate_completion_score(complete_line(incomplete_line)))
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[(scores.len() / 2)]
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

    #[test]
    fn part2_sample_input() {
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

        let expected = 288957;

        assert_eq!(expected, part2(&input))
    }
}
