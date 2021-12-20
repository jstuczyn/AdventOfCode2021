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
use std::cmp::max;
use std::ops::Add;
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Number {
    Regular(u32),
    Pair,
}

impl Number {
    fn must_get_regular(&self) -> u32 {
        match self {
            Number::Regular(val) => *val,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
struct NumberTree {
    heights: Vec<Vec<Option<Number>>>,
}

impl NumberTree {
    fn ensure_height(&mut self, height: usize) {
        if self.heights.get_mut(height).is_none() {
            let height_size = 2usize.pow(height as u32);
            let mut height_data = Vec::with_capacity(height_size);
            height_data.resize_with(height_size, || None);

            self.heights.insert(height, height_data);
        }
    }

    fn insert_pair_node(&mut self, height: usize, branch: usize) {
        self.ensure_height(height);
        debug_assert!(self.heights[height][branch].is_none());
        self.heights[height][branch] = Some(Number::Pair)
    }

    fn insert_num_node(&mut self, height: usize, branch: usize, val: u32) {
        self.ensure_height(height);
        debug_assert!(self.heights[height][branch].is_none());
        self.heights[height][branch] = Some(Number::Regular(val))
    }

    fn explode_pair(&mut self, height: usize, branch: usize) {
        debug_assert_eq!(self.heights[height][branch], Some(Number::Pair));
        debug_assert!(matches!(
            self.heights[height + 1][branch * 2],
            Some(Number::Regular(_))
        ));
        debug_assert!(matches!(
            self.heights[height + 1][branch * 2 + 1],
            Some(Number::Regular(_))
        ));
        self.heights[height][branch] = Some(Number::Regular(0));

        let left_val = self.heights[height + 1][branch * 2]
            .take()
            .unwrap()
            .must_get_regular();
        let right_val = self.heights[height + 1][branch * 2 + 1]
            .take()
            .unwrap()
            .must_get_regular();

        self.add_left_of(height, branch, left_val);
        self.add_right_of(height, branch, right_val);

        // cleanup
        if self.heights[5].iter().all(|val| val.is_none()) {
            self.heights.remove(5);
        }
    }

    fn split_value(&mut self, height: usize, branch: usize) {
        debug_assert!(matches!(
            self.heights[height][branch],
            Some(Number::Regular(_))
        ));
        let val = self.heights[height][branch]
            .as_ref()
            .unwrap()
            .must_get_regular();
        debug_assert!(val >= 10);

        let x = val / 2;
        let y = if val % 2 == 0 { x } else { x + 1 };

        self.heights[height][branch] = Some(Number::Pair);
        self.insert_num_node(height + 1, branch * 2, x);
        self.insert_num_node(height + 1, branch * 2 + 1, y);
    }

    fn _magnitude(&self, height: usize, branch: usize) -> u32 {
        match self.heights[height][branch] {
            Some(Number::Regular(val)) => val,
            Some(Number::Pair) => {
                3 * self._magnitude(height + 1, branch * 2)
                    + 2 * self._magnitude(height + 1, branch * 2 + 1)
            }
            None => unreachable!(),
        }
    }

    fn magnitude(&self) -> u32 {
        self._magnitude(0, 0)
    }

    fn add_left_of(&mut self, this_height: usize, this_branch: usize, val: u32) {
        let in_order = self.in_order_values();
        if let Some(this_id) = in_order
            .iter()
            .position(|n| n == &((this_height, this_branch), 0))
        {
            if this_id > 0 {
                let ((height, branch), current_val) = in_order[this_id - 1];
                self.heights[height][branch] = Some(Number::Regular(current_val + val))
            }
        }
    }

    fn add_right_of(&mut self, this_height: usize, this_branch: usize, val: u32) {
        let in_order = self.in_order_values();
        if let Some(this_id) = in_order
            .iter()
            .position(|n| n == &((this_height, this_branch), 0))
        {
            if this_id < in_order.len() - 1 {
                let ((height, branch), current_val) = in_order[this_id + 1];
                self.heights[height][branch] = Some(Number::Regular(current_val + val))
            }
        }
    }

    fn explode(&mut self) -> bool {
        let mut to_explode = None;
        // values whose parents have to explode will only ever exist on height 5
        match self.heights.get_mut(5) {
            None => return false,
            Some(vals) => {
                for (branch, val) in vals.iter().enumerate() {
                    if val.is_some() {
                        to_explode = Some(branch);
                        break;
                    }
                }
            }
        }

        if let Some(exploding_branch) = to_explode {
            // we explode the parent
            self.explode_pair(4, exploding_branch / 2);
            true
        } else {
            false
        }
    }

    fn in_order_traversal(&self, node: (usize, usize)) -> Vec<((usize, usize), u32)> {
        match &self.heights[node.0][node.1] {
            Some(Number::Regular(val)) => vec![((node.0, node.1), *val)],
            Some(Number::Pair) => {
                let left = self.in_order_traversal((node.0 + 1, node.1 * 2));
                let mut right = self.in_order_traversal((node.0 + 1, node.1 * 2 + 1));
                let mut res = left;
                res.append(&mut right);
                res
            }
            None => vec![],
        }
    }

    fn in_order_values(&self) -> Vec<((usize, usize), u32)> {
        self.in_order_traversal((0, 0))
    }

    fn split(&mut self) -> bool {
        let in_order = self.in_order_values();
        for ((height, branch), val) in in_order {
            if val >= 10 {
                self.split_value(height, branch);
                return true;
            }
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            } else if !self.split() {
                break;
            }
        }
    }
}

impl Number {
    fn parse_into_tree(
        chars: &[char],
        tree: &mut NumberTree,
        height: usize,
        branch: usize,
    ) -> usize {
        // each pair starts with `[`, so we can ignore first character
        let mut used_chars = 1;
        if chars[1] == '[' {
            tree.insert_pair_node(height + 1, branch * 2);
            let used = Self::parse_into_tree(&chars[1..], tree, height + 1, branch * 2);
            used_chars += used;
        } else {
            let val = chars[1].to_digit(10).unwrap();
            tree.insert_num_node(height + 1, branch * 2, val);
            used_chars += 1;
        };

        // next we have to have a comma
        assert_eq!(chars[used_chars], ',');
        used_chars += 1;

        if chars[used_chars] == '[' {
            tree.insert_pair_node(height + 1, branch * 2 + 1);
            let used =
                Self::parse_into_tree(&chars[used_chars..], tree, height + 1, branch * 2 + 1);
            used_chars += used;
        } else {
            let val = chars[used_chars].to_digit(10).unwrap();
            tree.insert_num_node(height + 1, branch * 2 + 1, val);
            used_chars += 1;
        };

        // next we have to have a closing bracket
        assert_eq!(chars[used_chars], ']');
        used_chars += 1;

        used_chars
    }
}

impl<'a> Add<&'a NumberTree> for NumberTree {
    type Output = NumberTree;

    fn add(self, rhs: &'a NumberTree) -> Self::Output {
        let mut res = self.clone();
        let final_height = max(self.heights.len(), rhs.heights.len());
        for height in 1..final_height {
            res.ensure_height(height)
        }

        res.heights.insert(0, vec![Some(Number::Pair)]);

        for (height, height_data) in rhs.heights.iter().enumerate() {
            for val in height_data.iter() {
                res.heights[height + 1].push(val.clone())
            }
        }
        for height in 0..res.heights.len() {
            let height_size = 2usize.pow(height as u32);
            res.heights[height].resize_with(height_size, || None);
        }

        res.reduce();
        res
    }
}

impl FromStr for NumberTree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = NumberTree {
            heights: Vec::new(),
        };
        // we assume that the tree consists of a single pair at the root
        tree.heights.push(vec![Some(Number::Pair)]);

        Number::parse_into_tree(&s.chars().collect::<Vec<_>>(), &mut tree, 0, 0);
        Ok(tree)
    }
}

fn part1(numbers: &[NumberTree]) -> u32 {
    let mut acc = numbers[0].clone();
    for num in numbers.iter().skip(1) {
        acc = acc + num;
    }
    acc.magnitude()
}

fn part2(numbers: &[NumberTree]) -> u32 {
    // no point in using short numbers, they won't produce high magnitudes
    numbers
        .iter()
        .filter(|num| num.heights.len() >= 5)
        .permutations(2)
        .map(|nums| {
            max(
                (nums[0].clone() + &nums[1].clone()).magnitude(),
                (nums[1].clone() + &nums[0].clone()).magnitude(),
            )
        })
        .max()
        .unwrap()
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_parsing() {
        let num: NumberTree = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        let expected = NumberTree {
            heights: vec![
                vec![Some(Number::Pair)],
                vec![Some(Number::Pair), Some(Number::Pair)],
                vec![
                    Some(Number::Pair),
                    Some(Number::Pair),
                    Some(Number::Regular(8)),
                    Some(Number::Regular(1)),
                ],
                vec![
                    Some(Number::Pair),
                    Some(Number::Regular(4)),
                    Some(Number::Pair),
                    Some(Number::Pair),
                    None,
                    None,
                    None,
                    None,
                ],
                vec![
                    Some(Number::Regular(0)),
                    Some(Number::Regular(7)),
                    None,
                    None,
                    Some(Number::Regular(7)),
                    Some(Number::Regular(8)),
                    Some(Number::Regular(6)),
                    Some(Number::Regular(0)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ],
        };
        assert_eq!(expected, num);
    }

    #[test]
    fn explosion() {
        let mut before: NumberTree = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        assert!(before.explode());
        let after: NumberTree = "[[[[0,9],2],3],4]".parse().unwrap();
        assert_eq!(after, before);

        let mut before: NumberTree = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        assert!(before.explode());
        let after: NumberTree = "[7,[6,[5,[7,0]]]]".parse().unwrap();
        assert_eq!(after, before);

        let mut before: NumberTree = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        assert!(before.explode());
        let after: NumberTree = "[[6,[5,[7,0]]],3]".parse().unwrap();
        assert_eq!(after, before);

        let mut before: NumberTree = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        assert!(before.explode());
        let after: NumberTree = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        assert_eq!(after, before);

        let mut before: NumberTree = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        assert!(before.explode());
        let after: NumberTree = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();
        assert_eq!(after, before);
    }

    #[test]
    fn magnitude() {
        let tree: NumberTree = "[[1,2],[[3,4],5]]".parse().unwrap();
        let expected = 143;
        assert_eq!(tree.magnitude(), expected);

        let tree: NumberTree = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        let expected = 1384;
        assert_eq!(tree.magnitude(), expected);

        let tree: NumberTree = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        let expected = 445;
        assert_eq!(tree.magnitude(), expected);

        let tree: NumberTree = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        let expected = 791;
        assert_eq!(tree.magnitude(), expected);

        let tree: NumberTree = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        let expected = 1137;
        assert_eq!(tree.magnitude(), expected);

        let tree: NumberTree = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        let expected = 3488;
        assert_eq!(tree.magnitude(), expected);
    }

    #[test]
    fn sample_addition() {
        let t1: NumberTree = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let t2: NumberTree = "[1,1]".parse().unwrap();

        let expected: NumberTree = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(expected, t1 + &t2)
    }

    #[test]
    fn sample_sum() {
        let nums: Vec<NumberTree> = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap(),
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap(),
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".parse().unwrap(),
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
                .parse()
                .unwrap(),
            "[7,[5,[[3,8],[1,4]]]]".parse().unwrap(),
            "[[2,[2,2]],[8,[8,1]]]".parse().unwrap(),
            "[2,9]".parse().unwrap(),
            "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse().unwrap(),
            "[[[5,[7,4]],7],1]".parse().unwrap(),
            "[[[[4,2],2],6],[8,7]]".parse().unwrap(),
        ];

        let s1: NumberTree = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            .parse()
            .unwrap();

        let s2: NumberTree = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
            .parse()
            .unwrap();

        let s3: NumberTree = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
            .parse()
            .unwrap();

        let s4: NumberTree = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
            .parse()
            .unwrap();

        let s5: NumberTree = "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
            .parse()
            .unwrap();

        let s6: NumberTree = "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
            .parse()
            .unwrap();

        let s7: NumberTree = "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
            .parse()
            .unwrap();

        let s8: NumberTree = "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
            .parse()
            .unwrap();

        let s9: NumberTree = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();

        let mut running_total = nums[0].clone() + &nums[1];
        assert_eq!(running_total, s1);

        running_total = running_total + &nums[2];
        assert_eq!(running_total, s2);

        running_total = running_total + &nums[3];
        assert_eq!(running_total, s3);

        running_total = running_total + &nums[4];
        assert_eq!(running_total, s4);

        running_total = running_total + &nums[5];
        assert_eq!(running_total, s5);

        running_total = running_total + &nums[6];
        assert_eq!(running_total, s6);

        running_total = running_total + &nums[7];
        assert_eq!(running_total, s7);

        running_total = running_total + &nums[8];
        assert_eq!(running_total, s8);

        running_total = running_total + &nums[9];
        assert_eq!(running_total, s9);
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse()
                .unwrap(),
            "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse().unwrap(),
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse().unwrap(),
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".parse().unwrap(),
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".parse().unwrap(),
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".parse().unwrap(),
            "[[[[5,4],[7,7]],8],[[8,3],8]]".parse().unwrap(),
            "[[9,3],[[9,9],[6,[4,9]]]]".parse().unwrap(),
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".parse().unwrap(),
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".parse().unwrap(),
        ];

        let expected = 4140;
        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse()
                .unwrap(),
            "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse().unwrap(),
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse().unwrap(),
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".parse().unwrap(),
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".parse().unwrap(),
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".parse().unwrap(),
            "[[[[5,4],[7,7]],8],[[8,3],8]]".parse().unwrap(),
            "[[9,3],[[9,9],[6,[4,9]]]]".parse().unwrap(),
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".parse().unwrap(),
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".parse().unwrap(),
        ];

        let expected = 3993;
        assert_eq!(expected, part2(&input))
    }
}
